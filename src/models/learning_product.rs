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
    WebPage, Image, TimeString, TimeStringEarly, TimeStringLate,
    random_gen_quality};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a high level learning object such as a course
/// A learning object must contain at least one module, but may 
/// contain several.
pub struct LearningProduct {
    pub id: u32,
    pub code: String,
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

impl Default for LearningProduct {
    fn default() -> Self {
        LearningProduct {
            id: 100,
            code: String::from("I501"),
            name: String::from("Discover Digital"),
            description: String::from("Discover Digital series 1"),
            image: Faker.fake(),
            modules: Vec::new(),
            target_audience: vec![Audience::Employee, Audience::Manager],
            communities: vec![Role::All],
            web_page: Faker.fake(),
            hashtag: String::from("#DiscoverDigital"),
            business_line: BusinessLine::DigitalAcademy,
            status: Status::Pilot,
            created: Faker.fake(),
            updated: Vec::new(),
            shut_down: None,
        }
    }
}

impl LearningProduct {
    pub fn new(
        id: u32, name: String, code: String, description: String, audience: Audience, community: Role,
        hashtag: String, business_line: BusinessLine, status: Status
    ) -> LearningProduct {
        LearningProduct {
            id: id,
            name: name,
            code: code,
            description: description,
            image: Faker.fake(),
            modules: Vec::new(),
            target_audience: vec![audience,],
            communities: vec![community,],
            web_page: Faker.fake(),
            hashtag: hashtag,
            business_line: business_line,
            status: status,
            created: Faker.fake(),
            updated: Vec::new(),
            shut_down: None,
        }
    }
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

    #[dummy(faker = "0.3..0.99")]
    pub mock_quality: f64,

    #[dummy(faker = "0.3..0.99")]
    pub mock_clear: f64,
    #[dummy(faker = "0.3..0.99")]
    pub mock_entertaining: f64,
    #[dummy(faker = "0.3..0.99")]
    pub mock_relevant: f64,
    #[dummy(faker = "0.3..0.99")]
    pub mock_infomative: f64,
    #[dummy(faker = "0.3..0.99")]
    pub mock_useful: f64,
    #[dummy(faker = "0.3..0.99")]
    pub mock_inclusive: f64,

    #[dummy(faker = "0.3..0.99")]
    pub mock_difficulty: f64,

    #[dummy(faker = "0.3..0.99")]
    pub mock_length: f64,

    // Infrastructure & Resources
    pub physicial_infrastructure_id: Option<u32>,
    pub digital_infrastructure_id: Option<u32>,
    
    #[dummy(faker = "(Faker, 2..3)")]
    pub personnel_ids: Option<Vec<u32>>,
    }

impl Default for Module {
    fn default() -> Self {
        Module {
            id: 100,
            code: String::from("I501"),
            name: String::from("Discover Data"),
            description: String::from("Data is transforming our world..."),
            image: Faker.fake(),
            learning_styles: vec![LearningStyle::Study,],
            content: ContentType::OnlineFacilitated,
            learning_objectives: Faker.fake(),
            duration_minutes: 90,
            experience: Faker.fake(),
            quiz: None,
            web_page: Faker.fake(),
            mock_quality: 0.5,
            mock_clear: random_gen_quality(0.5),
            mock_entertaining: random_gen_quality(0.5),
            mock_relevant: random_gen_quality(0.5),
            mock_infomative: random_gen_quality(0.5),
            mock_useful: random_gen_quality(0.5),
            mock_inclusive: random_gen_quality(0.5),

            mock_difficulty: random_gen_quality(0.5),

            mock_length: random_gen_quality(0.5),

            physicial_infrastructure_id: None,
            digital_infrastructure_id: None,
            personnel_ids: None,
        }
    }
}

impl Module {
    pub fn new(
        id: u32,
        code: String,
        name: String,
        description: String,
        learning_styles: Vec<LearningStyle>,
        content: ContentType,
        learning_obj: Vec<LearningObjective>,
        duration: u32,
        quality: f64,
    ) -> Self {
        Module {
            id: id,
            code: code,
            name: name,
            description: description,
            image: Faker.fake(),
            learning_styles: learning_styles,
            content: content,
            learning_objectives: learning_obj,
            duration_minutes: duration,
            experience: Faker.fake(),
            quiz: None,
            web_page: Faker.fake(),
            mock_quality: quality,
            mock_clear: random_gen_quality(quality),
            mock_entertaining: random_gen_quality(quality),
            mock_relevant: random_gen_quality(quality),
            mock_infomative: random_gen_quality(quality),
            mock_useful: random_gen_quality(quality),
            mock_inclusive: random_gen_quality(quality),
            mock_difficulty: random_gen_quality(quality),
            mock_length: random_gen_quality(quality),
            physicial_infrastructure_id: None,
            digital_infrastructure_id: None,
            personnel_ids: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// A learning objective
pub struct LearningObjective {
    #[dummy(faker = "0.1..0.99")]
    pub weight: f64,
    pub statement: Statement,
}

impl Default for LearningObjective {
    fn default() -> Self {
        LearningObjective {
            weight: 0.5,
            statement: Statement::default(),
        }
    }
}

impl LearningObjective {
    pub fn new(weight: f64, statement: Statement) -> Self {
        LearningObjective {
            weight: weight,
            statement: statement,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// A learning objective of a module describing intended outcomes.
/// Expressed as "A learner can {verb} {noun}."
pub struct Statement {
    pub verb: Verb,

    #[dummy(faker = "BsNoun(EN)")]
    pub noun: String,

    #[dummy(faker = "0.1..0.99")]
    pub difficulty: f64,
}

impl Default for Statement {
    fn default() -> Self {
        Statement {
            verb: Verb::Understand,
            noun: String::from("Concept"),
            difficulty: 0.5,
        }
    }
}

impl Statement {
    pub fn new(
        verb: Verb,
        noun: String,
        diff: f64,
    ) -> Self {
        Statement {
            verb: Verb::Understand,
            noun: String::from("Concept"),
            difficulty: 0.5,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, Copy)]
/// Verb element of a statement for a learning objective
pub enum Verb {
    Understand,
    Describe,
    Explain,
    Perform,
    Design,
    Build,
    Teach,
    WorkWith,
    ManyMore,
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