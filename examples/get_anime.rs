use anidb_api::http::{AniDbHttpClient, models::anime::Anime};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME"),).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the client
    let anidb = AniDbHttpClient::new()?;
    // All IDs are Strings
    let anime_id = "17110";
    // Get xml data from AniDB and deserialize it into `Anime` struct
    let anime: Anime = anidb.get_anime(anime_id).await?;
    // Serialize this struct into json and write it into file, for testing purposes
    let file = std::fs::File::create("anime.json")?;
    serde_json::to_writer_pretty(file, &anime)?;
    println!("`Anime` struct serialized successfully!");
    Ok(())
}
