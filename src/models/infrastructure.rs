use std::rc::Rc;

use super::{Location, WebPage};
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A physical delivery point for learning content. 
/// Optionally attached to a Module.
pub struct PhysicalInfrastructure {
    pub id: u32,
    pub location_id: u32,
    pub opening_hours: String,
    pub closing_hours: String,
    pub capacity: u32,
    pub wifi: Option<u32>,

    #[dummy(faker = "0.1..0.99")]
    pub mock_quality: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_cleanliness: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_comfort: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_professional: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_pleasant: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_accessible: f64,

    pub cost_per_hour: f64,
    pub map_url: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A digital delivery point for learning content. 
/// Optionally attached to a Module.
pub struct DigitalInfrastructure {
    pub id: u32,
    pub storage: u64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_quality: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_professional: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_smooth: f64,

    #[dummy(faker = "0.1..0.99")]
    pub mock_accessible: f64,
    
    pub cost_per_minute: f64,
    pub capacity: u32,
    pub web_page: WebPage,
}