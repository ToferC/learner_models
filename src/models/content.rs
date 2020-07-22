use reqwest::Url;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};
use fake::faker::chrono::raw::*;
use chrono::Utc;
use chrono::prelude::*;
use fake::locales::*;


#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Struct containing image URL
pub struct Image {
    pub id: u32,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, Dummy, PartialEq)]
pub struct TimeStringEarly(#[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2018, 1, 1).and_hms(9, 10, 11), Utc.ymd(2019,6,12).and_hms(9, 10, 11))")] String);

#[derive(Serialize, Deserialize, Debug, Eq, Dummy, PartialEq)]
pub struct TimeString(#[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")] String);

#[derive(Serialize, Deserialize, Debug, Eq, Dummy, PartialEq)]
pub struct TimeStringLate(#[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2021, 1, 1).and_hms(9, 10, 11), Utc.ymd(2022,6,12).and_hms(9, 10, 11))")] String);