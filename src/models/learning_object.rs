use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;
use fake::faker::chrono::raw::*;
use chrono::Utc;


use super::{Stream, Quiz, LearningStyle, 
    Personnel, Audience, Role, 
    PhysicalInfrastructure, DigitalInfrastructure,
    WebPage, Image, TimeString, TimeStringEarly, TimeStringLate};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a high level learning object such as a course
/// A learning object must contain at least one module, but may 
/// contain several.
pub struct LearningObject {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub image: Image,

    #[dummy(faker = "(Faker, 3)")]
    pub modules: Vec<Module>,

    #[dummy(faker = "(Faker, 3)")]
    pub target_audience: Vec<Audience>,

    #[dummy(faker = "(Faker, 3)")]
    pub communities: Vec<Role>,
    pub web_page: WebPage,
    pub hashtag: String,
    pub business_line: BusinessLine,
    pub status: Status,

    pub created: TimeString,

    pub updated: Vec<TimeStringEarly>,

    pub shut_down: Option<TimeStringLate>,
    // Question: Do we have version control built in?
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A line of business in the CSPS
pub enum BusinessLine {
    GCSkills,
    TransferrableSkills,
    IndigenousLearning,
    RespectfulInclusiveWorkplace,
    DigitalAcademy,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A learning module that exists within a learning object.
pub struct Module {
    #[dummy(faker = "1..11")]
    pub id: u32,
    pub code: String,

    #[dummy(faker = "CatchPhase(EN)")]
    pub name: String,
    pub description: String,
    pub image: Image,

    #[dummy(faker = "(Faker, 2..3)")]
    pub learning_styles: Vec<LearningStyle>,
    pub content: ContentType,

    #[dummy(faker = "(Faker, 1..3)")]
    pub learning_objectives: Vec<LearningObjective>,

    pub duration_minutes: u32,
    pub experience: ExperienceTemplate,
    pub quiz: Option<Quiz>,
    pub web_page: WebPage,

    // Infrastructure & Resources
    pub physicial_infrastructure: Option<PhysicalInfrastructure>,
    pub digital_infrastructure: Option<DigitalInfrastructure>,
    
    #[dummy(faker = "(Faker, 2..3)")]
    pub personnel: Option<Vec<Personnel>>,
    
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct LearningObjective {
    #[dummy(faker = "1..11")]
    pub weight: usize,
    #[dummy(faker = "Sentence(EN, 1..2)")]
    pub statement: String
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
//// Represents pre-populated data for a learner's 
/// experience
pub struct ExperienceTemplate {
    pub learning_style: LearningStyle,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: String,
}