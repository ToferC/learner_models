use super::{Location, WebPage};
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A physical delivery point for learning content. 
/// Optionally attached to a Module.
pub struct PhysicalInfrastructure {
    pub location: Location,
    pub opening_hours: String,
    pub closing_hours: String,
    pub capacity: u32,
    pub wifi: Option<u32>,
    pub cost_per_hour: f64,
    pub map_url: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A digital delivery point for learning content. 
/// Optionally attached to a Module.
pub struct DigitalInfrastructure {
    pub storage: u64,
    pub cost_per_minute: f64,
    pub capacity: u32,
    pub web_page: WebPage,
}