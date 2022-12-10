use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    Unspecified,
    Japanese,
    Romaji,
    English,
}
