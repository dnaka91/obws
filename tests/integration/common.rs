use std::net::Ipv4Addr;

use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose};
use futures_util::{SinkExt, StreamExt};
use obws::{
    Client,
    events::Event,
    requests::{EventSubscription, inputs::InputId, scenes::SceneId},
    responses::StatusCode,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{self, Message},
};
use tracing::{debug, error, info};

pub const TEST_SCENE: SceneId<'_> = SceneId::Name("OBWS-TEST-Scene");
pub const TEST_SCENE_2: SceneId<'_> = SceneId::Name("OBWS-TEST-Scene2");
pub const TEST_SCENE_RENAME: SceneId<'_> = SceneId::Name("OBWS-TEST-Scene-Renamed");
pub const TEST_SCENE_CREATE: SceneId<'_> = SceneId::Name("OBWS-TEST-Scene-Created");
pub const TEST_TEXT: InputId<'_> = InputId::Name("OBWS-TEST-Text");
pub const TEST_BROWSER: InputId<'_> = InputId::Name("OBWS-TEST-Browser");
pub const TEST_BROWSER_RENAME: InputId<'_> = InputId::Name("OBWS-TEST-Browser-Renamed");
pub const TEST_MEDIA: InputId<'_> = InputId::Name("OBWS-TEST-Media");
pub const TEST_GROUP: SceneId<'_> = SceneId::Name("OBWS-TEST-Group");
pub const TEST_TRANSITION: &str = "OBWS-TEST-Transition";
pub const TEST_FILTER: &str = "OBWS-TEST-Filter";
pub const TEST_FILTER_2: &str = "OBWS-TEST-Filter2";
pub const TEST_FILTER_RENAME: &str = "OBWS-TEST-Filter-Renamed";
pub const INPUT_KIND_BROWSER: &str = "browser_source";
pub const INPUT_KIND_VLC: &str = "vlc_source";
pub const FILTER_COLOR: &str = "color_filter";

pub async fn new_client() -> Result<(Client, MockServer)> {
    let (server, port) = MockServer::start(Version::builder().build()).await?;
    let client = Client::connect("localhost", port, Some("mock-password")).await?;

    Ok((client, server))
}

#[macro_export]
macro_rules! wait_for {
    ($expression:expr, $pattern:pat) => {{
        use futures_util::stream::StreamExt;

        while let Some(event) = $expression.next().await {
            if matches!(event, $pattern) {
                break;
            }
        }
    }};
}

pub struct MockServer {
    handle: JoinHandle<Result<()>>,
    shutdown: Option<oneshot::Sender<()>>,
    expectations: mpsc::UnboundedSender<Expectation>,
    events: mpsc::UnboundedSender<Event>,
}

#[derive(Clone, Copy, bon::Builder)]
pub struct Version {
    #[builder(default = "31.0.0")]
    pub obs: &'static str,
    #[builder(default = "5.5.0")]
    pub websocket: &'static str,
    #[builder(default = 1)]
    pub rpc: u32,
}

impl MockServer {
    pub async fn start(version: Version) -> Result<(Self, u16)> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await?;
        let port = listener.local_addr()?.port();
        debug!("server started");

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (expect_tx, mut expect_rx) = mpsc::unbounded_channel();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel();

        let handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await?;
            let mut stream = tokio_tungstenite::accept_async(stream).await?;
            debug!("connected");

            handshake(&mut stream).await?;
            debug!("handshake done");
            version_check(&mut stream, version).await?;
            debug!("version check done");

            loop {
                select! {
                    _ = &mut shutdown_rx => break,
                    Some(msg) = stream.next() => {
                        handle_ws_message(&mut stream, &mut expect_rx, msg).await?;
                    }
                    Some(event) = event_rx.recv() => {
                        handle_event(&mut stream, event).await?;
                    }
                }
            }

            anyhow::Ok(())
        });

        Ok((
            Self {
                handle,
                shutdown: Some(shutdown_tx),
                expectations: expect_tx,
                events: event_tx,
            },
            port,
        ))
    }

    pub async fn stop(mut self) -> Result<()> {
        if let Some(tx) = self.shutdown.take() {
            tx.send(()).ok();
        }
        self.handle.await?
    }

    pub fn expect<Req, Rsp>(&self, name: &str, req: Req, rsp: Rsp)
    where
        Req: Serialize,
        Rsp: Serialize,
    {
        self.expectations
            .send(Expectation {
                name: name.to_owned(),
                req: serde_json::to_value(req).unwrap(),
                rsp: serde_json::to_value(rsp).unwrap(),
            })
            .unwrap();
    }

    pub fn send_event(&self, event: Event) {
        self.events.send(event).unwrap();
    }
}

struct Expectation {
    name: String,
    req: serde_json::Value,
    rsp: serde_json::Value,
}

async fn handshake(stream: &mut WebSocketStream<TcpStream>) -> Result<()> {
    let hello = ServerMessage::Hello(Hello {
        obs_web_socket_version: semver::Version::new(5, 5, 0),
        rpc_version: 1,
        authentication: Some(Authentication {
            challenge: "mock-challenge".to_owned(),
            salt: "mock-salt".to_owned(),
        }),
    });

    stream
        .send(Message::text(serde_json::to_string(&hello)?))
        .await?;

    let identify = stream.next().await.context("no message from client")??;
    let ClientMessage::Identify(identify) =
        serde_json::from_str::<ClientMessage>(identify.to_text()?)?
    else {
        bail!("unexpected client message");
    };

    ensure!(identify.rpc_version == 1);
    ensure!(identify.event_subscriptions == None);
    verify_auth(&identify)?;

    let identified = ServerMessage::Identified(Identified {
        negotiated_rpc_version: 1,
    });

    stream
        .send(Message::text(serde_json::to_string(&identified)?))
        .await?;

    Ok(())
}

fn verify_auth(identify: &Identify) -> Result<()> {
    let mut hasher = Sha256::new();
    hasher.update(b"mock-password");
    hasher.update(b"mock-salt");

    let intermediate = general_purpose::STANDARD.encode(hasher.finalize_reset());
    hasher.update(intermediate.as_bytes());
    hasher.update(b"mock-challenge");

    let auth = general_purpose::STANDARD.encode(hasher.finalize());
    ensure!(Some(auth) == identify.authentication);

    Ok(())
}

async fn version_check(stream: &mut WebSocketStream<TcpStream>, version: Version) -> Result<()> {
    let request = stream.next().await.context("no message from client")??;
    let request = serde_json::from_str::<ClientMessage>(request.to_text()?)?;

    let ClientMessage::Request(request) = request else {
        bail!("unexpected client message");
    };

    ensure!(request.request_type == "GetVersion");

    let response = ServerMessage::RequestResponse(RequestResponse {
        request_type: request.request_type,
        request_id: request.request_id,
        request_status: Status::ok(),
        response_data: json! {{
            "obsVersion": version.obs,
            "obsWebSocketVersion": version.websocket,
            "rpcVersion": version.rpc,
            "availableRequests": [],
            "supportedImageFormats": [],
            "platform": "mock",
            "platformDescription": "",
        }},
    });

    stream
        .send(Message::text(serde_json::to_string(&response)?))
        .await?;

    Ok(())
}

async fn handle_ws_message(
    stream: &mut WebSocketStream<TcpStream>,
    expect_rx: &mut mpsc::UnboundedReceiver<Expectation>,
    msg: tungstenite::Result<Message>,
) -> Result<()> {
    match msg {
        Ok(msg) => {
            let msg = serde_json::from_str::<ClientMessage>(msg.to_text()?)?;
            info!(message = ?msg);

            match msg {
                ClientMessage::Identify(identify) => {
                    bail!("should never get a second `Identify` message: {identify:?}")
                }
                ClientMessage::Reidentify(reidentify) => {
                    debug!(?reidentify, "received reidentification request");
                    ensure!(reidentify.event_subscriptions != None);

                    let identified = ServerMessage::Identified(Identified {
                        negotiated_rpc_version: 1,
                    });

                    stream
                        .send(Message::text(serde_json::to_string(&identified)?))
                        .await?;
                }
                ClientMessage::Request(request) => {
                    let expect = expect_rx
                        .recv()
                        .await
                        .context("no expectations for request")?;

                    ensure!(expect.name == request.request_type);
                    ensure!(expect.req == request.request_data);

                    stream
                        .send(Message::text(serde_json::to_string(
                            &ServerMessage::RequestResponse(RequestResponse {
                                request_type: request.request_type,
                                request_id: request.request_id,
                                request_status: Status::ok(),
                                response_data: expect.rsp,
                            }),
                        )?))
                        .await?;
                }
            }
        }
        Err(err) => error!(?err),
    }

    Ok(())
}

async fn handle_event(stream: &mut WebSocketStream<TcpStream>, event: Event) -> Result<()> {
    let msg = ServerMessage::Event(event);

    stream
        .send(Message::text(serde_json::to_string(&msg)?))
        .await
        .map_err(Into::into)
}

enum ServerMessage {
    Hello(Hello),
    Identified(Identified),
    Event(Event),
    RequestResponse(RequestResponse),
}

impl Serialize for ServerMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct RawMessage<T> {
            op: OpCode,
            d: T,
        }

        #[derive(Serialize_repr)]
        #[repr(u8)]
        enum OpCode {
            Hello = 0,
            Identified = 2,
            Event = 5,
            RequestResponse = 7,
        }

        match self {
            ServerMessage::Hello(d) => RawMessage {
                op: OpCode::Hello,
                d,
            }
            .serialize(serializer),
            ServerMessage::Identified(d) => RawMessage {
                op: OpCode::Identified,
                d,
            }
            .serialize(serializer),
            ServerMessage::Event(d) => RawMessage {
                op: OpCode::Event,
                d,
            }
            .serialize(serializer),
            ServerMessage::RequestResponse(d) => RawMessage {
                op: OpCode::RequestResponse,
                d,
            }
            .serialize(serializer),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Hello {
    obs_web_socket_version: semver::Version,
    rpc_version: u32,
    authentication: Option<Authentication>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Authentication {
    challenge: String,
    salt: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Identified {
    pub negotiated_rpc_version: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestResponse {
    request_type: String,
    request_id: String,
    request_status: Status,
    response_data: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    result: bool,
    code: StatusCode,
    comment: Option<String>,
}

impl Status {
    const fn ok() -> Self {
        Self {
            result: true,
            code: StatusCode::NoError,
            comment: None,
        }
    }
}

#[derive(Debug)]
enum ClientMessage {
    Identify(Identify),
    Reidentify(Reidentify),
    Request(Request),
}

impl<'de> Deserialize<'de> for ClientMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMessage {
            op: OpCode,
            d: serde_json::Value,
        }

        #[derive(Deserialize_repr)]
        #[repr(u8)]
        enum OpCode {
            Identify = 1,
            Reidentify = 3,
            Request = 6,
        }

        let raw = RawMessage::deserialize(deserializer)?;

        Ok(match raw.op {
            OpCode::Identify => {
                ClientMessage::Identify(serde_json::from_value(raw.d).map_err(de::Error::custom)?)
            }
            OpCode::Reidentify => {
                ClientMessage::Reidentify(serde_json::from_value(raw.d).map_err(de::Error::custom)?)
            }
            OpCode::Request => {
                ClientMessage::Request(serde_json::from_value(raw.d).map_err(de::Error::custom)?)
            }
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identify {
    rpc_version: u32,
    authentication: Option<String>,
    event_subscriptions: Option<EventSubscription>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Reidentify {
    event_subscriptions: Option<EventSubscription>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    request_id: String,
    request_type: String,
    #[serde(default)]
    request_data: serde_json::Value,
}
