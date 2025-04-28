# anidb-api

[![crates.io](https://img.shields.io/crates/v/anidb-api.svg)](https://crates.io/crates/anidb-api)
[![docs.rs](https://docs.rs/anidb-api/badge.svg)](https://docs.rs/anidb-api)

An asynchronous Rust client for interacting with the [AniDB HTTP API](https://wiki.anidb.net/HTTP_API_Definition). Features a built-in rate limiter for safe and respectful API usage.

## Features

*   Asynchronous operation.
*   Built-in rate limiter to comply with AniDB API limits.
*   Simple interface for fetching data.
*   Basic support for `tracing` logging.

## Status

*   **Implemented:**
    *   `get_anime`: Fetch detailed information about an anime by its ID.
*   **In Progress:**
    *   Fetching random recommendations.
    *   Fetching random similar anime.
    *   Fetching the hot anime list.
    *   Fetching main page data.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
anidb-api = "0.1.1"
```

## Usage Example

```rust
use anidb_api::http::{AniDbHttpClient, models::anime::Anime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```

## License

This project is licensed under the [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) License.
