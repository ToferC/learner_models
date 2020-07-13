use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use super::{Stream, Quiz, LearningStyle, 
    Personnel, Audience, Role, 
    PhysicalInfrastructure, DigitalInfrastructure,
    WebPage, Image};

#[derive(Serialize, Deserialize, Debug)]
/// Represents a high level learning object such as a course
/// A learning object must contain at least one module, but may 
/// contain several.
pub struct LearningObject {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub image: Image,
    pub modules: Vec<Module>,
    pub target_audience: Vec<Audience>,
    pub communities: Vec<Role>,
    pub web_page: WebPage,
    pub hashtag: String,
    pub business_line: BusinessLine,
    pub status: Status,

    pub created: NaiveDate,
    pub updated: Vec<NaiveDate>,
    pub shut_down: Option<NaiveDate>,
    // Question: Do we have version control built in?
}

#[derive(Serialize, Deserialize, Debug)]
/// A state of preparation and production for 
/// a learning object
pub enum Status {
    Ideation,
    Design,
    MVP,
    Pilot,
    Production,
    Paused,
    Discontinued,
}

#[derive(Serialize, Deserialize, Debug)]
/// A line of business in the CSPS
pub enum BusinessLine {
    GCSkills,
    TransferrableSkills,
    IndigenousLearning,
    RespectfulInclusiveWorkplace,
    DigitalAcademy,
}

#[derive(Serialize, Deserialize, Debug)]
/// A learning module that exists within a learning object.
pub struct Module {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: String,
    pub image: Image,
    pub learning_styles: Vec<LearningStyle>,
    pub content: ContentType,
    pub learning_objectives: Vec<String>,
    pub duration_minutes: u32,
    pub experience: ExperienceTemplate,
    pub quiz: Option<Quiz>,
    pub web_page: WebPage,

    // Infrastructure & Resources
    pub physicial_infrastructure: Option<PhysicalInfrastructure>,
    pub digital_infrastructure: Option<DigitalInfrastructure>,
    pub personnel: Option<Vec<Personnel>>,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
/// A content type for a learning module
pub enum ContentType {
    OnlineFacilitated,
    InPersonFacilitated,
    InPersonUnfacilitated,
    Asyncronous,
    Event,
    Conference,
    Video,
    Podcast,
    LearningAid,
}

#[derive(Serialize, Deserialize, Debug)]
//// Represents pre-populated data for a learner's 
/// experience
pub struct ExperienceTemplate {
    pub learning_style: LearningStyle,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: chrono::Duration,
}