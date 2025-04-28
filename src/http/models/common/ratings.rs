use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Ratings {
    pub permanent: Option<AnimeRating>,
    pub temporary: Option<AnimeRating>,
    pub review: Option<AnimeRating>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnimeRating {
    #[serde(rename = "count")]
    pub votes: u32,
    #[serde(rename(deserialize = "$value"))]
    pub value: f32,
}
