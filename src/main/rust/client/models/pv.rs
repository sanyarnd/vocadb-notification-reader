use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use strum_macros::ToString;

use crate::client::models::song::SongContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct PVContract {
    pub id: i32,
    #[serde(rename = "pvType")]
    pub pv_type: PvType,
    pub service: PvService,
    pub url: String,
    pub name: String,
    pub disabled: bool,
    pub author: Option<String>,
    #[serde(rename = "extendedMetadata")]
    pub extended_metadata: Option<PVExtendedMetadata>,
    pub length: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "publishDate")]
    pub publish_date: Option<String>,
    #[serde(rename = "pvId")]
    pub pv_id: Option<String>,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PVExtendedMetadata {
    #[serde(deserialize_with = "raw_json_to_map_deserializer")]
    pub json: HashMap<String, Value>,
}

fn raw_json_to_map_deserializer<'de, D>(deserializer: D) -> Result<HashMap<String, Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    let json_map = serde_json::from_str(&str);
    return match json_map {
        Ok(map) => Ok(map),
        Err(_) => Err(serde::de::Error::custom("Unable to parse raw JSON map")),
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PVForSongContract {
    song: SongContract,
    author: String,
    #[serde(rename = "createdBy")]
    created_by: Option<i32>,
    disabled: bool,
    #[serde(rename = "extendedMetadata")]
    extended_metadata: PVExtendedMetadata,
    id: i32,
    length: i32,
    name: String,
    #[serde(rename = "publishDate")]
    publish_date: String,
    #[serde(rename = "pvId")]
    pv_id: String,
    service: PvService,
    #[serde(rename = "pvType")]
    pv_type: PvType,
    #[serde(rename = "thumbUrl")]
    thumb_url: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, ToString, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Copy, ToString, Debug)]
pub enum PvType {
    Original,
    Reprint,
    Other,
}
