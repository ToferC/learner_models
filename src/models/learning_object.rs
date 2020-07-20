use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;


use super::{Stream, Quiz, LearningStyle, 
    Personnel, Audience, Role, 
    PhysicalInfrastructure, DigitalInfrastructure,
    WebPage, Image};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a high level learning object such as a course
/// A learning object must contain at least one module, but may 
/// contain several.
pub struct LearningObject {
    pub id: i64,
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

    pub created: NaiveDate,

    #[dummy(faker = "(Faker, 3)")]
    pub updated: Vec<NaiveDate>,
    pub shut_down: Option<NaiveDate>,
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
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: String,
    pub image: Image,

    #[dummy(faker = "(Faker, 2..3)")]
    pub learning_styles: Vec<LearningStyle>,
    pub content: ContentType,

    #[dummy(faker = "(Sentences(EN, 1..3))")]
    pub learning_objectives: Vec<String>,
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