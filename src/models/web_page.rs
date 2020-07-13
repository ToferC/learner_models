use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebPage {
    url: String,
    // Vec of AccessConversions
    access_conversions: Vec<AccessConversion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessConversion {
    access: NaiveDateTime,
    convert: Option<NaiveDateTime>,
}

