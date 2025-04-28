use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct RecommendationList {
    #[serde(rename = "recommendation")]
    pub list: Vec<Recommendation>,
    pub total: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Recommendation {
    #[serde(rename = "type")]
    pub recommendation_type: RecommendationType,
    #[serde(rename = "uid")]
    pub user_id: String,
    #[serde(rename(deserialize = "$value"))]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum RecommendationType {
    #[serde(rename = "For Fans")]
    ForFans,
    #[serde(rename = "Must See")]
    MustSee,
    Recommended,
}
