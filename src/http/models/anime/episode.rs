use crate::http::models::common::{ApiError, Title};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(from = "FromEpisode")]
// #[serde(into = "FromEpisode")]
pub struct Episode {
    pub episode_id: String,
    pub episode_num: Option<String>,
    pub episode_type: Option<EpisodeType>,
    pub length: Option<String>,
    pub updated: Option<String>,
    pub titles: Vec<Title>,
    pub description: Option<String>,
    pub air_date: Option<String>, // date
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EpisodeType {
    #[serde(rename(deserialize = "1"))]
    Episode = 1,
    #[serde(rename(deserialize = "2"))]
    Credits = 2,
    #[serde(rename(deserialize = "3"))]
    Special = 3,
    #[serde(rename(deserialize = "4"))]
    Trailer = 4,
    #[serde(rename(deserialize = "5"))]
    Parody = 5,
    #[serde(rename(deserialize = "6"))]
    Other = 6,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct EpisodeList {
    #[serde(rename = "episode")]
    pub list: Vec<Episode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct FromEpisode {
    #[serde(rename = "id")]
    pub episode_id: String,
    pub epno: Option<EpisodeNumber>,
    pub length: Option<String>,
    #[serde(rename = "update")]
    pub updated: Option<String>,
    #[serde(rename = "title")]
    pub titles: Option<Vec<Title>>,
    pub description: Option<String>,
    #[serde(rename = "airdate")]
    pub air_date: Option<String>, // date
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct EpisodeNumber {
    #[serde(rename(deserialize = "$value"))]
    pub episode_num: Option<String>,
    #[serde(rename = "type")]
    pub episode_type: Option<String>,
}

impl From<FromEpisode> for Episode {
    fn from(value: FromEpisode) -> Self {
        let (episode_num, episode_type) = value
            .epno
            .map_or_else(|| (None, None), |x| (x.episode_num, x.episode_type));
        Self {
            episode_id: value.episode_id,
            episode_num,
            episode_type: episode_type.map(|x| EpisodeType::from_str(&x).unwrap()),
            length: value.length,
            updated: value.updated,
            titles: value.titles.unwrap_or_default(),
            description: value.description,
            air_date: value.air_date,
        }
    }
}

impl From<Episode> for FromEpisode {
    fn from(value: Episode) -> Self {
        let epno = if value.episode_num.is_none() && value.episode_type.is_none() {
            None
        } else {
            Some(EpisodeNumber {
                episode_num: value.episode_num,
                episode_type: value.episode_type.map(|x| x.to_string()),
            })
        };

        Self {
            episode_id: value.episode_id,
            epno,
            length: value.length,
            updated: value.updated,
            titles: if value.titles.is_empty() {
                None
            } else {
                Some(value.titles)
            },
            description: value.description,
            air_date: value.air_date,
        }
    }
}

impl FromStr for EpisodeType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(EpisodeType::Episode),
            "2" => Ok(EpisodeType::Credits),
            "3" => Ok(EpisodeType::Special),
            "4" => Ok(EpisodeType::Trailer),
            "5" => Ok(EpisodeType::Parody),
            "6" => Ok(EpisodeType::Other),
            _ => Err(ApiError::Parse(
                "Error parse &str to EpisodeType".to_string(),
            )),
        }
    }
}

impl std::fmt::Display for EpisodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
