use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use fake::{Dummy, Fake, Faker};

use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::faker::boolean::en::*;
use fake::locales::*;

use super::{LearningProduct, Evaluation, Learner};

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// A registration point within our learning management system
/// Serves as a placeholder for a specific offering of a 
/// learning object
pub struct Registration {

    pub id: u32,

    pub learner_id: u32,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    pub offering_id: u32,
    pub referral_source: Referral,

    #[dummy(faker = "Boolean(70)")]
    pub completed: bool,
    #[dummy(faker = "Boolean(5)")]
    pub cancelled: bool,
}

impl Default for Registration {
    fn default() -> Self {
        Registration {
            id: 100,
            learner_id: 100,
            date_stamp: String::from("2020-06-01"),
            offering_id: 100,
            referral_source: Faker.fake(),
            completed: true,
            cancelled: false,
        }
    }
}

impl Registration {
    pub fn new(id: u32, learner_id: u32, date_stamp: String, offering_id: u32, completed: bool, cancelled: bool) -> Self {
        Registration {
            id: id,
            learner_id: learner_id,
            date_stamp: date_stamp,
            offering_id: offering_id,
            referral_source: Faker.fake(),
            completed: completed,
            cancelled: cancelled,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// A specific offering of a learning object. Also serves as 
/// the placholder for an overall evaluation of CSPS learning 
/// content.
pub struct Offering {
    pub id: u32,
    pub learning_product_id: u32,
    pub evaluation_ids: Vec<u32>,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub start_date: String,

    #[dummy(faker = "Boolean(5)")]
    pub cancelled: bool,

    #[dummy(faker = "Boolean(80)")]
    pub completed: bool,
}

impl Offering {
    pub fn new(id: u32, learning_prod_id: u32, start_date: String, cancelled: bool, completed: bool) -> Self {
        Offering {
            id: id,
            learning_product_id: learning_prod_id,
            evaluation_ids: Vec::new(),
            start_date: start_date,
            cancelled: cancelled,
            completed: completed,
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Referral data for a registration. Contains a String which can
/// represent a specific promotional campaign.
pub enum Referral {
    Email ( String ),
    Social( String ),
    Newsletter( String ),
    Direct,
}

impl Default for Referral {
    fn default() -> Self {
        Referral::Email (String::from("campaign_1"))
    }
}