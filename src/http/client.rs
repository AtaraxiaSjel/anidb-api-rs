use super::models::{
    anime::Anime,
    common::{ApiError, ResponseError},
};
use async_trait::async_trait;
use governor::Quota;
use reqwest::Method;
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use std::time::Duration;

const API_URL: &str = "http://api.anidb.net:9001/httpapi";
const CLIENT_NAME: &str = "anidbapirs";
const CLIENT_VER: usize = 1;
const HTTP_PROTO_VER: usize = 1;

#[derive(Clone)]
pub struct AniDbHttpClient {
    base_url: reqwest::Url,
    client: reqwest_middleware::ClientWithMiddleware,
}

struct MyRateLimiter {
    limiter: governor::DefaultDirectRateLimiter,
}

#[async_trait]
impl reqwest_ratelimit::RateLimiter for MyRateLimiter {
    async fn acquire_permit(&self) {
        self.limiter.until_ready().await;
    }
}

impl AniDbHttpClient {
    /// Returns client with default rate limit (1 request per 2 seconds).
    /// # Errors
    /// This method fails if reqwest client has failed to initialized
    pub fn new() -> Result<Self, ApiError> {
        let limit = Duration::from_secs(2);
        Self::with_ratelimit(limit)
    }

    /// Returns client with custom rate limit.
    /// Disables rate limiting if `limit` equals 0.
    /// # Errors
    /// This method fails if reqwest client has failed to initialized
    pub fn with_ratelimit(limit: Duration) -> Result<Self, ApiError> {
        let base_url = format!(
            "{API_URL}?client={CLIENT_NAME}&clientver={CLIENT_VER}&protover={HTTP_PROTO_VER}"
        );
        let base_url = reqwest::Url::parse(&base_url).map_err(ApiError::UrlParse)?;

        let req_client = reqwest::Client::builder()
            .gzip(true)
            .build()
            .map_err(|e| ApiError::Reqwest(e.into()))?;

        let client = if let Some(quota) = Quota::with_period(limit) {
            let rate_limiter = MyRateLimiter {
                limiter: governor::RateLimiter::direct(quota),
            };
            ClientBuilder::new(req_client).with(reqwest_ratelimit::all(rate_limiter))
        } else {
            ClientBuilder::new(req_client)
        };

        let client = client.with(TracingMiddleware::default()).build();
        Ok(Self { base_url, client })
    }

    /// Retrieve information for a specific anime by AID (anidb anime id).
    /// # Errors
    /// This method fails if reqwest cannot get response, url cannot be parsed, anidb server returns error or deserialization failed.
    pub async fn get_anime(&self, anime_id: &str) -> Result<Anime, ApiError> {
        let params = [("request", "anime"), ("aid", anime_id)];
        let url = reqwest::Url::parse_with_params(self.base_url.as_str(), &params)
            .map_err(ApiError::UrlParse)?;
        let request = self.client.request(Method::GET, url);
        let response = request.send().await.map_err(ApiError::Reqwest)?;

        // AniDB always return success status code
        // Check status from body
        let body_text = response
            .text()
            .await
            .map_err(|e| ApiError::Reqwest(e.into()))?;

        let anidb_error = serde_xml_rs::from_str::<ResponseError>(&body_text);

        if let Ok(err) = anidb_error {
            Err(ApiError::HttpError {
                status: err.status.unwrap_or_default(),
                message: err.text.unwrap_or("Empty error message".to_string()),
            })
        } else {
            Ok(serde_xml_rs::from_str::<Anime>(&body_text).map_err(ApiError::Deserialize)?)
        }
    }
}
