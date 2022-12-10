use serde::de::IntoDeserializer;
use serde::{Deserialize, Deserializer, Serialize};
use strum_macros::AsRefStr;

use crate::client::models::entrythumb::{EntryThumbContract, EntryThumbForApiContract};
use crate::client::models::pv::PVContract;
use crate::client::models::status::Status;
use crate::client::models::tag::TagUsageForApiContract;
use crate::client::models::user::{UserForApiContract, UserWithEmailContract};
use crate::client::models::weblink::WebLinkForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub enum FeaturedCategory {
    Nothing,
    Concerts,
    VocaloidRanking,
    Pools,
    Other,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
pub enum PvServices {
    Nothing,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RatedSongForUserForApiContract {
    date: String,
    song: SongForApiContract,
    user: UserForApiContract,
    rating: SongRating,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedSongsContract {
    #[serde(rename = "artistMatches")]
    artist_matches: Vec<SongForApiContract>,
    #[serde(rename = "likeMatches")]
    like_matches: Vec<SongForApiContract>,
    #[serde(rename = "tagMatches")]
    tag_matches: Vec<SongForApiContract>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    #[serde(rename = "artistString")]
    artist_string: String,
    #[serde(rename = "createDate")]
    create_date: String,
    // deleted: bool,
    #[serde(rename = "favoritedTimes")]
    favorited_times: i32,
    id: i32,
    #[serde(rename = "lengthSeconds")]
    length_seconds: i32,
    name: String,
    #[serde(rename = "nicoId")]
    nico_id: String,
    #[serde(rename = "publishDate")]
    publish_date: String,
    #[serde(rename = "pvServices")]
    pv_services: PvServices,
    #[serde(rename = "ratingScore")]
    rating_score: i32,
    #[serde(rename = "songType")]
    song_type: SongType,
    status: Status,
    #[serde(rename = "thumbUrl")]
    thumb_url: String,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SongFeaturedCategory {
    Nothing,
    Concerts,
    VocaloidRanking,
    Pools,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongForApiContract {
    pub id: i32,
    #[serde(rename = "additionalNames")]
    pub additional_names: String,
    // pub albums: Vec<AlbumContract>,
    // pub artists: Vec<ArtistForSongContract>,
    #[serde(rename = "artistString")]
    pub artist_string: String,
    // @JsonDeserialize(using = ZonedStringDeserializer.class)
    #[serde(rename = "createDate")]
    pub create_date: String,
    // #[serde(rename = "defaultName")]
    // pub default_name: String,
    // #[serde(rename = "defaultNameLanguage")]
    // pub default_name_language: Language,
    // pub deleted: bool,
    #[serde(rename = "favoritedTimes")]
    pub favorited_times: i32,
    // #[serde(rename = "lengthSeconds")]
    // pub length_seconds: i32,
    // pub lyrics: Vec<LyricsForSongContract>,
    #[serde(rename = "mainPicture")]
    pub main_picture: Option<EntryThumbForApiContract>,
    // #[serde(rename = "mergedTo")]
    // pub merged_to: i32,
    pub name: String,
    // pub names: Vec<LocalizedStringContract>,
    // #[serde(rename = "originalVersionId")]
    // pub original_version_id: Option<i32>,
    #[serde(rename = "publishDate")]
    pub publish_date: Option<String>,
    pub pvs: Vec<PVContract>,
    #[serde(
        rename = "pvServices",
        deserialize_with = "comma_separated_string_to_vec_deserializer"
    )]
    pub pv_services: Vec<PvServices>,
    #[serde(rename = "ratingScore")]
    pub rating_score: i32,
    // #[serde(rename = "releaseEvent")]
    // pub release_event: ReleaseEventForApiContract,
    #[serde(rename = "songType")]
    pub song_type: SongType,
    pub status: Status,
    pub tags: Vec<TagUsageForApiContract>,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: Option<String>,
    // pub version: i32,
    #[serde(rename = "webLinks")]
    pub web_links: Vec<WebLinkForApiContract>,
}

fn comma_separated_string_to_vec_deserializer<'de, D>(
    deserializer: D,
) -> Result<Vec<PvServices>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    return str
        .split(',')
        .map(|s| s.trim())
        .map(|s| PvServices::deserialize(String::from(s).into_deserializer()))
        .collect();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongInAlbumForApiContract {
    #[serde(rename = "discNumber")]
    disc_number: i32,
    id: i32,
    name: String,
    song: SongForApiContract,
    #[serde(rename = "trackNumber")]
    track_number: i32,
}

#[derive(Serialize, Deserialize, Debug)]
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
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SongRating {
    Nothing,
    Dislike,
    Like,
    Favorite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongRatingContract {
    rating: SongRating,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongListForEditContract {
    #[serde(rename = "songLinks")]
    song_links: Vec<SongInListEditContract>,
    #[serde(rename = "updateNotes")]
    update_notes: String,
    author: UserWithEmailContract,
    #[serde(rename = "canEdit")]
    can_edit: bool,
    // deleted: bool,
    description: String,
    #[serde(rename = "eventDate")]
    event_date: String,
    status: Status,
    thumb: EntryThumbContract,
    version: i32,
    #[serde(rename = "featuredCategory")]
    featured_category: FeaturedCategory,
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongListBaseContract {
    id: i32,
    #[serde(rename = "featuredCategory")]
    featured_category: SongFeaturedCategory,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongInListEditContract {
    #[serde(rename = "songInListId")]
    song_in_list_id: i32,
    notes: String,
    order: i32,
    song: SongForApiContract,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongInListForApiContract {
    notes: String,
    order: i32,
    song: SongForApiContract,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongListForApiContract {
    author: UserForApiContract,
    #[serde(rename = "eventDate")]
    event_date: String,
    #[serde(rename = "mainPicture")]
    main_picture: EntryThumbForApiContract,
    #[serde(rename = "featuredCategory")]
    featured_category: FeaturedCategory,
    id: i32,
    name: String,
}
