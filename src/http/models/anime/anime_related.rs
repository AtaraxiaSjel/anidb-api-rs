use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct AnimeRelatedList {
    #[serde(rename = "anime")]
    pub list: Vec<AnimeRelated>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnimeRelated {
    #[serde(rename = "id")]
    pub anime_id: String,
    #[serde(rename = "type")]
    pub anime_type: AnimeRelationType,
    #[serde(rename(deserialize = "$value"))]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AnimeRelationType {
    /// Story that happens before the original.
    Prequel,
    /// Same setting, same characters, story is told differently.
    #[serde(rename = "Alternative Version")]
    AlternativeVersion,
    /// Shares one or more characters, story is unrelated.
    Character,
    /// Unspecified relation.
    Other,
    /// Same characters, different universe/world/reality/timeline.
    #[serde(rename = "Alternative Setting")]
    AlternativeSetting,
    /// Full version of the summarised story.
    #[serde(rename = "Full Story")]
    FullStory,
    /// Summarises full story, may contain additional stuff.
    Summary,
    /// Same universe/world/reality/timeline, completely different characters.
    #[serde(rename = "Same Setting")]
    SameSetting,
    /// Takes place sometime during the parent storyline.
    #[serde(rename = "Side Story")]
    SideStory,
    /// Parent story for another story that takes place sometime during the same time.
    #[serde(rename = "Parent Story")]
    ParentStory,
    /// Direct continuation of the story.
    Sequel,
}
