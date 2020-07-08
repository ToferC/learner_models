use chrono::prelude::*;

use super::{Location, Experience, Quiz, Verb, Evaluation, 
    Personnel, Audience, Role};

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
pub enum Referral {
    email_campaign ( String ),
    social( String ),
    newsletter( String ),
    direct,
}