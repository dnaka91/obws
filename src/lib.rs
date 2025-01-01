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
//!
//! ## Differences to obs-websocket API design
//!
//! You may notice that several functions are named differently from the original `obs-websocket`
//! documentation. To help you find the right functions, have a look at [`docs::mapping`].

#![warn(missing_docs, rust_2018_idioms, clippy::all)]

pub use self::client::Client;

pub mod client;
pub mod common;
#[cfg(doc)]
pub mod docs;
pub mod error;
#[cfg(feature = "events")]
pub mod events;
pub mod requests;
pub mod responses;

mod serde;
