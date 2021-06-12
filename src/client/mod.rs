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

#[cfg(feature = "events")]
use futures_util::stream::Stream;
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use log::{debug, error, trace};
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

#[cfg(feature = "events")]
use crate::events::{Event, EventType};
use crate::{
    requests::{Request, RequestType},
    responses::{AuthRequired, Response},
    Error, Result,
};

pub use self::{
    general::General, media_control::MediaControl, outputs::Outputs, profiles::Profiles,
    recording::Recording, replay_buffer::ReplayBuffer, scene_collections::SceneCollections,
    scene_items::SceneItems, scenes::Scenes, sources::Sources, streaming::Streaming,
    studio_mode::StudioMode, transitions::Transitions, virtual_cam::VirtualCam,
};

mod general;
mod media_control;
mod outputs;
mod profiles;
mod recording;
mod replay_buffer;
mod scene_collections;
mod scene_items;
mod scenes;
mod sources;
mod streaming;
mod studio_mode;
mod transitions;
mod virtual_cam;

#[derive(Debug, thiserror::Error)]
enum InnerError {
    #[error("websocket message not convertible to text")]
    IntoText(#[source] tokio_tungstenite::tungstenite::Error),
    #[error("failed deserializing message")]
    DeserializeMessage(#[source] serde_json::Error),
    #[error("failed deserializing event")]
    #[cfg_attr(not(feature = "events"), allow(dead_code))]
    DeserializeEvent(#[source] serde_json::Error),
}

/// The client is the main entry point to access the obs-websocket API. It allows to call various
/// functions to remote control an OBS instance as well as to listen to events caused by the user
/// by interacting with OBS.
pub struct Client {
    /// The writer handle to the websocket stream.
    write: Mutex<MessageWriter>,
    /// Global counter for requests that help to find out what response belongs to what previously
    /// sent request.
    id_counter: AtomicU64,
    /// A list of currently waiting requests to get a response back. The key is the string version
    /// of a request ID and the value is a oneshot sender that allows to send the response back to
    /// the other end that waits for the response.
    receivers: Arc<Mutex<HashMap<u64, oneshot::Sender<serde_json::Value>>>>,
    /// Broadcast sender that distributes received events to all current listeners. Events are
    /// dropped if nobody listens.
    #[cfg(feature = "events")]
    event_sender: Weak<broadcast::Sender<Event>>,
    /// Handle to the background task that receives messages and distributes them to waiting
    /// receivers and event listeners. It allows to shut down all the machinery once the client is
    /// no longer needed.
    handle: Option<JoinHandle<()>>,
}

/// Shorthand for the writer side of a websocket stream that has been split into reader and writer.
type MessageWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Default broadcast capacity used when not overwritten by the user.
#[cfg(feature = "events")]
const DEFAULT_CAPACITY: usize = 100;

/// Configuration for connecting to a obs-websocket instance.
pub struct ConnectConfig<H>
where
    H: AsRef<str>,
{
    /// The hostname, usually `localhost` unless the OBS instance is on a remote machine.
    pub host: H,
    /// Port to connect to.
    pub port: u16,
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
    op: Op::Tilde,
    major: 4,
    minor: Some(9),
    patch: Some(1),
    pre: Prerelease::EMPTY,
};

impl<H> ConnectConfig<H>
where
    H: AsRef<str>,
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
    pub async fn connect(host: impl AsRef<str>, port: u16) -> Result<Self> {
        Self::connect_with_config(ConnectConfig {
            host,
            port,
            #[cfg(feature = "tls")]
            tls: false,
            broadcast_capacity: None,
        })
        .await
    }

    /// Connect to a obs-websocket instance with the given configuration.
    pub async fn connect_with_config<H: AsRef<str>>(config: ConnectConfig<H>) -> Result<Self> {
        let (socket, _) = tokio_tungstenite::connect_async(format!(
            "{}://{}:{}",
            if config.tls() { "wss" } else { "ws" },
            config.host.as_ref(),
            config.port
        ))
        .await
        .map_err(Error::Connect)?;

        let (write, mut read) = socket.split();
        let receivers = Arc::new(Mutex::new(HashMap::<_, oneshot::Sender<_>>::new()));
        let receivers2 = Arc::clone(&receivers);
        #[cfg(feature = "events")]
        let (event_sender, _) =
            broadcast::channel(config.broadcast_capacity.unwrap_or(DEFAULT_CAPACITY));
        #[cfg(feature = "events")]
        let event_sender = Arc::new(event_sender);
        #[cfg(feature = "events")]
        let events_tx = Arc::clone(&event_sender);

        let handle = tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                trace!("{}", msg);
                let res: Result<(), InnerError> = async {
                    let text = msg.into_text().map_err(InnerError::IntoText)?;
                    let text = if text == "Server stopping" {
                        debug!("Websocket server is stopping");
                        r#"{"update-type": "ServerStopping"}"#.to_string()
                    } else {
                        text
                    };

                    let json = serde_json::from_str::<serde_json::Value>(&text)
                        .map_err(InnerError::DeserializeMessage)?;

                    if let Some(message_id) = json
                        .as_object()
                        .and_then(|obj| obj.get("message-id"))
                        .and_then(|id| id.as_str())
                        .and_then(|id| id.parse().ok())
                    {
                        debug!("got message with id {}", message_id);
                        if let Some(tx) = receivers2.lock().await.remove(&message_id) {
                            tx.send(json).ok();
                        }
                    } else {
                        #[cfg(feature = "events")]
                        {
                            let event = serde_json::from_value(json)
                                .map_err(InnerError::DeserializeEvent)?;
                            events_tx.send(event).ok();
                        }
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = res {
                    error!("failed handling message: {:?}", e);
                }
            }

            #[cfg(feature = "events")]
            {
                let event = Event {
                    stream_timecode: None,
                    rec_timecode: None,
                    ty: EventType::ServerStopped,
                };
                events_tx.send(event).ok();
            }

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

        if !OBS_STUDIO_VERSION.matches(&version.obs_studio_version) {
            return Err(Error::ObsStudioVersion(
                version.obs_studio_version,
                OBS_STUDIO_VERSION,
            ));
        }

        if !OBS_WEBSOCKET_VERSION.matches(&version.obs_websocket_version) {
            return Err(Error::ObsWebsocketVersion(
                version.obs_websocket_version,
                OBS_WEBSOCKET_VERSION,
            ));
        }

        Ok(())
    }

    async fn send_message<T>(&self, req: RequestType<'_>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        let req = Request {
            message_id: &id.to_string(),
            ty: req,
        };
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

        let mut resp = rx.await.map_err(Error::ReceiveMessage)?;

        if let Some(error) = extract_error(&mut resp) {
            return Err(Error::Api(error));
        }

        serde_json::from_value::<Response<T>>(resp)
            .map(|r| r.details)
            .map_err(Error::DeserializeResponse)
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

    /// Login to the OBS websocket if an authentication is required.
    pub async fn login(&self, password: Option<impl AsRef<str>>) -> Result<()> {
        let auth_required = self.general().get_auth_required().await?;

        if let AuthRequired {
            auth_required: true,
            challenge: Some(challenge),
            salt: Some(salt),
        } = auth_required
        {
            match password {
                Some(password) => {
                    let auth = Self::create_auth_response(&challenge, &salt, password.as_ref());
                    self.general().authenticate(&auth).await?;
                }
                None => return Err(Error::NoPassword),
            }
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

    /// Access general API functions.
    pub fn general(&self) -> General<'_> {
        General { client: self }
    }

    /// Access API functions related to media control.
    pub fn media_control(&self) -> MediaControl<'_> {
        MediaControl { client: self }
    }

    /// Access API functions related to sources.
    pub fn sources(&self) -> Sources<'_> {
        Sources { client: self }
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

    /// Access API functions related to streaming.
    pub fn streaming(&self) -> Streaming<'_> {
        Streaming { client: self }
    }

    /// Access API functions related to the studio mode.
    pub fn studio_mode(&self) -> StudioMode<'_> {
        StudioMode { client: self }
    }

    /// Access API functions related to transitions.
    pub fn transitions(&self) -> Transitions<'_> {
        Transitions { client: self }
    }

    /// Access API functions related to the virtual cam.
    pub fn virtual_cam(&self) -> VirtualCam<'_> {
        VirtualCam { client: self }
    }
}

fn extract_error(value: &mut serde_json::Value) -> Option<String> {
    value
        .as_object_mut()
        .and_then(|o| o.get_mut("error"))
        .and_then(|e| {
            if let serde_json::Value::String(msg) = e.take() {
                Some(msg)
            } else {
                None
            }
        })
}

impl Drop for Client {
    fn drop(&mut self) {
        // We simply drop the future as the background task has been aborted but we have no way here
        // to wait for it to fully shut down (except spinning up a new tokio runtime).
        drop(self.disconnect());
    }
}
