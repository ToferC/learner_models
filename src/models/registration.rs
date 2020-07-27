use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake};

use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::faker::boolean::en::*;
use fake::locales::*;

use super::{LearningProduct, Evaluation, Learner};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A registration point within our learning management system
/// Serves as a placeholder for a specific offering of a 
/// learning object
pub struct Registration {

    pub id: u32,

    pub learner: Learner,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    pub offering: Offering,
    pub referral_source: Referral,

    #[dummy(faker = "Boolean(70)")]
    pub completed: bool,
    #[dummy(faker = "Boolean(5)")]
    pub cancelled: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A specific offering of a learning object. Also serves as 
/// the placholder for an overall evaluation of CSPS learning 
/// content.
pub struct Offering {
    pub id: u32,
    pub learning_product: LearningProduct,
    pub evaluation: Evaluation,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub start_date: String,

    //#[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    //pub end_date: String,

    #[dummy(faker = "Boolean(5)")]
    pub cancelled: bool,

    #[dummy(faker = "Boolean(80)")]
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Referral data for a registration. Contains a String which can
/// represent a specific promotional campaign.
pub enum Referral {
    Email ( String ),
    Social( String ),
    Newsletter( String ),
    Direct,
}