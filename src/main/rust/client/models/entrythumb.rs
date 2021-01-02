use serde::{Deserialize, Serialize};

use crate::client::models::album::DiscType;
use crate::client::models::archived::ArchivedWebLinkContract;
use crate::client::models::artist::ArtistType;
use crate::client::models::language::Language;
use crate::client::models::misc::LocalizedStringContract;
use crate::client::models::pv::PVContract;
use crate::client::models::song::{SongFeaturedCategory, SongType};
use crate::client::models::status::Status;
use crate::client::models::tag::TagUsageForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryThumbForApiContract {
    mime: Option<String>,
    #[serde(rename = "urlSmallThumb")]
    url_small_thumb: Option<String>,
    #[serde(rename = "urlThumb")]
    url_thumb: String,
    #[serde(rename = "urlTinyThumb")]
    url_tiny_thumb: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryForApiContract {
    #[serde(rename = "activityDate")]
    activity_date: String,
    #[serde(rename = "additionalNames")]
    additional_names: String,
    #[serde(rename = "artistString")]
    artist_string: String,
    #[serde(rename = "artistType")]
    artist_type: ArtistType,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "defaultName")]
    default_name: String,
    #[serde(rename = "defaultNameLanguage")]
    default_name_language: Language,
    description: String,
    #[serde(rename = "discType")]
    disc_type: DiscType,
    #[serde(rename = "entryType")]
    entry_type: EntryType,
    #[serde(rename = "eventCategory")]
    event_category: EventCategory,
    id: i32,
    #[serde(rename = "mainPicture")]
    main_picture: EntryThumbForApiContract,
    name: String,
    names: Vec<LocalizedStringContract>,
    #[serde(rename = "pVs")]
    pvs: Vec<PVContract>,
    #[serde(rename = "songListFeaturedCategory")]
    song_list_featured_category: SongFeaturedCategory,
    #[serde(rename = "songType")]
    song_type: SongType,
    status: Status,
    #[serde(rename = "releaseEventSeriesName")]
    release_event_series_name: String,
    #[serde(rename = "tagCategoryName")]
    tag_category_name: String,
    tags: Vec<TagUsageForApiContract>,
    #[serde(rename = "urlSlug")]
    url_slug: String,
    version: i32,
    #[serde(rename = "webLinks")]
    web_links: Vec<ArchivedWebLinkContract>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryThumbContract {
    #[serde(rename = "entryType")]
    entry_type: Option<EntryType>,
    id: Option<i32>,
    mime: Option<String>,
    version: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryType {
    Undefined,
    Album,
    Artist,
    DiscussionTopic,
    PV,
    ReleaseEvent,
    ReleaseEventSeries,
    Song,
    SongList,
    Tag,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventCategory {
    Unspecified,
    AlbumRelease,
    Anniversary,
    Club,
    Concert,
    Contest,
    Convention,
    Other,
}
