use serde::{Deserialize, Serialize};

use crate::client::models::weblink::WebLinkCategory;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchivedObjectVersionForApiContract {
    #[serde(rename = "changedFields")]
    changed_fields: Vec<String>,
    id: i32,
    notes: String,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchivedWebLinkContract {
    category: WebLinkCategory,
    description: String,
    url: String,
}
