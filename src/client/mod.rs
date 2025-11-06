//! The client to the obs-websocket API and main entry point.

#[cfg(feature = "events")]
use std::sync::Weak;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, Stream, StreamExt},
};
use semver::{Comparator, Op, Prerelease};
use serde::de::DeserializeOwned;
#[cfg(feature = "events")]
use tokio::sync::broadcast;
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{self, Message, protocol::CloseFrame},
};
use tracing::{debug, error, info, trace, warn};

use self::connection::{ReceiverList, ReidentifyReceiverList};
pub use self::{
    config::Config,
    connection::{HandshakeError, IntoTextError, ReceiveError},
    filters::Filters,
    general::General,
    hotkeys::Hotkeys,
    inputs::Inputs,
    media_inputs::MediaInputs,
    outputs::Outputs,
    profiles::Profiles,
    recording::Recording,
    replay_buffer::ReplayBuffer,
    scene_collections::SceneCollections,
    scene_items::SceneItems,
    scenes::Scenes,
    sources::Sources,
    streaming::Streaming,
    transitions::Transitions,
    ui::Ui,
    virtual_cam::VirtualCam,
};
#[cfg(feature = "events")]
use crate::events::Event;
use crate::{
    error::{Error, Result},
    requests::{ClientRequest, EventSubscription, Reidentify, Request, RequestType},
    responses::ServerMessage,
};

mod config;
mod connection;
mod filters;
mod general;
mod hotkeys;
mod inputs;
mod media_inputs;
mod outputs;
mod profiles;
mod recording;
mod replay_buffer;
mod scene_collections;
mod scene_items;
mod scenes;
mod sources;
mod streaming;
mod transitions;
mod ui;
mod virtual_cam;

#[derive(Debug, thiserror::Error)]
enum InnerError {
    #[error("websocket message not convertible to text")]
    IntoText(#[source] tokio_tungstenite::tungstenite::Error),
    #[error("failed deserializing message")]
    DeserializeMessage(#[source] serde_json::Error),
    #[error("the request ID `{0}` is not an integer")]
    InvalidRequestId(#[source] std::num::ParseIntError, String),
    #[error("received unexpected server message: {0:?}")]
    UnexpectedMessage(ServerMessage),
}

/// The client is the main entry point to access the obs-websocket API. It allows to call various
/// functions to remote control an OBS instance as well as to listen to events caused by the user
/// by interacting with OBS.
pub struct Client {
    /// The writer handle to the web-socket stream.
    write: Mutex<MessageWriter>,
    /// Global counter for requests that help to find out what response belongs to what previously
    /// sent request.
    id_counter: AtomicU64,
    /// A list of currently waiting requests to get a response back. The key is the string version
    /// of a request ID and the value is a oneshot sender that allows to send the response back to
    /// the other end that waits for the response.
    receivers: Arc<ReceiverList>,
    /// A list of awaiting [`Self::reidentify`] requests, waiting for confirmation. As
    /// these requests don't carry any kind of ID, they're handled sequentially and must be tracked
    /// separate from normal requests.
    reidentify_receivers: Arc<ReidentifyReceiverList>,
    /// Broadcast sender that distributes received events to all current listeners. Events are
    /// dropped if nobody listens.
    #[cfg(feature = "events")]
    event_sender: Weak<broadcast::Sender<Event>>,
    /// Handle to the background task that receives messages and distributes them to waiting
    /// receivers and event listeners. It allows to shut down all the machinery once the client is
    /// no longer needed.
    handle: Option<JoinHandle<()>>,
    dangerous: DangerousConnectConfig,
}

/// Shorthand for the writer side of a web-socket stream that has been split into reader and writer.
type MessageWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Default broadcast capacity used when not overwritten by the user.
pub const DEFAULT_BROADCAST_CAPACITY: usize = 100;
/// Default connect timeout duration used when not overwritten by the user.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Configuration for connecting to a obs-websocket instance.
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct ConnectConfig<H, P>
where
    H: AsRef<str>,
    P: AsRef<str>,
{
    #[cfg_attr(feature = "builder", builder(start_fn))]
    /// The host name, usually `localhost` unless the OBS instance is on a remote machine.
    pub host: H,
    /// Port to connect to.
    #[cfg_attr(feature = "builder", builder(start_fn))]
    pub port: u16,
    /// Dangerous configuration options that are not given any support for.
    #[cfg_attr(feature = "builder", builder(field))]
    pub dangerous: Option<DangerousConnectConfig>,
    /// The host name, usually `localhost` unless the OBS instance is on a remote machine.
    /// Optional password to authenticate against `obs-websocket`.
    pub password: Option<P>,
    /// Optional list of event subscriptions, controlling what events to receive. By default all
    /// events are listened to, with the exception of high volume events.
    pub event_subscriptions: Option<EventSubscription>,
    /// Whether to use TLS when connecting. Only useful when OBS runs on a remote machine.
    #[cfg(feature = "tls")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tls: bool,
    /// Capacity of the broadcast channel for events. The default is [`DEFAULT_BROADCAST_CAPACITY`]
    /// which should suffice.
    ///
    /// If the consumption of events takes a long time and the broadcast channel fills up faster
    /// than events are consumed, it will start dropping old messages from the queue and these will
    /// not be send to listeners anymore.
    #[cfg_attr(not(feature = "events"), allow(dead_code))]
    #[cfg_attr(feature = "builder", builder(default = DEFAULT_BROADCAST_CAPACITY))]
    pub broadcast_capacity: usize,
    /// Maximum wait time to establish a connection with the OBS instance. The default is
    /// [`DEFAULT_CONNECT_TIMEOUT`].
    ///
    /// If this limit is exceeded, the connection ([`Client::connect_with_config`]) call will
    /// cancel the attempt and return an [`Error::Timeout`].
    #[cfg_attr(feature = "builder", builder(default = DEFAULT_CONNECT_TIMEOUT))]
    pub connect_timeout: Duration,
}

#[cfg(feature = "builder")]
impl<H, P, S> ConnectConfigBuilder<H, P, S>
where
    H: AsRef<str>,
    P: AsRef<str>,
    S: connect_config_builder::State,
{
    /// Enter into dangerous configuration.
    pub fn dangerous<S2: dangerous_connect_config_builder::State>(
        mut self,
        f: impl FnOnce(DangerousConnectConfigBuilder) -> DangerousConnectConfigBuilder<S2>,
    ) -> Self {
        self.dangerous = Some(f(DangerousConnectConfig::builder()).build());
        self
    }
}

/// Dangerous configuration options that are not given any support for.
#[derive(Default)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct DangerousConnectConfig {
    /// Skip validation of the minimum OBS Studio version.
    #[cfg_attr(feature = "builder", builder(default))]
    pub skip_studio_version_check: bool,
    /// Skip validation of the minimum OBS WebSocket version.
    #[cfg_attr(feature = "builder", builder(default))]
    pub skip_websocket_version_check: bool,
}

const OBS_STUDIO_VERSION: Comparator = Comparator {
    op: Op::GreaterEq,
    major: 30,
    minor: Some(2),
    patch: None,
    pre: Prerelease::EMPTY,
};

const OBS_WEBSOCKET_VERSION: Comparator = Comparator {
    op: Op::Caret,
    major: 5,
    minor: Some(5),
    patch: None,
    pre: Prerelease::EMPTY,
};

const RPC_VERSION: u32 = 1;

impl<H, P> ConnectConfig<H, P>
where
    H: AsRef<str>,
    P: AsRef<str>,
{
    #[cfg(feature = "tls")]
    fn tls(&self) -> bool {
        self.tls
    }

    #[cfg(not(feature = "tls"))]
    #[expect(clippy::unused_self)]
    fn tls(&self) -> bool {
        false
    }
}

impl Client {
    /// Connect to a obs-websocket instance on the given host and port.
    ///
    /// # Errors
    ///
    /// Will return an [`Error::Timeout`] if the connection couldn't be established within **30
    /// seconds**.
    pub async fn connect(
        host: impl AsRef<str>,
        port: u16,
        password: Option<impl AsRef<str>>,
    ) -> Result<Self> {
        Self::connect_with_config(ConnectConfig {
            host,
            port,
            password,
            event_subscriptions: if cfg!(feature = "events") {
                None
            } else {
                Some(EventSubscription::NONE)
            },
            #[cfg(feature = "tls")]
            tls: false,
            broadcast_capacity: DEFAULT_BROADCAST_CAPACITY,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            dangerous: None,
        })
        .await
    }

    /// Connect to a obs-websocket instance with the given configuration.
    pub async fn connect_with_config<H, P>(config: ConnectConfig<H, P>) -> Result<Self>
    where
        H: AsRef<str>,
        P: AsRef<str>,
    {
        if config.dangerous.is_some() {
            warn!(
                "dangerous configuration is being used. Please not that no support is given for \
                 any issues encountered while using these options"
            );
        }

        let (socket, _) = tokio::time::timeout(
            config.connect_timeout,
            tokio_tungstenite::connect_async(format!(
                "{}://{}:{}",
                if config.tls() { "wss" } else { "ws" },
                config.host.as_ref(),
                config.port
            )),
        )
        .await
        .map_err(|_| Error::Timeout)?
        .map_err(|e| crate::error::ConnectError(e.into()))?;

        let (mut write, mut read) = socket.split();

        let receivers = Arc::new(ReceiverList::default());
        let reidentify_receivers = Arc::new(ReidentifyReceiverList::default());

        #[cfg(feature = "events")]
        let (event_sender, _) = broadcast::channel(config.broadcast_capacity);
        #[cfg(feature = "events")]
        let event_sender = Arc::new(event_sender);
        #[cfg(feature = "events")]
        let events_tx = Arc::clone(&event_sender);

        self::connection::handshake(
            &mut write,
            &mut read,
            config.password.as_ref().map(AsRef::as_ref),
            config.event_subscriptions,
        )
        .await?;

        let handle = tokio::spawn(recv_loop(
            read,
            #[cfg(feature = "events")]
            events_tx,
            Arc::clone(&receivers),
            Arc::clone(&reidentify_receivers),
        ));

        let write = Mutex::new(write);
        let id_counter = AtomicU64::new(1);

        let client = Self {
            write,
            id_counter,
            receivers,
            reidentify_receivers,
            #[cfg(feature = "events")]
            event_sender: Arc::downgrade(&event_sender),
            handle: Some(handle),
            dangerous: config.dangerous.unwrap_or_default(),
        };

        client.verify_versions().await?;

        Ok(client)
    }

    async fn verify_versions(&self) -> Result<()> {
        let version = self.general().version().await?;

        if !self.dangerous.skip_studio_version_check
            && !OBS_STUDIO_VERSION.matches(&version.obs_version)
        {
            return Err(Error::ObsStudioVersion(
                version.obs_version,
                OBS_STUDIO_VERSION,
            ));
        }

        if !self.dangerous.skip_websocket_version_check
            && !OBS_WEBSOCKET_VERSION.matches(&version.obs_web_socket_version)
        {
            return Err(Error::ObsWebsocketVersion(
                version.obs_web_socket_version,
                OBS_WEBSOCKET_VERSION,
            ));
        }

        if RPC_VERSION != version.rpc_version {
            return Err(Error::RpcVersion {
                requested: RPC_VERSION,
                negotiated: version.rpc_version,
            });
        }

        Ok(())
    }

    async fn send_message<'a, R, T>(&self, req: R) -> Result<T>
    where
        R: Into<RequestType<'a>>,
        T: DeserializeOwned,
    {
        async fn send(
            id_counter: &AtomicU64,
            receivers: &Arc<ReceiverList>,
            write: &Mutex<MessageWriter>,
            req: RequestType<'_>,
        ) -> Result<serde_json::Value> {
            let id = id_counter.fetch_add(1, Ordering::SeqCst);
            let id_str = id.to_string();
            let req = ClientRequest::Request(Request {
                request_id: &id_str,
                ty: req,
            });
            let json = serde_json::to_string(&req).map_err(crate::error::SerializeMessageError)?;

            let rx = receivers.add(id).await;

            trace!(%json, "sending message");
            let write_result = write
                .lock()
                .await
                .send(Message::text(json))
                .await
                .map_err(|e| crate::error::SendError(e.into()));

            if let Err(e) = write_result {
                receivers.remove(id).await;
                return Err(e.into());
            }

            let (status, resp) = rx.await.map_err(crate::error::ReceiveMessageError)?;
            if !status.result {
                return Err(Error::Api {
                    code: status.code,
                    message: status.comment,
                });
            }

            Ok(resp)
        }

        let resp = send(&self.id_counter, &self.receivers, &self.write, req.into()).await?;
        serde_json::from_value(resp)
            .map_err(crate::error::DeserializeResponseError)
            .map_err(Into::into)
    }

    /// Disconnect from obs-websocket and shut down all machinery.
    ///
    /// This is called automatically when dropping the client but doesn't wait for all background
    /// tasks to complete. Therefore, it is recommended to call this manually once the client is
    /// no longer needed.
    pub fn disconnect(&mut self) -> impl Future + use<> {
        let handle = self.handle.take().inspect(|h| {
            h.abort();
        });

        async {
            if let Some(h) = handle {
                h.await.ok();
            }
        }
    }

    /// Adjust settings of the currently active connection by re-identifying against
    /// `obs-websocket`.
    ///
    /// This currently allows to change the events to listen for, without the need of a full
    /// disconnect and new connection.
    pub async fn reidentify(&self, event_subscriptions: EventSubscription) -> Result<()> {
        let json = serde_json::to_string(&ClientRequest::Reidentify(Reidentify {
            event_subscriptions: Some(event_subscriptions),
        }))
        .map_err(crate::error::SerializeMessageError)?;

        let rx = self.reidentify_receivers.add().await;

        self.write
            .lock()
            .await
            .send(Message::text(json))
            .await
            .map_err(|e| crate::error::SendError(e.into()))?;

        let resp = rx.await.map_err(crate::error::ReceiveMessageError)?;
        debug!(
            rpc_version = %resp.negotiated_rpc_version,
            "re-identified against obs-websocket",
        );

        Ok(())
    }

    /// Get a stream of events. Each call to this function creates a new listener, therefore it's
    /// recommended to keep the stream around and iterate over it.
    ///
    /// **Note**: To be able to iterate over the stream you have to pin it with
    /// [`futures_util::pin_mut`] for example.
    ///
    /// # Errors
    ///
    /// Getting a new stream of events fails with [`Error::Disconnected`] if the client is
    /// disconnected from obs-websocket. That can happen either by manually disconnecting, stopping
    /// obs-websocket or closing OBS.
    #[cfg(feature = "events")]
    pub fn events(&self) -> Result<impl Stream<Item = Event> + use<>> {
        if let Some(sender) = &self.event_sender.upgrade() {
            let mut receiver = sender.subscribe();

            Ok(async_stream::stream! {
                while let Ok(event) = receiver.recv().await {
                    yield event;
                }
            })
        } else {
            Err(crate::error::Error::Disconnected)
        }
    }

    /// Access API functions related to OBS configuration.
    pub fn config(&self) -> Config<'_> {
        Config { client: self }
    }

    /// Access API functions related to filters.
    pub fn filters(&self) -> Filters<'_> {
        Filters { client: self }
    }

    /// Access general API functions.
    pub fn general(&self) -> General<'_> {
        General { client: self }
    }

    /// Access API functions related to hotkeys.
    pub fn hotkeys(&self) -> Hotkeys<'_> {
        Hotkeys { client: self }
    }

    /// Access API functions related to inputs.
    pub fn inputs(&self) -> Inputs<'_> {
        Inputs { client: self }
    }

    /// Access API functions related to media inputs.
    pub fn media_inputs(&self) -> MediaInputs<'_> {
        MediaInputs { client: self }
    }

    /// Access API functions related to outputs.
    pub fn outputs(&self) -> Outputs<'_> {
        Outputs { client: self }
    }

    /// Access API functions related to profiles.
    pub fn profiles(&self) -> Profiles<'_> {
        Profiles { client: self }
    }

    /// Access API functions related to recording.
    pub fn recording(&self) -> Recording<'_> {
        Recording { client: self }
    }

    /// Access API functions related to the replay buffer.
    pub fn replay_buffer(&self) -> ReplayBuffer<'_> {
        ReplayBuffer { client: self }
    }

    /// Access API functions related to scene collections.
    pub fn scene_collections(&self) -> SceneCollections<'_> {
        SceneCollections { client: self }
    }

    /// Access API functions related to scene items.
    pub fn scene_items(&self) -> SceneItems<'_> {
        SceneItems { client: self }
    }

    /// Access API functions related to scenes.
    pub fn scenes(&self) -> Scenes<'_> {
        Scenes { client: self }
    }

    /// Access API functions related to sources.
    pub fn sources(&self) -> Sources<'_> {
        Sources { client: self }
    }

    /// Access API functions related to streaming.
    pub fn streaming(&self) -> Streaming<'_> {
        Streaming { client: self }
    }

    /// Access API functions related to transitions.
    pub fn transitions(&self) -> Transitions<'_> {
        Transitions { client: self }
    }

    /// Access API functions related to the user interface.
    pub fn ui(&self) -> Ui<'_> {
        Ui { client: self }
    }

    /// Access API functions related to the virtual camera.
    pub fn virtual_cam(&self) -> VirtualCam<'_> {
        VirtualCam { client: self }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // We simply drop the future as the background task has been aborted but we have no way here
        // to wait for it to fully shut down (except spinning up a new tokio runtime).
        drop(self.disconnect());
    }
}

/// Run the receiving side of the WebSocket connection.
async fn recv_loop(
    mut read: impl Stream<Item = tungstenite::Result<Message>> + Unpin,
    #[cfg(feature = "events")] events_tx: Arc<broadcast::Sender<Event>>,
    receivers: Arc<ReceiverList>,
    reidentify_receivers: Arc<ReidentifyReceiverList>,
) {
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Close(info) = &msg {
            if let Some(CloseFrame { reason, .. }) = info {
                info!(%reason, "connection closed with reason");
            }

            #[cfg(feature = "events")]
            events_tx.send(Event::ServerStopping).ok();
            continue;
        }

        let res: Result<(), InnerError> = async {
            let text = msg.into_text().map_err(InnerError::IntoText)?;

            let message = serde_json::from_str::<ServerMessage>(&text)
                .map_err(InnerError::DeserializeMessage)?;

            match message {
                ServerMessage::RequestResponse(response) => {
                    trace!(
                        id = %response.id,
                        status = ?response.status,
                        data = %response.data,
                        "got request-response message",
                    );
                    receivers.notify(response).await?;
                }
                #[cfg(feature = "events")]
                ServerMessage::Event(event) => {
                    trace!(?event, "got OBS event");
                    events_tx.send(event).ok();
                }
                #[cfg(not(feature = "events"))]
                ServerMessage::Event => {
                    trace!("got OBS event");
                }
                ServerMessage::Identified(identified) => {
                    trace!(?identified, "got identified message");
                    reidentify_receivers.notify(identified).await;
                }
                _ => {
                    trace!(?message, "got unexpected message");
                    return Err(InnerError::UnexpectedMessage(message));
                }
            }

            Ok(())
        }
        .await;

        if let Err(error) = res {
            error!(?error, "failed handling message");
        }
    }

    #[cfg(feature = "events")]
    events_tx.send(Event::ServerStopped).ok();

    // clear all outstanding receivers to stop them from waiting forever on responses
    // they'll never receive.
    receivers.reset().await;
    reidentify_receivers.reset().await;
}
