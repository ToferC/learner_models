use chrono::prelude::*;

#[derive(Debug)]
pub struct WebPage {
    url: String,
    // Vec of AccessConversions
    access_conversions: Vec<AccessConversion>,
}

#[derive(Debug)]
pub struct AccessConversion {
    access: NaiveDateTime,
    convert: Option<NaiveDateTime>,
}

