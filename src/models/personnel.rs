use serde::{Serialize, Deserialize};

use super::learner::Group;
use super::location::Location;

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