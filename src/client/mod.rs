//! The client to the obs-websocket API and main entry point.

#[cfg(feature = "events")]
use std::sync::Weak;
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, Stream, StreamExt},
    Sink,
};
use semver::{Comparator, Op, Prerelease};
use serde::de::DeserializeOwned;
#[cfg(feature = "events")]
use tokio::sync::broadcast;
use tokio::{
    net::TcpStream,
    sync::{oneshot, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, trace};

pub use self::{
    config::Config, general::General, inputs::Inputs, media_inputs::MediaInputs,
    recording::Recording, scene_items::SceneItems, scenes::Scenes, sources::Sources,
    streaming::Streaming,
};
#[cfg(feature = "events")]
use crate::events::Event;
use crate::{
    requests::{ClientRequest, EventSubscription, Identify, Request, RequestType},
    responses::{Hello, Identified, RequestResponse, ServerMessage, Status},
    Error, Result,
};

mod config;
mod general;
mod inputs;
mod media_inputs;
mod recording;
mod scene_items;
mod scenes;
mod sources;
mod streaming;

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
    receivers: Arc<Mutex<ReceiverList>>,
    /// Broadcast sender that distributes received events to all current listeners. Events are
    /// dropped if nobody listens.
    #[cfg(feature = "events")]
    event_sender: Weak<broadcast::Sender<Event>>,
    /// Handle to the background task that receives messages and distributes them to waiting
    /// receivers and event listeners. It allows to shut down all the machinery once the client is
    /// no longer needed.
    handle: Option<JoinHandle<()>>,
}

/// Shorthand for the writer side of a web-socket stream that has been split into reader and writer.
type MessageWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Shorthand for the list of ongoing requests that wait for a response.
type ReceiverList = HashMap<u64, oneshot::Sender<(Status, serde_json::Value)>>;

/// Default broadcast capacity used when not overwritten by the user.
#[cfg(feature = "events")]
const DEFAULT_CAPACITY: usize = 100;

/// Configuration for connecting to a obs-websocket instance.
pub struct ConnectConfig<H, P>
where
    H: AsRef<str>,
    P: AsRef<str>,
{
    /// The host name, usually `localhost` unless the OBS instance is on a remote machine.
    pub host: H,
    /// Port to connect to.
    pub port: u16,
    pub password: Option<P>,
    pub event_subscriptions: Option<EventSubscription>,
    /// Whether to use TLS when connecting. Only useful when OBS runs on a remote machine.
    #[cfg(feature = "tls")]
    pub tls: bool,
    /// Capacity of the broadcast channel for events. The default is `100` which should suffice.
    /// If the consumption of events takes a long time and the broadcast channel fills up faster
    /// than events are consumed, it will start dropping old messages from the queue and these will
    /// not be send to listeners anymore.
    #[cfg_attr(not(feature = "events"), allow(dead_code))]
    pub broadcast_capacity: Option<usize>,
}

const OBS_STUDIO_VERSION: Comparator = Comparator {
    op: Op::GreaterEq,
    major: 27,
    minor: None,
    patch: None,
    pre: Prerelease::EMPTY,
};
const OBS_WEBSOCKET_VERSION: Comparator = Comparator {
    op: Op::Caret,
    major: 5,
    minor: None,
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
    fn tls(&self) -> bool {
        false
    }
}

impl Client {
    /// Connect to a obs-websocket instance on the given host and port.
    pub async fn connect(
        host: impl AsRef<str>,
        port: u16,
        password: Option<impl AsRef<str>>,
    ) -> Result<Self> {
        Self::connect_with_config(ConnectConfig {
            host,
            port,
            password,
            event_subscriptions: None,
            #[cfg(feature = "tls")]
            tls: false,
            broadcast_capacity: None,
        })
        .await
    }

    /// Connect to a obs-websocket instance with the given configuration.
    pub async fn connect_with_config<H, P>(config: ConnectConfig<H, P>) -> Result<Self>
    where
        H: AsRef<str>,
        P: AsRef<str>,
    {
        let (socket, _) = tokio_tungstenite::connect_async(format!(
            "{}://{}:{}",
            if config.tls() { "wss" } else { "ws" },
            config.host.as_ref(),
            config.port
        ))
        .await
        .map_err(Error::Connect)?;

        let (mut write, mut read) = socket.split();
        let receivers = Arc::new(Mutex::new(HashMap::<_, oneshot::Sender<_>>::new()));
        let receivers2 = Arc::clone(&receivers);
        #[cfg(feature = "events")]
        let (event_sender, _) =
            broadcast::channel(config.broadcast_capacity.unwrap_or(DEFAULT_CAPACITY));
        #[cfg(feature = "events")]
        let event_sender = Arc::new(event_sender);
        #[cfg(feature = "events")]
        let events_tx = Arc::clone(&event_sender);

        handshake(
            &mut write,
            &mut read,
            config.password.as_ref().map(AsRef::as_ref),
            config.event_subscriptions,
        )
        .await?;

        let handle = tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                trace!("{}", msg);

                if msg.is_close() {
                    #[cfg(feature = "events")]
                    events_tx.send(Event::ServerStopping).ok();
                    continue;
                }

                let res: Result<(), InnerError> = async {
                    let text = msg.into_text().map_err(InnerError::IntoText)?;

                    let message = serde_json::from_str::<ServerMessage>(&text)
                        .map_err(InnerError::DeserializeMessage)?;

                    match message {
                        ServerMessage::RequestResponse(RequestResponse {
                            request_type: _,
                            request_id,
                            request_status,
                            response_data,
                        }) => {
                            let request_id = request_id
                                .parse()
                                .map_err(|e| InnerError::InvalidRequestId(e, request_id))?;

                            debug!("got message with id {}", request_id);
                            if let Some(tx) = receivers2.lock().await.remove(&request_id) {
                                tx.send((request_status, response_data)).ok();
                            }
                        }
                        #[cfg(feature = "events")]
                        ServerMessage::Event(event) => {
                            events_tx.send(event).ok();
                        }
                        _ => return Err(InnerError::UnexpectedMessage(message)),
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = res {
                    error!("failed handling message: {:?}", e);
                }
            }

            #[cfg(feature = "events")]
            events_tx.send(Event::ServerStopped).ok();

            // clear all outstanding receivers to stop them from waiting forever on responses
            // they'll never receive.
            receivers2.lock().await.clear();
        });

        let write = Mutex::new(write);
        let id_counter = AtomicU64::new(1);

        let client = Self {
            write,
            id_counter,
            receivers,
            #[cfg(feature = "events")]
            event_sender: Arc::downgrade(&event_sender),
            handle: Some(handle),
        };

        client.verify_versions().await?;

        Ok(client)
    }

    async fn verify_versions(&self) -> Result<()> {
        let version = self.general().get_version().await?;

        if !OBS_STUDIO_VERSION.matches(&version.obs_version) {
            return Err(Error::ObsStudioVersion(
                version.obs_version,
                OBS_STUDIO_VERSION,
            ));
        }

        if !OBS_WEBSOCKET_VERSION.matches(&version.obs_web_socket_version) {
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

    async fn send_message<T>(&self, req: RequestType<'_>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        let id_str = id.to_string();
        let req = ClientRequest::Request(Request {
            request_id: &id_str,
            ty: req,
        });
        let json = serde_json::to_string(&req).map_err(Error::SerializeMessage)?;

        let (tx, rx) = oneshot::channel();
        self.receivers.lock().await.insert(id, tx);

        debug!("sending message: {}", json);
        let write_result = self
            .write
            .lock()
            .await
            .send(Message::Text(json))
            .await
            .map_err(Error::Send);

        if let Err(e) = write_result {
            self.receivers.lock().await.remove(&id);
            return Err(e);
        }

        let (status, resp) = rx.await.map_err(Error::ReceiveMessage)?;
        if !status.result {
            return Err(Error::Api {
                code: status.code,
                message: status.comment,
            });
        }

        serde_json::from_value(resp).map_err(Error::DeserializeResponse)
    }

    /// Disconnect from obs-websocket and shut down all machinery.
    ///
    /// This is called automatically when dropping the client but doesn't wait for all background
    /// tasks to complete. Therefore, it is recommended to call this manually once the client is
    /// no longer needed.
    pub fn disconnect(&mut self) -> impl Future {
        let handle = self.handle.take().map(|h| {
            h.abort();
            h
        });

        async {
            if let Some(h) = handle {
                h.await.ok();
            }
        }
    }

    pub async fn reidentify(&self) -> Result<()> {
        todo!("The `Reidentify` command is not yet implemented")
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
    pub fn events(&self) -> Result<impl Stream<Item = Event>> {
        if let Some(sender) = &self.event_sender.upgrade() {
            let mut receiver = sender.subscribe();

            Ok(async_stream::stream! {
                while let Ok(event) = receiver.recv().await {
                    yield event;
                }
            })
        } else {
            Err(crate::Error::Disconnected)
        }
    }

    /// Access API functions related to OBS configuration.
    pub fn config(&self) -> Config<'_> {
        Config { client: self }
    }

    /// Access general API functions.
    pub fn general(&self) -> General<'_> {
        General { client: self }
    }

    /// Access API functions related to inputs.
    pub fn inputs(&self) -> Inputs<'_> {
        Inputs { client: self }
    }

    /// Access API functions related to media inputs.
    pub fn media_inputs(&self) -> MediaInputs<'_> {
        MediaInputs { client: self }
    }

    /// Access API functions related to recording.
    pub fn recording(&self) -> Recording<'_> {
        Recording { client: self }
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
}

impl Drop for Client {
    fn drop(&mut self) {
        // We simply drop the future as the background task has been aborted but we have no way here
        // to wait for it to fully shut down (except spinning up a new tokio runtime).
        drop(self.disconnect());
    }
}

/// Errors that can occur while performing the initial handshake with obs-websocket.
#[derive(Debug, thiserror::Error)]
pub enum HandshakeError {
    /// The connection to obs-websocket was interrupted while trying to read a message.
    #[error("connection to obs-websocket was closed")]
    ConnectionClosed,
    /// Receiving a message did not succeed.
    #[error("failed reading websocket message")]
    Receive(#[source] tokio_tungstenite::tungstenite::Error),
    /// The web-socket message was not convertible to text.
    #[error("websocket message not convertible to text")]
    IntoText(#[source] tokio_tungstenite::tungstenite::Error),
    /// A message from obs-websocket could not be deserialized.
    #[error("failed deserializing message")]
    DeserializeMessage(#[source] serde_json::Error),
    /// A message could not be serialized for sending.
    #[error("failed serializing message")]
    SerializeMessage(#[source] serde_json::Error),
    /// Sending a message to obs-websocket failed.
    #[error("failed to send message to obs-websocket")]
    Send(#[source] tokio_tungstenite::tungstenite::Error),
    /// Didn't receive the initial `Hello` message from obs-websocket after connecting.
    #[error("didn't receive a `Hello` message after connecting")]
    NoHello,
    /// Didn't receive a `Identified` message from obs-websocket after authentication.
    #[error("didn't receive a `Identified` message")]
    NoIdentified,
}

async fn handshake(
    write: &mut (impl Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin),
    read: &mut (impl Stream<Item = tokio_tungstenite::tungstenite::Result<Message>> + Unpin),
    password: Option<&str>,
    event_subscriptions: Option<EventSubscription>,
) -> Result<(), HandshakeError> {
    async fn read_message(
        read: &mut (impl Stream<Item = tokio_tungstenite::tungstenite::Result<Message>> + Unpin),
    ) -> Result<ServerMessage, HandshakeError> {
        let message = read
            .next()
            .await
            .ok_or(HandshakeError::ConnectionClosed)?
            .map_err(HandshakeError::Receive)?
            .into_text()
            .map_err(HandshakeError::IntoText)?;

        serde_json::from_str::<ServerMessage>(&message).map_err(HandshakeError::DeserializeMessage)
    }

    match read_message(read).await? {
        ServerMessage::Hello(Hello {
            obs_web_socket_version: _,
            rpc_version,
            authentication,
        }) => {
            let authentication = authentication.zip(password).map(|(auth, password)| {
                create_auth_response(&auth.challenge, &auth.salt, password)
            });

            let req = serde_json::to_string(&ClientRequest::Identify(Identify {
                rpc_version,
                authentication,
                event_subscriptions,
            }))
            .map_err(HandshakeError::SerializeMessage)?;

            write
                .send(Message::Text(req))
                .await
                .map_err(HandshakeError::Send)?;
        }
        _ => return Err(HandshakeError::NoHello),
    }

    match read_message(read).await? {
        ServerMessage::Identified(Identified {
            negotiated_rpc_version,
        }) => {
            debug!("identified with RPC version {}", negotiated_rpc_version);
        }
        _ => return Err(HandshakeError::NoIdentified),
    }

    Ok(())
}

fn create_auth_response(challenge: &str, salt: &str, password: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());

    let mut auth = String::with_capacity(Sha256::output_size() * 4 / 3 + 4);

    base64::encode_config_buf(hasher.finalize_reset(), base64::STANDARD, &mut auth);

    hasher.update(auth.as_bytes());
    hasher.update(challenge.as_bytes());
    auth.clear();

    base64::encode_config_buf(hasher.finalize(), base64::STANDARD, &mut auth);

    auth
}

pub enum WebSocketCloseCode {
    /// For internal use only to tell the request handler not to perform any close action.
    DontClose = 0,
    /// Unknown reason, should never be used.
    UnknownReason = 4000,
    /// The server was unable to decode the incoming web-socket message.
    MessageDecodeError = 4002,
    /// A data field is required but missing from the payload.
    MissingDataField = 4003,
    /// A data field's value type is invalid.
    InvalidDataFieldType = 4004,
    /// A data field's value is invalid.
    InvalidDataFieldValue = 4005,
    /// The specified `op` was invalid or missing.
    UnknownOpCode = 4006,
    /// The client sent a web-socket message without first sending `Identify` message.
    NotIdentified = 4007,
    /// The client sent an `Identify` message while already identified.
    AlreadyIdentified = 4008,
    /// The authentication attempt (via `Identify`) failed.
    AuthenticationFailed = 4009,
    /// The server detected the usage of an old version of the obs-websocket RPC protocol.
    UnsupportedRpcVersion = 4010,
    /// The web-socket session has been invalidated by the obs-websocket server.
    SessionInvalidated = 4011,
    /// A requested feature is not supported due to hardware/software limitations.
    UnsupportedFeature = 4012,
}
