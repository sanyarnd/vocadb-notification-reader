use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum ArtistParticipationStatus {
    Everything,
    OnlyMainAlbums,
    OnlyCollaborations,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FollowedArtistSortRule {
    None,
    AdditionDate,
    AdditionDateAsc,
    FollowerCount,
    Name,
    ReleaseDate,
    SongCount,
    SongRating,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NameMatchMode {
    None,
    Partial,
    StartsWidth,
    Exact,
    Words,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
pub enum LanguagePreference {
    Default,
    Japanese,
    Romaji,
    English,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
pub enum OptionalFields {
    None,

    AdditionalNames,
    Albums,
    Artists,
    Description,
    Groups,
    Lyrics,
    KnownLanguages,
    MainPicture,
    Members,
    Names,
    OldUsernames,
    PVs,
    Tags,
    ThumbUrl,
    WebLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SongSortRule {
    None,
    Name,
    AdditionDate,
    FavoritedTimes,
    RatingScore,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TopSongsFilterRule {
    CreateDate,
    PublishDate,
    Popularity,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum TopSongsVocalist {
    Nothing,
    Vocaloid,
    UTAU,
    CeVIO,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserSortRule {
    RegisterDate,
    Name,
    Group,
}
