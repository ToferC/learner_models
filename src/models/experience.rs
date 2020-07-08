use chrono::prelude::*;

#[derive(Debug)]
/// Represents a learner's experience as part of an Xapi or experience
/// tracking system
pub struct Experience {
    pub date_stamp: NaiveDate,
    pub module_id: i64,
    pub verb: Verb,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: chrono::Duration,
    pub difficulty: u32,
    pub value: u32,
    pub tags: Vec<String>,
}

#[derive(Debug)]
/// A broad domain of learning (currently the digital streams)
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
}

#[derive(Debug)]
/// The type of learning that occurred in an experience. May also use this
/// to identify preferred learning styles for learners.
pub enum Verb {
    Read,
    Write,
    Listen,
    Watch,
    Do,
    Practice,
    Study,
    Participate,
}