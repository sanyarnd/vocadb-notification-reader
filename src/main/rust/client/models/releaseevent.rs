use serde::{Deserialize, Serialize};

use crate::client::models::artist::ArtistForEventContract;
use crate::client::models::entrythumb::EntryThumbForApiContract;
use crate::client::models::misc::LocalizedStringContract;
use crate::client::models::song::SongListBaseContract;
use crate::client::models::status::Status;
use crate::client::models::weblink::{WebLinkContract, WebLinkForApiContract};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReleaseEventCategory {
    Unspecified,
    AlbumRelease,
    Anniversary,
    Club,
    Concert,
    Contest,
    Convention,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseEventForApiContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    artists: Vec<ArtistForEventContract>,
    category: ReleaseEventCategory,
    date: String,
    description: String,
    #[serde(rename = "endDate")]
    end_date: String,
    id: i32,
    #[serde(rename = "mainPicture")]
    main_picture: EntryThumbForApiContract,
    name: String,
    names: Vec<LocalizedStringContract>,
    series: ReleaseEventSeriesContract,
    #[serde(rename = "seriesId")]
    series_id: i32,
    #[serde(rename = "seriesNumber")]
    series_number: i32,
    #[serde(rename = "seriesSuffix")]
    series_suffix: String,
    #[serde(rename = "songList")]
    song_list: SongListBaseContract,
    status: Status,
    #[serde(rename = "urlSlug")]
    url_slug: String,
    #[serde(rename = "venueName")]
    venue_name: String,
    version: i32,
    #[serde(rename = "webLinks")]
    web_links: Vec<WebLinkForApiContract>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseEventSeriesContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    category: ReleaseEventCategory,
    // deleted: bool,
    description: String,
    id: i32,
    name: String,
    #[serde(rename = "pictureMime")]
    picture_mime: String,
    status: Status,
    #[serde(rename = "urlSlug")]
    url_slug: String,
    version: i32,
    #[serde(rename = "webLinks")]
    web_links: Vec<WebLinkContract>,
}
