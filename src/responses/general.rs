//! General responses, not fitting into any category.

use serde::Deserialize;

/// Response value for [`crate::client::General::version`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    /// Current OBS Studio version.
    pub obs_version: semver::Version,
    /// Current obs-websocket version.
    pub obs_web_socket_version: semver::Version,
    /// Current latest obs-websocket RPC version.
    pub rpc_version: u32,
    /// Array of available RPC requests for the currently negotiated RPC version.
    pub available_requests: Vec<String>,
    /// Image formats available in `GetSourceScreenshot` and `SaveSourceScreenshot` requests.
    pub supported_image_formats: Vec<String>,
    /// Name of the platform. Usually `windows`, `macos`, or `ubuntu` (Linux flavor). Not guaranteed
    /// to be any of those.
    pub platform: String,
    /// Description of the platform, like `Windows 10 (10.0)`.
    pub platform_description: String,
}

/// Response value for [`crate::client::General::stats`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Current CPU usage in percent.
    pub cpu_usage: f64,
    /// Amount of memory in MB currently being used by OBS.
    pub memory_usage: f64,
    /// Available disk space on the device being used for recording storage.
    pub available_disk_space: f64,
    /// Current FPS being rendered.
    pub active_fps: f64,
    /// Average time in milliseconds that OBS is taking to render a frame.
    pub average_frame_render_time: f64,
    /// Number of frames skipped by OBS in the render thread.
    pub render_skipped_frames: u32,
    /// Total number of frames outputted by the render thread.
    pub render_total_frames: u32,
    /// Number of frames skipped by OBS in the output thread.
    pub output_skipped_frames: u32,
    /// Total number of frames outputted by the output thread.
    pub output_total_frames: u32,
    /// Total number of messages received by obs-websocket from the client.
    pub web_socket_session_incoming_messages: u64,
    /// Total number of messages sent by obs-websocket to the client.
    pub web_socket_session_outgoing_messages: u64,
}

/// Response value for [`crate::client::General::call_vendor_request`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorResponse<T> {
    /// Name of the vendor.
    pub vendor_name: String,
    /// Type of request.
    pub request_type: String,
    /// Object containing appropriate response data.
    pub response_data: T,
}
