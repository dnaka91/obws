//! General responses, not fitting into any category.

use serde::Deserialize;

/// Response value for [`crate::client::General::version`].
#[derive(Debug, Deserialize)]
pub struct Version {
    /// Current OBS Studio version.
    #[serde(rename = "obsVersion")]
    pub obs_version: semver::Version,
    /// Current obs-websocket version.
    #[serde(rename = "obsWebSocketVersion")]
    pub obs_web_socket_version: semver::Version,
    /// Current latest obs-websocket RPC version.
    #[serde(rename = "rpcVersion")]
    pub rpc_version: u32,
    /// Array of available RPC requests for the currently negotiated RPC version.
    #[serde(rename = "availableRequests")]
    pub available_requests: Vec<String>,
    /// Image formats available in `GetSourceScreenshot` and `SaveSourceScreenshot` requests.
    #[serde(rename = "supportedImageFormats")]
    pub supported_image_formats: Vec<String>,
    /// Name of the platform. Usually `windows`, `macos`, or `ubuntu` (Linux flavor). Not guaranteed
    /// to be any of those.
    #[serde(rename = "platform")]
    pub platform: String,
    /// Description of the platform, like `Windows 10 (10.0)`.
    #[serde(rename = "platformDescription")]
    pub platform_description: String,
}

/// Response value for [`crate::client::General::stats`].
#[derive(Debug, Deserialize)]
pub struct Stats {
    /// Current CPU usage in percent.
    #[serde(rename = "cpuUsage")]
    pub cpu_usage: f64,
    /// Amount of memory in MB currently being used by OBS.
    #[serde(rename = "memoryUsage")]
    pub memory_usage: f64,
    /// Available disk space on the device being used for recording storage.
    #[serde(rename = "availableDiskSpace")]
    pub available_disk_space: f64,
    /// Current FPS being rendered.
    #[serde(rename = "activeFps")]
    pub active_fps: f64,
    /// Average time in milliseconds that OBS is taking to render a frame.
    #[serde(rename = "averageFrameRenderTime")]
    pub average_frame_render_time: f64,
    /// Number of frames skipped by OBS in the render thread.
    #[serde(rename = "renderSkippedFrames")]
    pub render_skipped_frames: u32,
    /// Total number of frames outputted by the render thread.
    #[serde(rename = "renderTotalFrames")]
    pub render_total_frames: u32,
    /// Number of frames skipped by OBS in the output thread.
    #[serde(rename=""outputSkippedFrames)]
    pub output_skipped_frames: u32,
    /// Total number of frames outputted by the output thread.
    #[serde(rename = "outputTotalFrames")]
    pub output_total_frames: u32,
    /// Total number of messages received by obs-websocket from the client.
    #[serde(rename = "webSocketSessionIncomingMessages")]
    pub web_socket_session_incoming_messages: u64,
    /// Total number of messages sent by obs-websocket to the client.
    #[serde(rename = "webSocketSessionOutgoingMessages")]
    pub web_socket_session_outgoing_messages: u64,
}

/// Response value for [`crate::client::General::call_vendor_request`].
#[derive(Debug, Deserialize)]
pub struct VendorResponse<T> {
    /// Name of the vendor.
    #[serde(rename = "vendorName")]
    pub vendor_name: String,
    /// Type of request.
    #[serde(rename = "requestType")]
    pub request_type: String,
    /// Object containing appropriate response data.
    #[serde(rename = "responseData")]
    pub response_data: T,
}
