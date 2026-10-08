use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::models::{PVContract, PvService, PvType, SongType};

/// Notification as returned to the frontend; the variant name is stored in `notificationType`.
#[derive(Serialize, Debug)]
#[serde(tag = "notificationType")]
pub enum Notification {
    #[serde(rename = "SongNotification")]
    Song(SongNotification),
    #[serde(rename = "ArtistNotification")]
    Artist(BaseNotification),
    #[serde(rename = "AlbumNotification")]
    Album(BaseNotification),
    #[serde(rename = "EventNotification")]
    Event(BaseNotification),
    #[serde(rename = "ReportNotification")]
    Report(BaseNotification),
    #[serde(rename = "UnknownNotification")]
    Unknown(BaseNotification),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BaseNotification {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SongNotification {
    #[serde(flatten)]
    pub base: BaseNotification,
    #[serde(rename = "type")]
    pub song_notification_type: SongNotificationType,
    pub song_id: i32,
    pub song_type: SongType,
    pub tags: Vec<Tag>,
    pub pvs: Vec<PV>,
    pub title: String,
    pub artist: String,
    pub release_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SongNotificationType {
    Tagged,
    New,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub category_name: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PV {
    pub id: i32,
    pub pv_type: PvType,
    pub service: PvService,
    pub url: String,
    pub name: String,
    pub disabled: bool,
    pub author: Option<String>,
    pub publish_date: Option<String>,
    pub pv_id: Option<String>,
    pub thumb_url: Option<String>,
    /// Piapro embeds require the upload timestamp.
    pub timestamp: Option<String>,
}

impl From<PVContract> for PV {
    fn from(pv: PVContract) -> Self {
        let timestamp = match (pv.service, &pv.extended_metadata) {
            (PvService::Piapro, Some(metadata)) => metadata
                .json
                .get("Timestamp")
                .and_then(|t| t.as_str())
                .map(String::from),
            _ => None,
        };

        PV {
            id: pv.id,
            pv_type: pv.pv_type,
            service: pv.service,
            url: pv.url,
            name: pv.name,
            disabled: pv.disabled,
            author: pv.author,
            publish_date: pv.publish_date,
            pv_id: pv.pv_id,
            thumb_url: pv.thumb_url,
            timestamp,
        }
    }
}
