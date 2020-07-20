use serde::{Serialize, Deserialize};
use fake::{Dummy, Fake, Faker};

use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::faker::name::raw::*;
use fake::faker::address::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

use chrono::offset::TimeZone;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A multiple choice knowledge test
pub struct Quiz {
    pub id: u32,
    pub questions: Vec<Question>,
    pub score: usize,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_time: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A question in a knowledge test
pub struct Question {
    pub number: u32,
    pub question_text: String,

    #[dummy(faker = "(Sentences(EN, 2..3))")]
    pub answers: Vec<String>,

    #[dummy(faker = "(1..11)")]
    pub correct_answer: usize,
}