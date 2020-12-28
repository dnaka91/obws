//! # OBSWS - The obws (obvious) remote control library for OBS

#![warn(missing_docs, rust_2018_idioms, clippy::all)]

pub mod client;
pub mod common;
pub mod events;
pub mod requests;
pub mod responses;

mod de;
