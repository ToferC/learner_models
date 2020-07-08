use chrono::prelude::*;

use super::{LearningObject, Evaluation};

#[derive(Debug)]
/// A registration point within our learning management system
/// Serves as a placeholder for a specific offering of a learning object
pub struct Registration {
    pub id: i64,
    pub date_stamp: NaiveDate,
    pub offering: Offering,
    pub referral_source: Referral,
    pub completed: bool,
    pub cancelled: bool,
}

#[derive(Debug)]
/// A specific offering of a learning object. Also serves as the placholder
/// for an overall evaluation of CSPS learning content.
pub struct Offering {
    pub id: i64,
    pub learning_object: LearningObject,
    pub evaluation: Evaluation,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub cancelled: bool,
    pub completed: bool,
}

#[derive(Debug)]
/// Referral data for a registration. Contains a String which can
/// represent a specific promotional campaign.
pub enum Referral {
    Email ( String ),
    Social( String ),
    Newsletter( String ),
    Direct,
}