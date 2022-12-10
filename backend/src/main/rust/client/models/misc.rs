use serde::{Deserialize, Serialize};

use crate::client::models::language::Language;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedSearchFilter {
    #[serde(rename = "filterType")]
    filter_type: FilterType,
    negate: bool,
    param: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterType {
    Nothing,
    ArtistType,
    WebLink,
    HasUserAccount,
    RootVoicebank,
    VoiceProvider,
    HasStoreLink,
    HasTracks,
    NoCoverPicture,
    HasAlbum,
    HasOriginalMedia,
    HasMedia,
    HasMultipleVoicebanks,
    HasPublishDate,
    Lyrics,
    LyricsContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalizedStringContract {
    language: Language,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LyricsForSongContract {
    #[serde(rename = "cultureCode")]
    culture_code: String,
    id: i32,
    source: String,
    #[serde(rename = "translationType")]
    translation_type: TranslationType,
    url: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldUsernameContract {
    date: String,
    #[serde(rename = "oldName")]
    old_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionalDateTimeContract {
    day: i32,
    formatted: String,
    #[serde(rename = "isEmpty")]
    is_empty: bool,
    month: i32,
    year: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialFindResult<T> {
    pub items: Vec<T>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    pub term: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TranslationType {
    Original,
    Romanized,
    Translation,
}
