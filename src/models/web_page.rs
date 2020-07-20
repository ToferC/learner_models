use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct WebPage {
    url: String,
    // Vec of AccessConversions
    access_conversions: Vec<AccessConversion>,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct AccessConversion {
    access: NaiveDateTime,
    convert: Option<NaiveDateTime>,
}

