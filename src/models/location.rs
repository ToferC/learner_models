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

    pub id: u32,

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

impl Default for Location {
    fn default() -> Self {
        Location {
            id: 300,
            street_number: 186,
            address: String::from("Place du Portage"),
            city: String::from("Gatineau"),
            province: String::from("Quebec"),
            timezone_offset: 0,
        }
    }
}

impl Location {
    pub fn new(id: u32, street_no: usize, address: String, city: String, prov: String, tz: i32) -> Self {
        Location {
            id: id,
            street_number: street_no,
            address: address,
            city: city,
            province: prov,
            timezone_offset: tz,
        }
    }
}