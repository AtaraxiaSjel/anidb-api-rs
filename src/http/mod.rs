mod client;
/// Various types for `AniDB` data
pub mod models;

/// Async `AniDB` client
pub use client::AniDbHttpClient;
/// Main error type for http client
pub use models::common::ApiError;
