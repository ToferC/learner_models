#[derive(Debug)]
// A location that can apply to PhysicalInfrastructure, a Learner
// or Personnel.
pub struct Location {
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub timezone_offset: i32,
}