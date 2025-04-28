use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct CharacterList {
    #[serde(rename = "character")]
    pub list: Vec<Character>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Character {
    #[serde(rename = "id")]
    pub character_id: String,
    #[serde(rename = "type")]
    pub cast_type: Option<CharacterCastType>,
    #[serde(rename = "update")]
    pub updated: Option<String>,
    pub rating: Option<CharacterRating>,
    pub name: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "charactertype")]
    pub character_type: Option<String>,
    pub description: Option<String>,
    pub picture: Option<String>,
    pub seiyuu: Option<Seiyuu>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CharacterRating {
    pub votes: u32,
    #[serde(rename(deserialize = "$value"))]
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Seiyuu {
    #[serde(rename = "id")]
    pub creator_id: String,
    pub picture: Option<String>,
    #[serde(rename(deserialize = "$value"))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum CharacterCastType {
    #[serde(rename = "main character in")]
    Character,
    #[serde(rename = "secondary cast in")]
    Secondary,
    #[serde(rename = "appears in")]
    AppearsIn,
    #[serde(rename = "cameo appearance in")]
    Cameo,
}
