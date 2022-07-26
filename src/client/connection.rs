use std::collections::{HashMap, VecDeque};

use futures_util::{Sink, SinkExt, Stream, StreamExt};
use tokio::{
    sync::{oneshot, Mutex},
    time::{self, Duration},
};
pub use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::Message;
use tracing::debug;

use super::InnerError;
use crate::{
    requests::{ClientRequest, EventSubscription, Identify},
    responses::{Hello, Identified, RequestResponse, ServerMessage, Status},
};

/// Wrapper for the list of ongoing requests that wait for response.
#[derive(Default)]
pub(super) struct ReceiverList(Mutex<HashMap<u64, oneshot::Sender<(Status, serde_json::Value)>>>);

impl ReceiverList {
    /// Add a new receiver to the wait list, that will be notified once a request with the given
    /// ID is received.
    pub async fn add(&self, id: u64) -> oneshot::Receiver<(Status, serde_json::Value)> {
        let (tx, rx) = oneshot::channel();
        self.0.lock().await.insert(id, tx);
        rx
    }

    /// Remove a previously added receiver. Used to free up resources, in case sending the request
    /// failed.
    pub async fn remove(&self, id: u64) {
        self.0.lock().await.remove(&id);
    }

    /// Notify a waiting receiver with the response to a request.
    pub async fn notify(&self, response: RequestResponse) -> Result<(), InnerError> {
        let RequestResponse {
            r#type: _,
            id,
            status,
            data,
        } = response;

        let id = id
            .parse()
            .map_err(|e| InnerError::InvalidRequestId(e, id))?;

        if let Some(tx) = self.0.lock().await.remove(&id) {
            tx.send((status, data)).ok();
        }

        Ok(())
    }

    /// Reset the list, canceling any outstanding receivers.
    pub async fn reset(&self) {
        self.0.lock().await.clear();
    }
}

/// Wrapper around a thread-safe queue to park and notify re-identify listener.
#[derive(Default)]
pub(super) struct ReidentifyReceiverList(Mutex<VecDeque<oneshot::Sender<Identified>>>);

impl ReidentifyReceiverList {
    /// Add a new receiver to the wait list, returning a channel to await the result on.
    pub async fn add(&self) -> oneshot::Receiver<Identified> {
        let (tx, rx) = oneshot::channel();
        self.0.lock().await.push_back(tx);
        rx
    }

    /// Notify the next listener in the queue, transferring it the response.
    pub async fn notify(&self, identified: Identified) {
        if let Some(tx) = self.0.lock().await.pop_front() {
            tx.send(identified).ok();
        }
    }

    /// Reset the list, canceling any outstanding receivers.
    pub async fn reset(&self) {
        self.0.lock().await.clear();
    }
}

/// Errors that can occur while performing the initial handshake with obs-websocket.
#[derive(Debug, thiserror::Error)]
pub enum HandshakeError {
    /// The connection to obs-websocket was interrupted while trying to read a message.
    #[error("connection to obs-websocket was closed: {}", match .0 {
        Some(details) => &details.reason,
        None => "no details provided",
    })]
    ConnectionClosed(Option<CloseDetails>),
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

/// Description about the reason of why the web-socket connection was closed.
#[derive(Debug)]
pub struct CloseDetails {
    /// Close code to precisely identify the reason.
    ///
    /// This can be turned into a [`u16`] and compared against the
    /// [`WebSocketCloseCode`](crate::responses::WebSocketCloseCode) to further identify the close
    /// reason, if related to `obs-websocket`.
    pub code: CloseCode,
    /// Textual representation of the close code, or additional details for it.
    pub reason: String,
}

pub(super) async fn handshake(
    write: &mut (impl Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin),
    read: &mut (impl Stream<Item = tokio_tungstenite::tungstenite::Result<Message>> + Unpin),
    password: Option<&str>,
    event_subscriptions: Option<EventSubscription>,
) -> Result<(), HandshakeError> {
    async fn read_message(
        read: &mut (impl Stream<Item = tokio_tungstenite::tungstenite::Result<Message>> + Unpin),
    ) -> Result<ServerMessage, HandshakeError> {
        let mut message = read
            .next()
            .await
            .ok_or(HandshakeError::ConnectionClosed(None))?
            .map_err(HandshakeError::Receive)?;

        if let Message::Close(info) = &mut message {
            return Err(HandshakeError::ConnectionClosed(info.take().map(|i| {
                CloseDetails {
                    code: i.code,
                    reason: i.reason.into_owned(),
                }
            })));
        }

        let message = message.into_text().map_err(HandshakeError::IntoText)?;

        serde_json::from_str::<ServerMessage>(&message).map_err(HandshakeError::DeserializeMessage)
    }

    let server_message = time::timeout(Duration::from_secs(5), read_message(read))
        .await
        .map_err(|_| HandshakeError::NoHello)?;

    match server_message? {
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
            debug!(rpc_version = %negotiated_rpc_version, "identified against obs-websocket");
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
