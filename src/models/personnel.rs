use serde::{Serialize, Deserialize};

use super::learner::Group;
use super::location::Location;
use super::utilities::random_gen_quality;

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// An employee working in learning delivery.
pub struct Personnel {
    id: u32,
    #[dummy(faker = "FirstName(EN)")]
    pub first_name: String,

    #[dummy(faker = "LastName(EN)")]
    pub last_name: String,

    /// simulates the effectiveness of the person on a scale of 1-10
    #[dummy(faker = "0.1..0.99")]
    pub mock_quality: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_professionalism: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_pleasant: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_helpful: f64,

    /// simulates the chance of personnel making a critical error
    /// in learning delivery that alienates learners.
    #[dummy(faker = "0.01..0.2")]
    pub error_chance: f64,

    pub role: DeliveryRole,
    pub group: Group,
    pub level: usize,

    #[dummy(faker = "50_000..120_000")]
    pub salary: u32,

    pub work_location_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A delivery role for an employee.
pub enum DeliveryRole {
    Facilitator,
    Producer,
    Speaker,
    Operations,
    Broadcasting,
    Other ( String ),
}

impl Default for Personnel {
    fn default() -> Self {
        Personnel {
            id: 100,
            first_name: String::from("Default"),
            last_name: String::from("Facilitator"),
            mock_quality: random_gen_quality(0.5),
            mock_professionalism: random_gen_quality(0.5),
            mock_pleasant: random_gen_quality(0.5),
            mock_helpful: random_gen_quality(0.5),
            error_chance: random_gen_quality(0.5),
            role: DeliveryRole::Facilitator,
            group: Group::EC,
            level: 06,
            salary: 100_000,
            work_location_id: 0,
        }
    }
}

impl Personnel {
    pub fn new(id: u32, fname: String, lname: String, quality: f64, role: DeliveryRole,
    group: Group, level: usize, salary: u32, work_loc: u32) -> Self {
        Personnel {
            id: id,
            first_name: fname,
            last_name: lname,
            mock_quality: quality,
            mock_professionalism: random_gen_quality(quality),
            mock_pleasant: random_gen_quality(quality),
            mock_helpful: random_gen_quality(quality),
            error_chance: random_gen_quality(quality),
            role: role,
            group: group,
            level: level,
            salary: salary,
            work_location_id: work_loc,
        }
    }
}