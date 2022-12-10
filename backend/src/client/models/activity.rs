use serde::{Deserialize, Serialize};

use crate::client::models::archived::ArchivedObjectVersionForApiContract;
use crate::client::models::entrythumb::EntryForApiContract;
use crate::client::models::user::UserForApiContract;

#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityEditEvent {
    Created,
    Updated,
    Deleted,
    Restored,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityEntryForApiContract {
    #[serde(rename = "archivedVersion")]
    archived_version: ArchivedObjectVersionForApiContract,
    author: UserForApiContract,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "editEvent")]
    edit_event: ActivityEditEvent,
    entry: EntryForApiContract,
}
