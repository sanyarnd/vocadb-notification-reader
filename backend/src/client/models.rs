//! Subset of the VocaDB API contracts used by the application.
//!
//! Only the fields the service actually reads are declared; everything else
//! in the upstream payload is ignored, so new upstream fields don't break us.

use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use strum::AsRefStr;
use ts_rs::TS;

#[derive(Serialize, Deserialize, AsRefStr, Clone, Copy, PartialEq, Eq, Debug, TS)]
#[ts(export)]
pub enum LanguagePreference {
    Default,
    Japanese,
    Romaji,
    English,
}

#[derive(AsRefStr, Clone, Copy, Debug)]
pub enum OptionalFields {
    AdditionalNames,
    MainPicture,
    Names,
    PVs,
    Tags,
    ThumbUrl,
    WebLinks,
}

#[derive(AsRefStr, Clone, Copy, Debug)]
pub enum Inbox {
    Notifications,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialFindResult<T> {
    pub items: Vec<T>,
    pub total_count: i32,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UserForApiContract {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub member_since: String,
    pub verified_artist: bool,
    pub group_id: String,
    pub main_picture: Option<EntryThumbForApiContract>,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct EntryThumbForApiContract {
    pub mime: Option<String>,
    pub url_small_thumb: Option<String>,
    pub url_thumb: Option<String>,
    pub url_tiny_thumb: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserMessageContract {
    pub id: i32,
    pub subject: String,
    pub body: String,
    #[serde(deserialize_with = "formatted_string_to_date")]
    pub created_formatted: DateTime<Utc>,
}

fn formatted_string_to_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    parse_formatted_date(&date_str)
        .ok_or_else(|| serde::de::Error::custom(format!("Unknown date format: {date_str}")))
}

/// VocaDB formats dates according to the user's culture, e.g.
/// `11.02.2018 11:19` or `2/25/2022 2:29 PM`.
pub fn parse_formatted_date(value: &str) -> Option<DateTime<Utc>> {
    const FORMATS: [&str; 2] = ["%d.%m.%Y %H:%M", "%m/%d/%Y %I:%M %p"];

    FORMATS
        .iter()
        .find_map(|format| NaiveDateTime::parse_from_str(value, format).ok())
        .map(|date| date.and_utc())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SongForApiContract {
    pub id: i32,
    pub name: String,
    pub artist_string: String,
    pub publish_date: Option<String>,
    pub song_type: SongType,
    #[serde(default)]
    pub pvs: Vec<PVContract>,
    #[serde(default)]
    pub tags: Vec<TagUsageForApiContract>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, TS)]
#[ts(export)]
pub enum SongType {
    Unspecified,
    Original,
    Remaster,
    Remix,
    Cover,
    Arrangement,
    Instrumental,
    Mashup,
    MusicPV,
    DramaPV,
    Live,
    Illustration,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagUsageForApiContract {
    pub count: i32,
    pub tag: TagBaseContract,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagBaseContract {
    pub id: i32,
    pub name: String,
    pub category_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PVContract {
    pub id: i32,
    pub pv_type: PvType,
    pub service: PvService,
    pub url: String,
    pub name: String,
    pub disabled: bool,
    pub author: Option<String>,
    #[serde(default, deserialize_with = "lenient_extended_metadata")]
    pub extended_metadata: Option<PVExtendedMetadata>,
    pub publish_date: Option<String>,
    pub pv_id: Option<String>,
    pub thumb_url: Option<String>,
}

/// Extended metadata comes as a JSON document serialized into a string.
#[derive(Debug, Default)]
pub struct PVExtendedMetadata {
    pub json: HashMap<String, Value>,
}

fn lenient_extended_metadata<'de, D>(
    deserializer: D,
) -> Result<Option<PVExtendedMetadata>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Raw {
        json: Option<String>,
    }

    let raw = Option::<Raw>::deserialize(deserializer)?;
    Ok(raw.map(|raw| PVExtendedMetadata {
        json: raw
            .json
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default(),
    }))
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, AsRefStr, Debug, TS)]
#[ts(export)]
pub enum PvService {
    NicoNicoDouga,
    Youtube,
    SoundCloud,
    Vimeo,
    Piapro,
    Bilibili,
    File,
    LocalFile,
    Creofuga,
    Bandcamp,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, AsRefStr, Debug, TS)]
#[ts(export)]
pub enum PvType {
    Original,
    Reprint,
    Other,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn parses_both_date_formats() {
        assert_eq!(
            parse_formatted_date("11.02.2018 11:19"),
            Some(Utc.with_ymd_and_hms(2018, 2, 11, 11, 19, 0).unwrap())
        );
        assert_eq!(
            parse_formatted_date("2/25/2022 2:29 PM"),
            Some(Utc.with_ymd_and_hms(2022, 2, 25, 14, 29, 0).unwrap())
        );
        assert_eq!(parse_formatted_date("2022-02-25"), None);
    }

    #[test]
    fn deserializes_pv_with_extended_metadata() {
        let pv: PVContract = serde_json::from_value(serde_json::json!({
            "id": 1,
            "pvType": "Original",
            "service": "Piapro",
            "url": "https://piapro.jp/t/abcd",
            "name": "name",
            "disabled": false,
            "length": 120,
            "extendedMetadata": { "json": "{\"Timestamp\":\"20220101000000\"}" }
        }))
        .unwrap();

        let metadata = pv.extended_metadata.unwrap();
        assert_eq!(metadata.json["Timestamp"], "20220101000000");
    }

    #[test]
    fn tolerates_broken_extended_metadata() {
        let pv: PVContract = serde_json::from_value(serde_json::json!({
            "id": 1,
            "pvType": "Original",
            "service": "Youtube",
            "url": "https://youtu.be/x",
            "name": "name",
            "disabled": false,
            "extendedMetadata": { "json": "not a json" }
        }))
        .unwrap();

        assert!(pv.extended_metadata.unwrap().json.is_empty());
    }

    #[test]
    fn unknown_song_type_falls_back_to_other() {
        let song_type: SongType = serde_json::from_str("\"SomethingNew\"").unwrap();
        assert_eq!(song_type, SongType::Other);
    }
}
