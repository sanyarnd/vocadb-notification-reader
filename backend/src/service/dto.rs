use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::models::pv::{PVContract, PvService, PvType};
use crate::client::models::song::SongType;

#[typetag::serde(tag = "notificationType")]
pub trait Notification {}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[typetag::serde(name = "ArtistNotification")]
impl Notification for Artist {}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[typetag::serde(name = "EventNotification")]
impl Notification for Event {}

#[derive(Serialize, Deserialize)]
pub struct Album {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[typetag::serde(name = "AlbumNotification")]
impl Notification for Album {}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[typetag::serde(name = "ReportNotification")]
impl Notification for Report {}

#[derive(Serialize, Deserialize)]
pub struct Unknown {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,
}

#[typetag::serde(name = "UnknownNotification")]
impl Notification for Unknown {}

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub id: i32,
    #[serde(rename = "originalSubject")]
    pub original_subject: String,
    #[serde(rename = "originalBody")]
    pub original_body: String,
    pub created_date: DateTime<Utc>,

    #[serde(rename = "type")]
    pub song_notification_type: SongNotificationType,
    #[serde(rename = "songId")]
    pub song_id: i32,
    #[serde(rename = "songType")]
    pub song_type: SongType,
    pub tags: Vec<Tag>,
    pub pvs: Vec<PV>,
    pub title: String,
    pub artist: String,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
}

#[typetag::serde(name = "SongNotification")]
impl Notification for Song {}

#[derive(Serialize, Deserialize, Debug)]
pub enum SongNotificationType {
    Tagged,
    New,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub count: i32,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PV {
    pub id: i32,
    #[serde(rename = "pvType")]
    pub pv_type: PvType,
    pub service: PvService,
    pub url: String,
    pub name: String,
    pub disabled: bool,
    pub author: Option<String>,
    #[serde(rename = "publishDate")]
    pub publish_date: Option<String>,
    #[serde(rename = "pvId")]
    pub pv_id: Option<String>,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: Option<String>,
    pub timestamp: Option<String>,
}

impl From<&PVContract> for PV {
    fn from(pv: &PVContract) -> Self {
        let em = &pv.extended_metadata;
        let mut timestamp = None;
        if pv.service == PvService::Piapro {
            if let Some(em) = em {
                timestamp = match em.json.get("Timestamp") {
                    None => None,
                    Some(timestamp) => {
                        let option = timestamp.as_str();
                        option.map(String::from)
                    }
                }
            }
        }

        PV {
            id: pv.id,
            pv_type: pv.pv_type,
            service: pv.service,
            url: pv.url.clone(),
            name: pv.name.clone(),
            disabled: pv.disabled,
            author: pv.author.clone(),
            publish_date: pv.publish_date.clone(),
            pv_id: pv.pv_id.clone(),
            thumb_url: pv.thumb_url.clone(),
            timestamp,
        }
    }
}
