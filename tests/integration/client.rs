use anyhow::{Result, anyhow};
use obws::{Client, client::ConnectConfig, error::Error, requests::EventSubscription};
use test_log::test;

use crate::common::{self, MockServer, Version};

#[test(tokio::test)]
async fn client() -> Result<()> {
    let (client, server) = common::new_client().await?;

    client.reidentify(EventSubscription::ALL).await?;

    server.stop().await
}

#[test(tokio::test)]
async fn invalid_obs_version() -> Result<()> {
    let (server, port) = MockServer::start(Version::builder().obs("1.0.0").build()).await?;

    let result = match Client::connect("127.0.0.1", port, Some("mock-password")).await {
        Err(Error::ObsStudioVersion(_, _)) => Ok(()),
        Err(err) => Err(anyhow!("failed with the wrong error: {err:?}")),
        Ok(_) => Err(anyhow!("should fail due to too low OBS version")),
    };

    server.stop().await?;
    result
}

#[test(tokio::test)]
async fn invalid_websocket_version() -> Result<()> {
    let (server, port) = MockServer::start(Version::builder().websocket("1.0.0").build()).await?;

    let result = match Client::connect("127.0.0.1", port, Some("mock-password")).await {
        Err(Error::ObsWebsocketVersion(_, _)) => Ok(()),
        Err(err) => Err(anyhow!("failed with the wrong error: {err:?}")),
        Ok(_) => Err(anyhow!("should fail due to too low obs-websocket version")),
    };

    server.stop().await?;
    result
}

#[test(tokio::test)]
async fn invalid_rpc_version() -> Result<()> {
    let (server, port) = MockServer::start(Version::builder().rpc(0).build()).await?;

    let result = match Client::connect("127.0.0.1", port, Some("mock-password")).await {
        Err(Error::RpcVersion { .. }) => Ok(()),
        Err(err) => Err(anyhow!("failed with the wrong error: {err:?}")),
        Ok(_) => Err(anyhow!("should fail due to too low RPC version")),
    };

    server.stop().await?;
    result
}

#[test(tokio::test)]
async fn ignore_version() -> Result<()> {
    let (server, port) =
        MockServer::start(Version::builder().obs("1.0.0").websocket("1.0.0").build()).await?;

    let config = ConnectConfig::builder("127.0.0.1", port)
        .password("mock-password")
        .dangerous(|cfg| {
            cfg.skip_studio_version_check(true)
                .skip_websocket_version_check(true)
        })
        .build();

    Client::connect_with_config(config).await?;

    server.stop().await
}
