use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AnimeType {
    Movie,
    #[serde(rename = "Music Video")]
    MusicVideo,
    Other,
    #[serde(rename = "OVA")]
    Ova,
    #[serde(rename = "TV Series")]
    TvSeries,
    #[serde(rename = "TV Special")]
    TvSpecial,
    #[serde(rename = "unknown")]
    Unknown,
    Web,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum TitleType {
    KanaReading,
    Main,
    Official,
    Short,
    Synonym,
    TitleCard,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TitleLanguage {
    #[serde(rename(deserialize = "ja"))]
    Japanese,
    #[serde(rename(deserialize = "x-jat"))]
    Romaji,
    #[serde(rename(deserialize = "en"))]
    English,
    #[serde(rename(deserialize = "af"))]
    Afrikaans,
    #[serde(rename(deserialize = "al"))]
    Albanian,
    #[serde(rename(deserialize = "ar"))]
    Arabic,
    #[serde(rename(deserialize = "es-pv"))]
    Basque,
    #[serde(rename(deserialize = "bd"))]
    Bengali,
    #[serde(rename(deserialize = "bg"))]
    Bulgarian,
    #[serde(rename(deserialize = "bs"))]
    Bosnian,
    #[serde(rename(deserialize = "bur"))]
    MyanmarBurmese,
    #[serde(rename(deserialize = "es-ca"))]
    Catalan,
    #[serde(rename(deserialize = "x-zht"))]
    Pinyin,
    #[serde(alias = "zh")]
    #[serde(alias = "zh-cmn")]
    #[serde(alias = "zh-nan")]
    #[serde(alias = "zh-yue")]
    Chinese,
    #[serde(rename(deserialize = "zh-hant"))]
    ChineseTraditional,
    #[serde(rename(deserialize = "zh-hans"))]
    ChineseSimplified,
    #[serde(rename(deserialize = "hr"))]
    Croatian,
    #[serde(rename(deserialize = "cs"))]
    Czech,
    #[serde(rename(deserialize = "da"))]
    Danish,
    #[serde(rename(deserialize = "nl"))]
    Dutch,
    #[serde(rename(deserialize = "eo"))]
    Esperanto,
    #[serde(rename(deserialize = "et"))]
    Estonian,
    #[serde(rename(deserialize = "tl"))]
    Filipino,
    #[serde(rename(deserialize = "fi"))]
    Finnish,
    #[serde(rename(deserialize = "fr"))]
    French,
    #[serde(rename(deserialize = "es-ga"))]
    Galician,
    #[serde(rename(deserialize = "ka"))]
    Georgian,
    #[serde(rename(deserialize = "de"))]
    German,
    #[serde(alias = "el")]
    #[serde(alias = "grc")]
    Greek,
    #[serde(rename(deserialize = "ht"))]
    HaitianCreole,
    #[serde(rename(deserialize = "he"))]
    Hebrew,
    #[serde(rename(deserialize = "hi"))]
    Hindi,
    #[serde(rename(deserialize = "hu"))]
    Hungarian,
    #[serde(rename(deserialize = "is"))]
    Icelandic,
    #[serde(rename(deserialize = "id"))]
    Indonesian,
    #[serde(rename(deserialize = "it"))]
    Italian,
    #[serde(rename(deserialize = "jv"))]
    Javanese,
    #[serde(rename(deserialize = "ko"))]
    Korean,
    #[serde(rename(deserialize = "x-kot"))]
    KoreanTranscription,
    #[serde(rename(deserialize = "la"))]
    Latin,
    #[serde(rename(deserialize = "lv"))]
    Latvian,
    #[serde(rename(deserialize = "lt"))]
    Lithuanian,
    #[serde(rename(deserialize = "my"))]
    Malaysian,
    #[serde(rename(deserialize = "mn"))]
    Mongolian,
    #[serde(rename(deserialize = "ne"))]
    Nepali,
    #[serde(rename(deserialize = "no"))]
    Norwegian,
    #[serde(rename(deserialize = "fa"))]
    Persian,
    #[serde(rename(deserialize = "pl"))]
    Polish,
    #[serde(rename(deserialize = "pt"))]
    Portuguese,
    #[serde(rename(deserialize = "pt-br"))]
    BrazilianPortuguese,
    #[serde(rename(deserialize = "ro"))]
    Romanian,
    #[serde(rename(deserialize = "ru"))]
    Russian,
    #[serde(rename(deserialize = "sr"))]
    Serbian,
    #[serde(rename(deserialize = "si"))]
    Sinhala,
    #[serde(rename(deserialize = "sk"))]
    Slovak,
    #[serde(rename(deserialize = "sl"))]
    Slovenian,
    #[serde(alias = "es")]
    #[serde(alias = "es-419")]
    Spanish,
    #[serde(rename(deserialize = "sv"))]
    Swedish,
    #[serde(rename(deserialize = "ta"))]
    Tamil,
    #[serde(rename(deserialize = "tt"))]
    Tatar,
    #[serde(rename(deserialize = "te"))]
    Telugu,
    #[serde(rename(deserialize = "th"))]
    Thai,
    #[serde(rename(deserialize = "x-tht"))]
    ThaiTranscription,
    #[serde(rename(deserialize = "tr"))]
    Turkish,
    #[serde(rename(deserialize = "uk"))]
    Ukrainian,
    #[serde(rename(deserialize = "ur"))]
    Urdu,
    #[serde(rename(deserialize = "vi"))]
    Vietnamese,
    #[serde(rename(deserialize = "x-other"))]
    Other,
    #[serde(other)]
    Unknown,
}
