//! The client to the obs-websocket API and main entry point.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, Stream, StreamExt},
};
use log::{debug, error, trace};
use serde::de::DeserializeOwned;
use tokio::{
    net::TcpStream,
    sync::{broadcast, oneshot, Mutex},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    events::Event,
    requests::{Request, RequestType},
    responses::{AuthRequired, Response},
    Error, Result,
};

pub use self::{
    general::General, outputs::Outputs, profiles::Profiles, recording::Recording,
    replay_buffer::ReplayBuffer, scene_collections::SceneCollections, scene_items::SceneItems,
    scenes::Scenes, sources::Sources, streaming::Streaming, studio_mode::StudioMode,
    transitions::Transitions,
};

mod general;
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

#[derive(Debug, thiserror::Error)]
enum InnerError {
    #[error("websocket message not convertible to text")]
    IntoText(#[source] tungstenite::Error),
    #[error("failed deserializing message")]
    DeserializeMessage(#[source] serde_json::Error),
    #[error("failed deserializing event")]
    DeserializeEvent(#[source] serde_json::Error),
}

/// The client is the main entry point to access the obs-websocket API. It allows to call various
/// functions to remote control an OBS instance as well as to listen to events caused by the user
/// by interacting with OBS.
pub struct Client {
    write: Mutex<MessageWriter>,
    id_counter: AtomicU64,
    receivers: Arc<Mutex<HashMap<String, oneshot::Sender<serde_json::Value>>>>,
    event_sender: broadcast::Sender<Event>,
}

type MessageWriter = SplitSink<WebSocketStream<TcpStream>, Message>;

impl Client {
    /// Connect to a obs-websocket instance on the given host and port.
    pub async fn connect(host: impl AsRef<str>, port: u16) -> Result<Self> {
        let (socket, _) =
            tokio_tungstenite::connect_async(format!("ws://{}:{}", host.as_ref(), port))
                .await
                .map_err(Error::Connect)?;
        let (write, mut read) = socket.split();
        let receivers = Arc::new(Mutex::new(HashMap::<
            String,
            oneshot::Sender<serde_json::Value>,
        >::new()));
        let receivers2 = Arc::clone(&receivers);
        let (event_sender, _) = broadcast::channel(100);
        let events_tx = event_sender.clone();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                trace!("{}", msg);
                let res: Result<(), InnerError> = async {
                    let text = msg.into_text().map_err(InnerError::IntoText)?;
                    let json = serde_json::from_str::<serde_json::Value>(&text)
                        .map_err(InnerError::DeserializeMessage)?;

                    if let Some(message_id) = json
                        .as_object()
                        .and_then(|obj| obj.get("message-id"))
                        .and_then(|id| id.as_str())
                    {
                        debug!("got message with id {}", message_id);
                        if let Some(tx) = receivers2.lock().await.remove(message_id) {
                            tx.send(json).ok();
                        }
                    } else {
                        let event =
                            serde_json::from_value(json).map_err(InnerError::DeserializeEvent)?;
                        events_tx.send(event).ok();
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = res {
                    error!("failed handling message: {:?}", e);
                }
            }
        });

        let write = Mutex::new(write);
        let id_counter = AtomicU64::new(1);

        Ok(Self {
            write,
            id_counter,
            receivers,
            event_sender,
        })
    }

    async fn send_message<T>(&self, req: RequestType) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst).to_string();
        let req = Request {
            message_id: id.clone(),
            ty: req,
        };
        let json = serde_json::to_string(&req).map_err(Error::SerializeMessage)?;

        let (tx, rx) = oneshot::channel();
        self.receivers.lock().await.insert(id, tx);

        debug!("sending message: {}", json);
        self.write
            .lock()
            .await
            .send(Message::Text(json))
            .await
            .map_err(Error::Send)?;

        let mut resp = rx.await.map_err(Error::ReceiveMessage)?;

        if let Some(error) = extract_error(&mut resp) {
            return Err(Error::Api(error));
        }

        serde_json::from_value::<Response<T>>(resp)
            .map(|r| r.details)
            .map_err(Error::DeserializeResponse)
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
                    self.general().authenticate(auth).await?;
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
    pub fn events(&self) -> impl Stream<Item = Event> {
        let mut receiver = self.event_sender.subscribe();

        async_stream::stream! {
            while let Ok(event) = receiver.recv().await {
                yield event;
            }
        }
    }

    /// Access general API functions.
    pub fn general(&self) -> General<'_> {
        General { client: self }
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
