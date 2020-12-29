//! # OBSWS - The obws (obvious) remote control library for OBS

#![warn(missing_docs, rust_2018_idioms, clippy::all)]

pub use client::Client;

pub mod client;
pub mod common;
pub mod events;
pub mod requests;
pub mod responses;

mod de;

/// Result type used throughout the crate that uses [`Error`] as default error.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur while using this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occured while trying to connect to the websocket.
    #[error("failed to connect to the obs-websocket plugin")]
    Connect(#[source] tungstenite::Error),
    /// Failed to serialize the message to be send to the websocket.
    #[error("failed to serialize message")]
    SerializeMessage(#[source] serde_json::Error),
    /// A message could not be send through the websocket.
    #[error("failed to send message to the obs-websocket plugin")]
    Send(#[source] tungstenite::Error),
    /// Tried to receive data while the send side was already closed.
    #[error("send side is closed")]
    ReceiveMessage(#[source] tokio::sync::oneshot::error::RecvError),
    /// Failed to deserialize the message that came back as response.
    #[error("the response message could not be deserialized")]
    DeserializeResponse(#[source] serde_json::Error),
    /// Failed to serialize custom user defined data for a message.
    #[error("failed to serialize custom data")]
    SerializeCustomData(#[source] serde_json::Error),
    /// An error returned from the obs-websocket API.
    #[error("API error: {0}")]
    Api(String),
    /// The obs-websocket API requires authentication but no password was given.
    #[error("authentication required but no password provided")]
    NoPassword,
    /// Unknown flags were found while trying to parse bitflags.
    #[error("value {0} contains unknown flags")]
    UnknownFlags(u8),
}
