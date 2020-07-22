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

    role: DeliveryRole,
    group: Group,
    level: usize,

    #[dummy(faker = "50_000..120_000")]
    salary: u32,

    work_location: Location,
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