use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum WebLinkCategory {
    Official,
    Commercial,
    Reference,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebLinkContract {
    category: WebLinkCategory,
    description: String,
    #[serde(rename = "descriptionOrUrl")]
    description_or_url: String,
    id: i32,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebLinkForApiContract {
    category: WebLinkCategory,
    description: String,
    id: i32,
    url: String,
}
