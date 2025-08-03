//! Various error types that can occur while using this crate.

use crate::responses::StatusCode;

/// Result type used throughout the crate that uses [`Error`] as default error.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur while using this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Failed to construct a valid URI for connecting.
    #[error("failed constructing a valid URI")]
    InvalidUri(#[from] InvalidUriError),
    /// An error occurred while trying to connect to the web-socket.
    #[error("failed to connect to the obs-websocket plugin")]
    Connect(#[from] ConnectError),
    /// The set connection timeout was reached before the connection could be created.
    #[error("timeout happened before the connection could be established")]
    Timeout,
    /// The initial handshake with `obs-websocket` didn't succeed.
    #[error("failed to execute the handshake with obs-websocket")]
    Handshake(#[from] crate::client::HandshakeError),
    /// Failed to serialize the message to be send to the web-socket.
    #[error("failed to serialize message")]
    SerializeMessage(#[from] SerializeMessageError),
    /// A message could not be send through the web-socket.
    #[error("failed to send message to the obs-websocket plugin")]
    Send(#[from] SendError),
    /// Tried to receive data while the send side was already closed.
    #[error("send side is closed")]
    ReceiveMessage(#[from] ReceiveMessageError),
    /// Failed to deserialize the message that came back as response.
    #[error("the response message could not be deserialized")]
    DeserializeResponse(#[from] DeserializeResponseError),
    /// Failed to serialize custom user defined data for a message.
    #[error("failed to serialize custom data")]
    SerializeCustomData(#[from] SerializeCustomDataError),
    /// Custom data didn't serialize into a JSON object.
    #[error("custom data must serialize into a JSON object")]
    InvalidCustomData,
    /// An error returned from the obs-websocket API.
    #[error("API error: {code:?}")]
    Api {
        /// Status code that describes the kind of error.
        code: StatusCode,
        /// Optional message to provide additional details about the error.
        message: Option<String>,
    },
    /// Unknown flags were found while trying to parse bitflags.
    #[error("value {0} contains unknown flags")]
    UnknownFlags(u8),
    /// Tried to interact with obs-websocket while not connected (for example trying to get a new
    /// event stream).
    #[error("currently not connected to obs-websocket")]
    Disconnected,
    /// The OBS studio version of the connected instance doesn't match the required version for
    /// this crate.
    #[error("obs studio version {0} doesn't match required {1}")]
    ObsStudioVersion(semver::Version, semver::Comparator),
    /// The obs-websocket plugin version doesn't match the required version for this crate.
    #[error("obs-websocket version {0} doesn't match required {1}")]
    ObsWebsocketVersion(semver::Version, semver::Comparator),
    /// The obs-websocket plugin negotiated a different RPC version than requested.
    #[error("RPC version {requested} requested, but server negotiated version {negotiated}")]
    RpcVersion {
        /// Version requested by the client.
        requested: u32,
        /// Unexpected version as negotiated by the server.
        negotiated: u32,
    },
}

/// Failed constructing a valid URI.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct InvalidUriError(pub(crate) http::uri::InvalidUri);

/// An error occurred while trying to connect to the web-socket.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ConnectError(pub(crate) tokio_websockets::Error);

/// Failed to serialize the message to be send to the web-socket.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializeMessageError(pub(crate) serde_json::Error);

/// A message could not be send through the web-socket.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SendError(pub(crate) tokio_websockets::Error);

/// Tried to receive data while the send side was already closed.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ReceiveMessageError(pub(crate) tokio::sync::oneshot::error::RecvError);

/// Failed to deserialize the message that came back as response.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializeResponseError(pub(crate) serde_json::Error);

/// Failed to serialize custom user defined data for a message.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializeCustomDataError(pub(crate) serde_json::Error);
