use super::learner::Group;
use super::location::Location;

#[derive(Debug)]
pub struct Personnel {
    id: u32,
    last_name: String,
    first_name: String,
    role: DeliveryRole,
    group: Group,
    level: usize,
    salary: u32,
    residence: Location,
}

#[derive(Debug)]
pub enum DeliveryRole {
    Facilitator,
    Producer,
    Speaker,
    Operations,
    Broadcasting,
}