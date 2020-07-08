use super::{Location};

#[derive(Debug)]
pub struct PhysicalInfrastructure {
    pub location: Location,
    pub opening_hours: String,
    pub closing_hours: String,
    pub capacity: u32,
    pub wifi: Option<u32>,
    pub cost_per_hour: f64,
    pub map_url: String,
}

#[derive(Debug)]
pub struct DigitalInfrastructure {
    pub storage: u64,
    pub cost_per_minute: f64,
    pub capacity: u32,
    pub url: String,
}