use chrono::prelude::*;

use super::{Location, Stream, Quiz, Verb, 
    Personnel, Audience, Role};

#[derive(Debug)]
pub struct LearningObject {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub modules: Vec<Module>,
    pub target_audience: Vec<Audience>,
    pub communities: Vec<Role>,
    pub url: String,
    pub status: Status,

    pub created: NaiveDate,
    pub updated: Vec<NaiveDate>,
    pub shut_down: Option<NaiveDate>,
}

#[derive(Debug)]
pub enum Status {
    Ideation,
    Design,
    MVP,
    Pilot,
    Production,
    Paused,
    Discontinued,
}

// Need to evaluate each module separately
#[derive(Debug)]
pub struct Module {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub verb: Vec<Verb>,
    pub content: ContentType,
    pub learning_objectives: Vec<String>,
    pub duration_minutes: u32,
    pub experience: ExperienceTemplate,
    pub quiz: Option<Quiz>,
    pub url: String,

    // Infrastructure & Resources
    pub physicial_infrastructure: Option<PhysicalInfrastructure>,
    pub digital_infrastructure: Option<DigitalInfrastructure>,
    pub personnel: Option<Vec<Personnel>>,
    pub completed: bool,
}

#[derive(Debug)]
pub enum ContentType {
    OnlineFacilitated,
    InPersonFacilitated,
    Asyncronous,
    Event,
    Conference,
}

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

#[derive(Debug)]
/// Represents pre-populated data for a learner's experience
pub struct ExperienceTemplate {
    pub verb: Verb,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: chrono::Duration,
}