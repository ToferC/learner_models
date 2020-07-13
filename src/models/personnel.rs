use serde::{Serialize, Deserialize};

use super::learner::Group;
use super::location::Location;

#[derive(Serialize, Deserialize, Debug)]
/// An employee working in learning delivery.
pub struct Personnel {
    id: u32,
    last_name: String,
    first_name: String,
    role: DeliveryRole,
    group: Group,
    level: usize,
    salary: u32,
    work_location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
/// A delivery role for an employee.
pub enum DeliveryRole {
    Facilitator,
    Producer,
    Speaker,
    Operations,
    Broadcasting,
    Other ( String ),
}