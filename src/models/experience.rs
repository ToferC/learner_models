use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// Represents a learner's experience as part of an Xapi 
/// or experience tracking system
pub struct Experience {
    pub date_stamp: NaiveDate,
    pub module_id: i64,
    pub learning_style: LearningStyle,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: String,
    pub difficulty: u32,
    pub value: u32,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// A broad domain of learning (currently 
/// the digital streams)
pub enum Stream {
    Data,
    Design,
    Disruption,
    AIML,
    DevOps,
    Development,
    Agile,
    ProductOwner,
    Leadership,
    DigiGov,
    Finance,
    HumanResources,
    Collaboration,
    ManyMore,
}

#[derive(Serialize, Deserialize, Debug)]
/// The type of learning that occurred in an experience. 
/// May also use this to identify preferred learning 
/// styles for learners.
pub enum LearningStyle {
    Read,
    Write,
    Listen,
    Watch,
    Do,
    Practice,
    Study,
    Participate,
}