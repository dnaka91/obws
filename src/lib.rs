//! # OBWS - The obws (obvious) remote control library for OBS
//!
//! Remote control OBS with the [obs-websocket] plugin from Rust 🦀.
//!
//! [obs-websocket]: https://github.com/Palakis/obs-websocket
//!
//! ## Example
//!
//! ```no_run
//! use anyhow::Result;
//! use obws::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     /// Connect to the OBS instance through obs-websocket.
//!     let client = Client::connect("localhost", 4455, Some("password")).await?;
//!
//!     /// Get and print out version information of OBS and obs-websocket.
//!     let version = client.general().version().await?;
//!     println!("{:#?}", version);
//!
//!     /// Get a list of available scenes and print them out.
//!     let scene_list = client.scenes().list().await?;
//!     println!("{:#?}", scene_list);
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs, rust_2018_idioms, clippy::all)]

use responses::StatusCode;
pub use semver::{Comparator, Version};

pub use self::client::Client;

pub mod client;
pub mod common;
#[cfg(feature = "events")]
pub mod events;
pub mod requests;
pub mod responses;

mod serde;

/// Result type used throughout the crate that uses [`Error`] as default error.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur while using this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while trying to connect to the web-socket.
    #[error("failed to connect to the obs-websocket plugin")]
    Connect(#[source] tokio_tungstenite::tungstenite::Error),
    /// The initial handshake with `obs-websocket` didn't succeed.
    #[error("failed to execute the handshake with obs-websocket")]
    Handshake(#[from] crate::client::HandshakeError),
    /// Failed to serialize the message to be send to the web-socket.
    #[error("failed to serialize message")]
    SerializeMessage(#[source] serde_json::Error),
    /// A message could not be send through the web-socket.
    #[error("failed to send message to the obs-websocket plugin")]
    Send(#[source] tokio_tungstenite::tungstenite::Error),
    /// Tried to receive data while the send side was already closed.
    #[error("send side is closed")]
    ReceiveMessage(#[source] tokio::sync::oneshot::error::RecvError),
    /// Failed to deserialize the message that came back as response.
    #[error("the response message could not be deserialized")]
    DeserializeResponse(#[source] serde_json::Error),
    /// Failed to serialize custom user defined data for a message.
    #[error("failed to serialize custom data")]
    SerializeCustomData(#[source] serde_json::Error),
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
    /// The obs-websocket API requires authentication but no password was given.
    #[error("authentication required but no password provided")]
    NoPassword,
    /// Unknown flags were found while trying to parse bitflags.
    #[error("value {0} contains unknown flags")]
    UnknownFlags(u8),
    /// Tried to interact with obs-websocket while not connected (for example trying to get a new
    /// event stream).
    #[error("currently not connected to obs-websocket")]
    Disconnected,
    /// The OBS studio version of the connected instance doesn't match the required version for this
    /// crate.
    #[error("obs studio version {0} doesn't match required {1}")]
    ObsStudioVersion(Version, Comparator),
    /// The obs-websocket plugin version doesn't match the required version for this crate.
    #[error("obs-websocket version {0} doesn't match required {1}")]
    ObsWebsocketVersion(Version, Comparator),
    /// The obs-websocket plugin negotiated a different RPC version than requested.
    #[error("RPC version {requested} requested, but server negotiated version {negotiated}")]
    RpcVersion {
        /// Version requested by the client.
        requested: u32,
        /// Unexpected version as negotiated by the server.
        negotiated: u32,
    },
}
