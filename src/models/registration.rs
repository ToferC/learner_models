use chrono::prelude::*;

use super::{Location, Experience, Stream, Verb, Evaluation};

#[derive(Debug)]
pub struct Registration {
    pub id: i64,
    pub date_stamp: NaiveDate,
    pub offering: Offering,
    pub referral_source: Referral,
    pub objective: Objective,
    pub current_skill: usize,
    pub desired_skill: usize,
    pub completed: bool,
}

#[derive(Debug)]
pub enum Objective {
    ImproveCurrentSkills,
    NewJob_Promotional,
    NewJob_Lateral,
    CareerChange,
    PersonalInterest,
    MandatoryLearning,
    Other ( String ),
}

#[derive(Debug)]
pub struct Offering {
    pub id: i64,
    pub learning: LearningObject,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub completed: bool,
    pub evaluation: Evaluation,
}

#[derive(Debug)]
pub struct LearningObject {
    code: String,
    name: String,
    content: ContentType,
    duration_minutes: u32,
    experiences: Vec<Experience>,
}

#[derive(Debug)]
pub enum ContentType {
    OnlineCourse,
    InPersonCourse ( Classroom ),
    Asyncronous ( Verb ),
    Event,
    Conference,
}

// Should this be more generic? Might be a virtual classroom?
#[derive(Debug)]
pub struct Classroom {
    pub location: Location,
    pub opening_hours: String,
    pub closing_hours: String,
    pub capacity: u32,
    pub wifi: Option<u32>,
}

#[derive(Debug)]
pub enum Referral {
    email,
    social( String ),
    direct,
}