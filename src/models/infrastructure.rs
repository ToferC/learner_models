use super::{Location, WebPage};

use super::utilities::random_gen_quality;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A physical delivery point for learning content. 
/// Optionally attached to a Module.
pub struct PhysicalInfrastructure {
    pub id: u32,
    pub location_id: u32,
    pub name: String,
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

impl Default for PhysicalInfrastructure {
    fn default() -> Self {
        PhysicalInfrastructure {
            id: 1000,
            location_id: 300,
            name: String::from("Room 300"),
            opening_hours: String::from("9h00"),
            closing_hours: String::from("17h00"),
            capacity: 24,
            wifi: Some(30),
            cost_per_hour: 300.0,
            map_url: String::from("map_url_one"),
    
            mock_quality: 0.5,
            mock_accessible: random_gen_quality(0.3),
            mock_comfort: random_gen_quality(0.3),
            mock_cleanliness: random_gen_quality(0.3),
            mock_pleasant: random_gen_quality(0.3),
            mock_professional: random_gen_quality(0.3),
        }
    }
}

impl PhysicalInfrastructure {
    pub fn new(id: u32, location_id: u32, name: String, capacity: u32, quality: f64) -> Self {
        PhysicalInfrastructure {
            id: id,
            location_id: location_id,
            name: name,
            opening_hours: String::from("9h00"),
            closing_hours: String::from("17h00"),
            capacity: capacity,
            wifi: Some(30),
            cost_per_hour: 300.0,
            map_url: String::from("map_url_one"),
    
            mock_quality: quality,
            mock_accessible: random_gen_quality(quality),
            mock_comfort: random_gen_quality(quality),
            mock_cleanliness: random_gen_quality(quality),
            mock_pleasant: random_gen_quality(quality),
            mock_professional: random_gen_quality(quality),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A digital delivery point for learning content. 
/// Optionally attached to a Module.
pub struct DigitalInfrastructure {
    pub id: u32,
    pub name: String,
    pub storage: u32,

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

impl Default for DigitalInfrastructure {
    fn default() -> Self {
        DigitalInfrastructure {
            id: 1000,
            name: String::from("Default"),
            storage: 10_000,
            mock_quality: 0.5,
            mock_professional: random_gen_quality(0.5),
            mock_smooth: random_gen_quality(0.5),
            mock_accessible: random_gen_quality(0.5),
            cost_per_minute: 0.034,
            capacity: 1000,
            web_page: WebPage {
                url: String::from("default_url"),
                access_conversions: Faker.fake()
            },
        }
    }
}

impl DigitalInfrastructure {
    pub fn new(id: u32, name: String, cost: f64, capacity: u32, storage: u32, quality: f64) -> Self {
        DigitalInfrastructure {
            id: id,
            name: name,
            storage: storage,
            mock_quality: quality,
            mock_accessible: random_gen_quality(quality),
            mock_smooth: random_gen_quality(quality),
            mock_professional: random_gen_quality(quality),
            cost_per_minute: cost,
            capacity: capacity,
            web_page: WebPage {
                url: String::from("default_url"),
                access_conversions: Faker.fake()
            },
        }
    }
}