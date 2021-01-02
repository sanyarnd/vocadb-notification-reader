use serde::{Deserialize, Serialize};

use crate::client::models::entrythumb::EntryType;
use crate::client::models::user::UserForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentForApiContract {
    author: UserForApiContract,
    #[serde(rename = "authorName")]
    author_name: String,
    created: String,
    entry: EntryRefContract,
    id: i32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscussionFolderContract {
    description: String,
    id: i32,
    #[serde(rename = "lastTopicAuthor")]
    last_topic_author: UserForApiContract,
    #[serde(rename = "lastTopicDate")]
    last_topic_date: String,
    name: String,
    #[serde(rename = "topicCount")]
    topic_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscussionTopicContract {
    author: UserForApiContract,
    #[serde(rename = "commentCount")]
    comment_count: i32,
    comments: Vec<CommentForApiContract>,
    content: String,
    created: String,
    #[serde(rename = "folderId")]
    folder_id: i32,
    id: i32,
    #[serde(rename = "lastComment")]
    last_comment: CommentForApiContract,
    locked: bool,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryRefContract {
    #[serde(rename = "entryType")]
    entry_type: EntryType,
    id: i32,
}
