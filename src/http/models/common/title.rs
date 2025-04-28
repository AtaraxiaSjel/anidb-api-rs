use super::TitleType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct TitleList {
    #[serde(rename = "title")]
    pub list: Vec<Title>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Title {
    #[serde(rename = "lang")]
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(rename = "type")]
    pub title_type: Option<TitleType>,
    #[serde(rename(deserialize = "$value"))]
    #[serde(default)]
    pub name: String,
}
