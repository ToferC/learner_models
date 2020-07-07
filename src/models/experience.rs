use chrono::prelude::*;

#[derive(Debug)]
pub struct Experience {
    pub date_stamp: NaiveDate,
    pub verb: Verb,
    pub stream: Stream,
    pub practice: String,
    pub skill: String,
    pub validated: bool,
    pub time: chrono::Duration,
    pub difficulty: u32,
    pub value: u32,
}

#[derive(Debug)]
pub enum Stream {
    Data,
    Design,
    AIML,
    DevOps,
    Development,
    Agile,
    ProductOwner,
    Leadership,
    DigiGov,
}

#[derive(Debug)]
pub enum Verb {
    Read,
    Write,
    Listen,
    Watch,
    Do,
    Practice,
}