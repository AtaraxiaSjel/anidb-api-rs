use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct AnimeSimilarList {
    #[serde(rename = "anime")]
    pub list: Vec<AnimeSimilar>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnimeSimilar {
    #[serde(rename = "id")]
    pub anime_id: String,
    pub approval: u32,
    pub total: u32,
    #[serde(rename(deserialize = "$value"))]
    pub name: String,
}
