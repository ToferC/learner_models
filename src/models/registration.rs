use chrono::prelude::*;

use super::{LearningObject, Evaluation};

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
    Email ( String ),
    Social( String ),
    Newsletter( String ),
    Direct,
}