use chrono::prelude::*;

use super::{Location, Experience, Quiz, Verb, Evaluation, MicroEvaluation, Personnel};

#[derive(Debug)]
pub struct Registration {
    pub id: i64,
    pub date_stamp: NaiveDate,
    pub offering: Offering,
    pub referral_source: Referral,
    pub evaluation: Evaluation,
    pub completed: bool,
    pub cancelled: bool,
}

#[derive(Debug)]
pub struct Offering {
    pub id: i64,
    pub learning_object: LearningObject,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub cancelled: bool,
    pub completed: bool,
}

#[derive(Debug)]
pub struct LearningObject {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub modules: Vec<Module>,
    pub communities: Vec<String>,
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
    pub experience: Experience,
    pub quiz: Option<Quiz>,

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
}

#[derive(Debug)]
pub struct DigitalInfrastructure {
    pub storage: u64,
    pub cost_per_minute: f64,
}

#[derive(Debug)]
pub enum Referral {
    email,
    social( String ),
    direct,
}