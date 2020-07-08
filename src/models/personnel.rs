use super::learner::Group;
use super::location::Location;

#[derive(Debug)]
// An employee working in learning delivery.
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
// A delivery role for an employee.
pub enum DeliveryRole {
    Facilitator,
    Producer,
    Speaker,
    Operations,
    Broadcasting,
}