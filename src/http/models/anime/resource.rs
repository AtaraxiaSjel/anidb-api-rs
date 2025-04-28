use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct ResourceList {
    #[serde(rename = "resource")]
    pub list: Vec<Resource>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Resource {
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    #[serde(rename = "externalentity")]
    pub external_entity: Vec<ExternalEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ExternalEntity {
    #[serde(rename = "identifier")]
    pub identifiers: Option<Vec<String>>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ResourceType {
    #[serde(rename(deserialize = "11"))]
    DotLain = 11,
    #[serde(rename(deserialize = "9"))]
    Allcinema = 9,
    #[serde(rename(deserialize = "48"))]
    AmazonPrimeVideo = 48,
    #[serde(rename(deserialize = "32"))]
    AmazonVideo = 32,
    #[serde(rename(deserialize = "16"))]
    Animemorial = 16,
    #[serde(rename(deserialize = "1"))]
    AnimeNewsNetwork = 1,
    #[serde(rename(deserialize = "3"))]
    AnimeNFO = 3,
    #[serde(rename(deserialize = "10"))]
    Anison = 10,
    #[serde(rename(deserialize = "33"))]
    BaiduBaike = 33,
    #[serde(rename(deserialize = "38"))]
    Bangumi = 38,
    #[serde(rename(deserialize = "47"))]
    BiliBili = 47,
    #[serde(rename(deserialize = "20"))]
    ChineseWikipedia = 20,
    #[serde(rename(deserialize = "28"))]
    Crunchyroll = 28,
    #[serde(rename(deserialize = "39"))]
    Douban = 39,
    #[serde(rename(deserialize = "6"))]
    EnglishWikipedia = 6,
    #[serde(rename(deserialize = "22"))]
    Facebook = 22,
    #[serde(rename(deserialize = "45"))]
    Funimation = 45,
    #[serde(rename(deserialize = "42"))]
    HiDive = 42,
    #[serde(rename(deserialize = "43"))]
    IMDB = 43,
    #[serde(rename(deserialize = "7"))]
    JapaneseWikipedia = 7,
    #[serde(rename(deserialize = "19"))]
    KoreanWikipedia = 19,
    #[serde(rename(deserialize = "15"))]
    Marumegane = 15,
    #[serde(rename(deserialize = "31"))]
    MediaArtDatabase = 31,
    #[serde(rename(deserialize = "2"))]
    MyAnimeList = 2,
    #[serde(rename(deserialize = "41"))]
    Netflix = 41,
    #[serde(rename(deserialize = "35"))]
    OfficialBlog = 35,
    #[serde(rename(deserialize = "5"))]
    OfficialEnglishWebsite = 5,
    #[serde(rename(deserialize = "34"))]
    OfficialStream = 34,
    #[serde(rename(deserialize = "4"))]
    OfficialWebsite = 4,
    #[serde(rename(deserialize = "46"))]
    QQ = 46,
    #[serde(rename(deserialize = "8"))]
    Syoboi = 8,
    #[serde(rename(deserialize = "44"))]
    TMDB = 44,
    #[serde(rename(deserialize = "17"))]
    TvAnimationMuseum = 17,
    #[serde(rename(deserialize = "23"))]
    Twitter = 23,
    #[serde(rename(deserialize = "14"))]
    VisualNovelDatabase = 14,
    #[serde(rename(deserialize = "26"))]
    YouTube = 26,
}
