use serde::{Deserialize, Deserializer, Serialize};
use strum_macros::AsRefStr;

use crate::client::models::entrythumb::EntryThumbForApiContract;
use crate::client::models::misc::OldUsernameContract;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub enum GroupId {
    Nothing,
    Limited,
    Regular,
    Trusted,
    Moderator,
    Admin,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
pub enum Inbox {
    Nothing,
    Received,
    Sent,
    Notifications,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Proficiency {
    Nothing,
    Basics,
    Intermediate,
    Advanced,
    Native,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserForApiContract {
    pub id: i32,
    pub name: String,
    pub active: bool,
    #[serde(rename = "memberSince")]
    pub member_since: String,
    #[serde(rename = "verifiedArtist")]
    pub verified_artist: bool,
    #[serde(rename = "groupId")]
    pub group_id: GroupId,
    #[serde(rename = "knownLanguages")]
    pub known_languages: Option<Vec<UserKnownLanguageContract>>,
    #[serde(rename = "mainPicture")]
    pub main_picture: Option<EntryThumbForApiContract>,
    #[serde(rename = "oldUsernames")]
    pub old_usernames: Option<Vec<OldUsernameContract>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserKnownLanguageContract {
    #[serde(rename = "cultureCode")]
    pub culture_code: String,
    pub proficiency: Proficiency,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMessageContract {
    pub id: i32,
    pub subject: String,
    pub body: String,
    #[serde(
        rename = "createdFormatted",
        deserialize_with = "formatted_string_to_date"
    )]
    pub created_formatted: DateTime<Utc>,
    #[serde(rename = "highPriority")]
    pub high_priority: bool,
    pub inbox: Inbox,
    pub read: bool,
    pub receiver: UserForApiContract,
    pub sender: Option<UserForApiContract>,
}

fn formatted_string_to_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // 11.02.2018 11:19
    const FORMAT: &'static str = "%d.%m.%Y %H:%M";

    let date_str = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&date_str, FORMAT)
        .map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWithEmailContract {
    pub email: String,
    pub id: i32,
    pub name: String,
}
