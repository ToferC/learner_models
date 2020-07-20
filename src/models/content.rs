use reqwest::Url;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Struct containing image URL
pub struct Image {
    pub id: i64,
    pub path: String,
}