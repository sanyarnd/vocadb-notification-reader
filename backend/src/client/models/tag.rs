use serde::{Deserialize, Serialize};

use crate::client::models::entrythumb::EntryThumbForApiContract;
use crate::client::models::language::Language;
use crate::client::models::status::Status;
use crate::client::models::weblink::WebLinkForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnglishTranslatedStringContract {
    english: String,
    original: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalizedStringWithIdContract {
    id: i32,
    language: Language,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagBaseContract {
    pub id: i32,
    pub name: String,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    #[serde(rename = "additionalNames")]
    pub additional_names: String,
    #[serde(rename = "urlSlug")]
    pub url_slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagForApiContract {
    #[serde(rename = "additionalNames")]
    additional_names: String,
    #[serde(rename = "aliasedTo")]
    aliased_to: TagBaseContract,
    #[serde(rename = "categoryName")]
    category_name: String,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "defaultNameLanguage")]
    default_name_language: Language,
    description: String,
    id: i32,
    #[serde(rename = "mainPicture")]
    main_picture: EntryThumbForApiContract,
    name: String,
    names: Vec<LocalizedStringWithIdContract>,
    parent: TagBaseContract,
    #[serde(rename = "relatedTags")]
    related_tags: Vec<TagBaseContract>,
    status: Status,
    targets: i32,
    #[serde(rename = "translatedDescription")]
    translated_description: EnglishTranslatedStringContract,
    #[serde(rename = "urlSlug")]
    url_slug: String,
    #[serde(rename = "usageCount")]
    usage_count: i32,
    version: i32,
    #[serde(rename = "webLinks")]
    web_links: Vec<WebLinkForApiContract>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagUsageForApiContract {
    pub tag: TagBaseContract,
    pub count: i32,
}
