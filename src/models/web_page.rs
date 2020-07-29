use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};
use fake::faker::chrono::raw::*;
use fake::locales::*;


#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct WebPage {
    pub url: String,
    // Vec of AccessConversions
    #[dummy(faker = "(Faker, 2..4)")]
    pub access_conversions: Vec<AccessConversion>,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct AccessConversion {
    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    access: String,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    convert: Option<String>,
}

