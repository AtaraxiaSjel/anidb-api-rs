use super::{
    AnimeRelated, AnimeRelatedList, AnimeSimilar, AnimeSimilarList, Character, CharacterList,
    Creator, CreatorList, Episode, EpisodeList, Recommendation, RecommendationList, Resource,
    ResourceList, Tag, TagList,
};
use crate::http::models::common::{AnimeType, Ratings, Title, TitleList};
use serde::{Deserialize, Serialize};

// !TODO: Serialization
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(from = "FromAnime")]
// #[serde(into = "FromAnime")]
/// Main data type for `get_anime()` response
pub struct Anime {
    pub anime_id: String,
    pub restricted: bool,
    pub anime_type: AnimeType,
    pub episode_count: u32,
    pub start_date: Option<String>, // date
    pub end_date: Option<String>,   // date
    pub titles: Vec<Title>,
    pub related_animes: Vec<AnimeRelated>,
    pub similar_animes: Vec<AnimeSimilar>,
    pub recommendations: Vec<Recommendation>,
    pub url: Option<String>,
    pub creators: Vec<Creator>,
    pub description: Option<String>,
    pub ratings: Option<Ratings>,
    pub picture: Option<String>,
    pub resources: Vec<Resource>,
    pub tags: Vec<Tag>,
    pub characters: Vec<Character>,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct FromAnime {
    #[serde(rename = "id")]
    pub anime_id: String,
    pub restricted: bool,
    #[serde(rename = "type")]
    pub anime_type: AnimeType,
    #[serde(rename = "episodecount")]
    pub episode_count: u32,
    #[serde(rename = "startdate")]
    pub start_date: Option<String>,
    #[serde(rename = "enddate")]
    pub end_date: Option<String>,
    pub titles: Option<TitleList>,
    #[serde(rename = "relatedanime")]
    pub related_animes: Option<AnimeRelatedList>,
    #[serde(rename = "similaranime")]
    pub similar_animes: Option<AnimeSimilarList>,
    pub recommendations: Option<RecommendationList>,
    pub url: Option<String>,
    pub creators: Option<CreatorList>,
    pub description: Option<String>,
    #[serde(rename = "ratings")]
    pub ratings: Option<Ratings>,
    pub picture: Option<String>,
    pub resources: Option<ResourceList>,
    pub tags: Option<TagList>,
    pub characters: Option<CharacterList>,
    pub episodes: Option<EpisodeList>,
}

impl From<FromAnime> for Anime {
    fn from(value: FromAnime) -> Self {
        Self {
            anime_id: value.anime_id,
            restricted: value.restricted,
            anime_type: value.anime_type,
            episode_count: value.episode_count,
            start_date: value.start_date,
            end_date: value.end_date,
            titles: value.titles.map_or_else(Vec::new, |x| x.list),
            related_animes: value.related_animes.map_or_else(Vec::new, |x| x.list),
            similar_animes: value.similar_animes.map_or_else(Vec::new, |x| x.list),
            recommendations: value.recommendations.map_or_else(Vec::new, |x| x.list),
            url: value.url,
            creators: value.creators.map_or_else(Vec::new, |x| x.list),
            description: value.description,
            ratings: value.ratings,
            picture: value.picture,
            resources: value.resources.map_or_else(Vec::new, |x| x.list),
            tags: value.tags.map_or_else(Vec::new, |x| x.list),
            characters: value.characters.map_or_else(Vec::new, |x| x.list),
            episodes: value.episodes.map_or_else(Vec::new, |x| x.list),
        }
    }
}

impl From<Anime> for FromAnime {
    fn from(value: Anime) -> Self {
        FromAnime {
            anime_id: value.anime_id,
            restricted: value.restricted,
            anime_type: value.anime_type,
            episode_count: value.episode_count,
            start_date: value.start_date,
            end_date: value.end_date,
            titles: if value.titles.is_empty() {
                None
            } else {
                Some(TitleList { list: value.titles })
            },
            related_animes: if value.related_animes.is_empty() {
                None
            } else {
                Some(AnimeRelatedList {
                    list: value.related_animes,
                })
            },
            similar_animes: if value.similar_animes.is_empty() {
                None
            } else {
                Some(AnimeSimilarList {
                    list: value.similar_animes,
                })
            },
            recommendations: if value.recommendations.is_empty() {
                None
            } else {
                let total = value.recommendations.len();
                Some(RecommendationList {
                    list: value.recommendations,
                    total: total.to_string(),
                })
            },
            url: value.url,
            creators: if value.creators.is_empty() {
                None
            } else {
                Some(CreatorList {
                    list: value.creators,
                })
            },
            description: value.description,
            ratings: value.ratings,
            picture: value.picture,
            resources: if value.resources.is_empty() {
                None
            } else {
                Some(ResourceList {
                    list: value.resources,
                })
            },
            tags: if value.tags.is_empty() {
                None
            } else {
                Some(TagList { list: value.tags })
            },
            characters: if value.characters.is_empty() {
                None
            } else {
                Some(CharacterList {
                    list: value.characters,
                })
            },
            episodes: if value.episodes.is_empty() {
                None
            } else {
                Some(EpisodeList {
                    list: value.episodes,
                })
            },
        }
    }
}

#[cfg(test)]
mod test_anime {
    use super::*;
    use crate::http::models::{
        anime::{
            AnimeRelationType, CharacterCastType, CharacterRating, EpisodeType, ExternalEntity,
            RecommendationType, ResourceType, Seiyuu,
        },
        common::{AnimeRating, TitleType},
    };

    const TEST_XML: &str = r#"<?xml version="1.0"?>
<anime id="1" restricted="false">
    <type>TV Series</type>
    <episodecount>13</episodecount>
    <startdate>1999-01-03</startdate>
    <enddate>1999-03-28</enddate>
    <titles>
        <title xml:lang="zh-Hans" type="synonym">星界之纹章</title>
        <title xml:lang="en" type="short">CotS</title>
    </titles>
    <relatedanime>
        <anime id="4" type="Sequel">Seikai no Senki</anime>
        <anime id="6" type="Prequel">Seikai no Danshou: Tanjou</anime>
    </relatedanime>
    <similaranime>
        <anime id="584" approval="75" total="89">Ginga Eiyuu Densetsu</anime>
        <anime id="2745" approval="52" total="62">Starship Operators</anime>
    </similaranime>
        <recommendations total="20">
        <recommendation type="Must See" uid="125868">This is the second best anime of all time.</recommendation>
        <recommendation type="Must See" uid="691547">An awesome space opera</recommendation>
    </recommendations>
    <url>http://www.sunrise-inc.co.jp/seikai/</url>
    <creators>
        <name id="4303" type="Music">Hattori Katsuhisa</name>
        <name id="4234" type="Direction">Nagaoka Yasuchika</name>
    </creators>
    <description>Based on the sci-fi novel series</description>
    <ratings>
        <permanent count="4430">8.16</permanent>
        <temporary count="4460">8.23</temporary>
    </ratings>
    <picture>440.jpg</picture>
    <resources>
        <resource type="1">
            <externalentity>
                <identifier>14</identifier>
            </externalentity>
            <externalentity>
                <identifier>376</identifier>
                <identifier>xzudnt</identifier>
            </externalentity>
        </resource>
        <resource type="4">
            <externalentity>
                <url>http://www.sunrise-inc.co.jp/seikai/</url>
            </externalentity>
        </resource>
    </resources>
        <tags>
        <tag id="36" parentid="2607" weight="300" localspoiler="false" globalspoiler="false"
            verified="true" update="2018-01-21">
            <name>military</name>
            <description>The military, ...</description>
            <picurl>212184.jpg</picurl>
        </tag>
    </tags>
    <characters>
        <character id="28" type="main character in" update="2012-07-25">
            <rating votes="1196">9.15</rating>
            <name>Abriel Nei Debrusc Borl Paryun Lafiel</name>
            <gender>female</gender>
            <charactertype id="1">Character</charactertype>
            <description>Viscountess Paryunu Abriel Nei Dobrusk Lafiel"</description>
            <picture>14304.jpg</picture>
            <seiyuu id="12" picture="184301.jpg">Kawasumi Ayako</seiyuu>
        </character>
        <character id="7501" type="appears in" update="2014-02-15">
            <rating votes="1">3.80</rating>
            <name>Wakusei Martine</name>
            <gender>unknown</gender>
            <charactertype id="3">Organization</charactertype>
            <description>The homeworld of Jinto</description>
            <picture>150280.jpg</picture>
        </character>
    </characters>
    <episodes>
        <episode id="1" update="2011-07-01">
            <length>25</length>
            <airdate>1999-01-03</airdate>
            <rating votes="28">3.31</rating>
            <title xml:lang="ja">侵略</title>
            <title xml:lang="en">Invasion</title>
            <title xml:lang="fr">Invasion</title>
            <title xml:lang="x-jat">Shinryaku</title>
        </episode>
        <episode id="1012" update="2011-07-01">
            <epno type="1">3</epno>
            <length>25</length>
            <airdate>1999-01-17</airdate>
            <rating votes="19">7.31</rating>
            <title xml:lang="ja">愛の娘</title>
            <title xml:lang="en">Daughter of Love</title>
            <title xml:lang="fr">La fille d`Amour</title>
            <title xml:lang="x-jat">Ai no Musume</title>
        </episode>
    </episodes>
</anime>
</xml>"#;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_anime() {
        let should_be = Anime {
            anime_id: 1.to_string(),
            restricted: false,
            anime_type: AnimeType::TvSeries,
            episode_count: 13,
            start_date: Some("1999-01-03".to_string()),
            end_date: Some("1999-03-28".to_string()),
            titles: vec![
                Title {
                    lang: Some("zh-Hans".to_string()),
                    title_type: Some(TitleType::Synonym),
                    name: "星界之纹章".to_string(),
                },
                Title {
                    lang: Some("en".to_string()),
                    title_type: Some(TitleType::Short),
                    name: "CotS".to_string(),
                },
            ],
            related_animes: vec![
                AnimeRelated {
                    anime_id: 4.to_string(),
                    anime_type: AnimeRelationType::Sequel,
                    name: "Seikai no Senki".to_string(),
                },
                AnimeRelated {
                    anime_id: 6.to_string(),
                    anime_type: AnimeRelationType::Prequel,
                    name: "Seikai no Danshou: Tanjou".to_string(),
                },
            ],
            similar_animes: vec![
                AnimeSimilar {
                    anime_id: 584.to_string(),
                    approval: 75,
                    total: 89,
                    name: "Ginga Eiyuu Densetsu".to_string(),
                },
                AnimeSimilar {
                    anime_id: 2745.to_string(),
                    approval: 52,
                    total: 62,
                    name: "Starship Operators".to_string(),
                },
            ],
            recommendations: vec![
                Recommendation {
                    recommendation_type: RecommendationType::MustSee,
                    user_id: "125868".to_string(),
                    text: "This is the second best anime of all time.".to_string(),
                },
                Recommendation {
                    recommendation_type: RecommendationType::MustSee,
                    user_id: "691547".to_string(),
                    text: "An awesome space opera".to_string(),
                },
            ],
            url: Some("http://www.sunrise-inc.co.jp/seikai/".to_string()),
            creators: vec![
                Creator {
                    creator_id: "4303".to_string(),
                    creator_type: "Music".to_string(),
                    name: "Hattori Katsuhisa".to_string(),
                },
                Creator {
                    creator_id: "4234".to_string(),
                    creator_type: "Direction".to_string(),
                    name: "Nagaoka Yasuchika".to_string(),
                },
            ],
            description: Some("Based on the sci-fi novel series".to_string()),
            ratings: Some(Ratings {
                permanent: Some(AnimeRating {
                    votes: 4430,
                    value: 8.16,
                }),
                temporary: Some(AnimeRating {
                    votes: 4460,
                    value: 8.23,
                }),
                review: None,
            }),
            picture: Some("440.jpg".to_string()),
            resources: vec![
                Resource {
                    resource_type: ResourceType::AnimeNewsNetwork,
                    external_entity: vec![
                        ExternalEntity {
                            identifiers: Some(vec!["14".to_string()]),
                            url: None,
                        },
                        ExternalEntity {
                            identifiers: Some(vec!["376".to_string(), "xzudnt".to_string()]),
                            url: None,
                        },
                    ],
                },
                Resource {
                    resource_type: ResourceType::OfficialWebsite,
                    external_entity: vec![ExternalEntity {
                        identifiers: None,
                        url: Some("http://www.sunrise-inc.co.jp/seikai/".to_string()),
                    }],
                },
            ],
            tags: vec![Tag {
                tag_id: "36".to_string(),
                parent_id: Some("2607".to_string()),
                weight: 300,
                local_spoiler: false,
                global_spoiler: false,
                verified: true,
                updated: Some("2018-01-21".to_string()),
                name: Some("military".to_string()),
                description: Some("The military, ...".to_string()),
                picture_url: Some("212184.jpg".to_string()),
            }],
            characters: vec![
                Character {
                    character_id: "28".to_string(),
                    cast_type: Some(CharacterCastType::Character),
                    updated: Some("2012-07-25".to_string()),
                    rating: Some(CharacterRating {
                        votes: 1196,
                        value: 9.15,
                    }),
                    name: Some("Abriel Nei Debrusc Borl Paryun Lafiel".to_string()),
                    gender: Some("female".to_string()),
                    character_type: Some("Character".to_string()),
                    description: Some(
                        "Viscountess Paryunu Abriel Nei Dobrusk Lafiel\"".to_string(),
                    ),
                    picture: Some("14304.jpg".to_string()),
                    seiyuu: Some(Seiyuu {
                        creator_id: "12".to_string(),
                        picture: Some("184301.jpg".to_string()),
                        name: Some("Kawasumi Ayako".to_string()),
                    }),
                },
                Character {
                    character_id: "7501".to_string(),
                    cast_type: Some(CharacterCastType::AppearsIn),
                    updated: Some("2014-02-15".to_string()),
                    rating: Some(CharacterRating {
                        votes: 1,
                        value: 3.8,
                    }),
                    name: Some("Wakusei Martine".to_string()),
                    gender: Some("unknown".to_string()),
                    character_type: Some("Organization".to_string()),
                    description: Some("The homeworld of Jinto".to_string()),
                    picture: Some("150280.jpg".to_string()),
                    seiyuu: None,
                },
            ],
            episodes: vec![
                Episode {
                    episode_id: "1".to_string(),
                    episode_num: None,
                    episode_type: None,
                    length: Some("25".to_string()),
                    updated: Some("2011-07-01".to_string()),
                    titles: vec![
                        Title {
                            lang: Some("ja".to_string()),
                            title_type: None,
                            name: "侵略".to_string(),
                        },
                        Title {
                            lang: Some("en".to_string()),
                            title_type: None,
                            name: "Invasion".to_string(),
                        },
                        Title {
                            lang: Some("fr".to_string()),
                            title_type: None,
                            name: "Invasion".to_string(),
                        },
                        Title {
                            lang: Some("x-jat".to_string()),
                            title_type: None,
                            name: "Shinryaku".to_string(),
                        },
                    ],
                    description: None,
                    air_date: Some("1999-01-03".to_string()),
                },
                Episode {
                    episode_id: "1012".to_string(),
                    episode_num: Some("3".to_string()),
                    episode_type: Some(EpisodeType::Episode),
                    length: Some("25".to_string()),
                    updated: Some("2011-07-01".to_string()),
                    titles: vec![
                        Title {
                            lang: Some("ja".to_string()),
                            title_type: None,
                            name: "愛の娘".to_string(),
                        },
                        Title {
                            lang: Some("en".to_string()),
                            title_type: None,
                            name: "Daughter of Love".to_string(),
                        },
                        Title {
                            lang: Some("fr".to_string()),
                            title_type: None,
                            name: "La fille d`Amour".to_string(),
                        },
                        Title {
                            lang: Some("x-jat".to_string()),
                            title_type: None,
                            name: "Ai no Musume".to_string(),
                        },
                    ],
                    description: None,
                    air_date: Some("1999-01-17".to_string()),
                },
            ],
        };
        let anime: Anime = serde_xml_rs::from_str(TEST_XML).unwrap();
        assert_eq!(anime, should_be);
    }
}
