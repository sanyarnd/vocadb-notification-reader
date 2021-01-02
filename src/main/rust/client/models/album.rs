use serde::{Deserialize, Serialize};

use crate::client::models::artist::ArtistForAlbumForApiContract;
use crate::client::models::entrythumb::EntryThumbForApiContract;
use crate::client::models::language::Language;
use crate::client::models::misc::{LocalizedStringContract, OptionalDateTimeContract};
use crate::client::models::pv::PVContract;
use crate::client::models::releaseevent::ReleaseEventForApiContract;
use crate::client::models::song::SongInAlbumForApiContract;
use crate::client::models::status::Status;
use crate::client::models::tag::TagUsageForApiContract;
use crate::client::models::user::UserForApiContract;
use crate::client::models::weblink::WebLinkForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    #[serde(rename = "artistString")]
    artist_string: String,
    #[serde(rename = "coverPictureMime")]
    cover_picture_mime: String,
    #[serde(rename = "createDate")]
    create_date: String,
    // deleted: bool,
    #[serde(rename = "discType")]
    disc_type: DiscType,
    id: i32,
    name: String,
    #[serde(rename = "ratingAverage")]
    rating_average: f64,
    #[serde(rename = "ratingCount")]
    rating_count: i32,
    #[serde(rename = "releaseDate")]
    release_date: OptionalDateTimeContract,
    #[serde(rename = "releaseEvent")]
    release_event: Option<ReleaseEventForApiContract>,
    status: Status,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumDiscPropertiesContract {
    #[serde(rename = "discNumber")]
    disc_number: i32,
    id: i32,
    #[serde(rename = "mediaType")]
    media_type: DiscMediaType,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumForApiContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    artists: Vec<ArtistForAlbumForApiContract>,
    #[serde(rename = "artistString")]
    artist_string: String,
    barcode: String,
    #[serde(rename = "catalogNumber")]
    catalog_number: String,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "defaultName")]
    default_name: String,
    #[serde(rename = "defaultNameLanguage")]
    default_name_language: Language,
    // deleted: bool,
    description: String,
    discs: Vec<AlbumDiscPropertiesContract>,
    #[serde(rename = "discType")]
    disc_type: DiscType,
    id: i32,
    identifiers: Vec<AlbumIdentifierContract>,
    #[serde(rename = "mainPicture")]
    main_picture: EntryThumbForApiContract,
    #[serde(rename = "mergedTo")]
    merged_to: i32,
    name: String,
    names: Vec<LocalizedStringContract>,
    pvs: Vec<PVContract>,
    #[serde(rename = "ratingAverage")]
    rating_average: f64,
    #[serde(rename = "ratingCount")]
    rating_count: i32,
    #[serde(rename = "releaseDate")]
    release_date: OptionalDateTimeContract,
    #[serde(rename = "releaseEvent")]
    release_event: ReleaseEventForApiContract,
    status: Status,
    tags: Vec<TagUsageForApiContract>,
    tracks: Vec<SongInAlbumForApiContract>,
    version: i32,
    #[serde(rename = "webLinks")]
    web_links: Vec<WebLinkForApiContract>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumForUserForApiContract {
    album: AlbumForApiContract,
    #[serde(rename = "mediaType")]
    media_type: AlbumMediaType,
    #[serde(rename = "purchaseStatus")]
    purchase_status: AlbumPurchaseStatus,
    rating: i32,
    user: UserForApiContract,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumIdentifierContract {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AlbumMediaType {
    PhysicalDisc,
    DigitalDownload,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AlbumPurchaseStatus {
    Nothing,
    Wishlisted,
    Ordered,
    Owned,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumReviewContract {
    id: i32,
    #[serde(rename = "albumId")]
    album_id: i32,
    date: String,
    #[serde(rename = "languageCode")]
    language_code: String,
    text: String,
    title: String,
    user: UserForApiContract,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiscMediaType {
    Audio,
    Video,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiscType {
    Unknown,
    Album,
    Single,
    EP,
    SplitAlbum,
    Compilation,
    Video,
    Artbook,
    Game,
    Fanmade,
    Instrumental,
    Other,
}
