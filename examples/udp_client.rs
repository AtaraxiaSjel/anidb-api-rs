use std::env;

use anidb_api::udp::{ApiError, UdpClient, UdpConfig, UdpResponse, command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;

    test_command_anime_enc().await?;

    Ok(())
}

async fn test_command_anime_enc() -> anidb_api::udp::Result<()> {
    let config = UdpConfig {
        local_port: 60000,
        username: env::var("USERNAME").unwrap().into(),
        password: env::var("PASSWORD").unwrap().into(),
        udp_api_key: Some(env::var("API_KEY").unwrap().into()),
        client_name: env::var("CLIENT_NAME").unwrap().into(),
        client_ver: "1".into(),
        encryption: true,
    };
    let mut client = UdpClient::new(config).await?;
    let resp: UdpResponse = client.execute(command::Encrypt()).await?;
    dbg!(&resp);
    let resp: UdpResponse = client.execute(command::Auth()).await?;
    dbg!(&resp);
    let resp: std::result::Result<UdpResponse, ApiError> = client
        .execute(command::AnimeDescription("15201".into()))
        .await;
    dbg!(&resp);
    let resp: UdpResponse = client.execute(command::Logout()).await?;
    dbg!(&resp);
    Ok(())
}
