use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct TagList {
    #[serde(rename = "tag")]
    pub list: Vec<Tag>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    #[serde(rename = "id")]
    pub tag_id: String,
    #[serde(rename = "parentid")]
    pub parent_id: Option<String>,
    pub weight: u32,
    #[serde(rename = "localspoiler")]
    pub local_spoiler: bool,
    #[serde(rename = "globalspoiler")]
    pub global_spoiler: bool,
    pub verified: bool,
    #[serde(rename = "update")]
    pub updated: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "picurl")]
    pub picture_url: Option<String>,
}
