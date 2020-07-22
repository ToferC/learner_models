use serde::{Serialize, Deserialize};

use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::address::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A location that can apply to PhysicalInfrastructure, 
/// a Learner or Personnel.
pub struct Location {
    #[dummy(faker = "(1..1800)")]
    pub street_number: usize,

    #[dummy(faker = "StreetName(EN)")]
    pub address: String,

    #[dummy(faker = "CityName(EN)")]
    pub city: String,

    #[dummy(faker = "StateName(EN)")]
    pub province: String,

    #[dummy(faker = "(-3..3)")]
    pub timezone_offset: i32,
}