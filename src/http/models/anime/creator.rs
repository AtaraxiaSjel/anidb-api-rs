use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct CreatorList {
    #[serde(rename = "name")]
    pub list: Vec<Creator>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Creator {
    #[serde(rename = "id")]
    pub creator_id: String,
    #[serde(rename = "type")]
    pub creator_type: String,
    #[serde(rename(deserialize = "$value"))]
    pub name: String,
}
