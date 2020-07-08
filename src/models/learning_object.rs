use chrono::prelude::*;

use super::{Stream, Quiz, Verb, 
    Personnel, Audience, Role, 
    PhysicalInfrastructure, DigitalInfrastructure};

#[derive(Debug)]
// Represents a high level learning object such as a course
// A learning object must contain at least one module, but may contain
// several.
pub struct LearningObject {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub modules: Vec<Module>,
    pub target_audience: Vec<Audience>,
    pub communities: Vec<Role>,
    pub url: String,
    pub hashtag: String,
    pub status: Status,

    pub created: NaiveDate,
    pub updated: Vec<NaiveDate>,
    pub shut_down: Option<NaiveDate>,
}

#[derive(Debug)]
// A state of preparation and production for a learning object
pub enum Status {
    Ideation,
    Design,
    MVP,
    Pilot,
    Production,
    Paused,
    Discontinued,
}

#[derive(Debug)]
// A learning module that exists within a learning object.
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
// A content type for a learning module
pub enum ContentType {
    OnlineFacilitated,
    InPersonFacilitated,
    InPersonUnfacilitated,
    Asyncronous,
    Event,
    Conference,
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