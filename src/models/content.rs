use reqwest::Url;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// Struct containing image URL
pub struct Image {
    pub id: i64,
    pub path: String,
}