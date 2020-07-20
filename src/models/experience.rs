use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::faker::chrono::raw::*;
use chrono::{Utc, Duration};
use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a learner's experience as part of an Xapi 
/// or experience tracking system
pub struct Experience {
    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    #[dummy(faker = "(1..11)")]
    pub module_id: u32,
    pub learning_style: LearningStyle,
    pub stream: Stream,
    //pub practice: String,
    //pub skill: String,
    pub validated: bool,

    #[dummy(faker = "(1..360)")]
    pub time: u32,

    #[dummy(faker = "(1..11)")]
    pub difficulty: usize,

    #[dummy(faker = "(1..11)")]
    pub value: usize,
    
    #[dummy(faker = "BsAdj(EN)")]
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
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