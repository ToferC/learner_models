use reqwest::Url;

#[derive(Debug)]
/// Struct containing image URL
pub struct Image {
    pub id: i64,
    pub path: Url,
}