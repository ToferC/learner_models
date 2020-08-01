use std::collections::HashMap;

use chrono::prelude::*;
use rand::prelude::*;

use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

use super::{LearningObjective, Module, PhysicalInfrastructure, DigitalInfrastructure, Personnel,
    THRESHOLD, random_gen_quality};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// An over-arching evaluation structure for a LearningProduct.
/// Contains MicroEvaluations.
pub struct Evaluation {
    pub id: u32,
    pub objective: Objective,

    // Before learning
    #[dummy(faker = "1..11")]
    pub current_skill: usize,

    #[dummy(faker = "1..11")]
    pub desired_skill: usize,

    // After learning
    #[dummy(faker = "1..11")]
    pub end_skill: usize,

    #[dummy(faker = "Sentence(EN, 1..3)")]
    pub comments: String,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    #[dummy(faker = "(Faker, 3)")]
    pub micro_evaluations: Vec<MicroEvaluation>,
}



#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Short, focused evaluations for Modules within a LearningProduct.
/// Contains several optional evaluations depending on the specific 
/// Module.
pub struct MicroEvaluation {
    pub id: u32,

    #[dummy(faker = "1..11")]
    pub module: usize,    

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    pub rapid_response: RapidResponse,

    // Hashmap of a LearningProductive for a module mapped to an 
    // assessment of how the module met that objective
    pub learning_obj_eval: Vec<(LearningObjective, LearningProductiveResponse)>,
    pub physical_eval: Option<PhysicalEval>,
    pub digital_eval: Option<DigitalEval>,
    pub personnel_eval: Option<PersonnelEval>,
    pub sent: bool,
    pub seen: bool,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Learner assessment of whether a Module met a specific 
/// LearningProductive.
pub enum LearningProductiveResponse {
    NotMeet,
    Meet,
    Exceeded,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Core component of MicroEvaluation with two mandatory and 
/// an array of optional true or false responses.
pub struct RapidResponse {
    pub would_recommend: bool,

    #[dummy(faker = "1..11")]
    pub rating_1_10: usize,

    // Positive
    pub clear: bool,
    pub entertaining: bool,
    pub relevant: bool,
    pub informative: bool,
    pub useful: bool,
    pub inclusive: bool,

    // Negative
    pub too_easy: bool,
    pub too_difficult: bool,
    pub too_long: bool,
    pub too_short: bool,
}

impl RapidResponse {
    pub fn generate_response(module_qualities: &[f64; 8], learner_openness: f64) {

        let [clear, entertaining, relevant, informative, useful, inclusive, difficulty, length] = module_qualities;

        let c = random_gen_quality(*clear); // creates a bell curve around the mock_entertain quality

        if c >= learner_openness {
            println!("True") 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating 
/// PhysicalInfrastructure
pub struct PhysicalEval {
    #[dummy(faker = "1..11")]
    pub module: usize,
    pub clean: bool,
    pub pleasant: bool,
    pub comfortable: bool,
    pub professional: bool,
    pub accessible: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating Personnel
pub struct PersonnelEval {
    #[dummy(faker = "1..11")]
    pub module: usize,
    pub pleasant: bool,
    pub helpful: bool,
    pub professional: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating 
/// DigitalInfrastructure
pub struct DigitalEval {
    #[dummy(faker = "1..11")]
    pub module: usize,
    pub smooth: bool,
    pub accessible: bool,
    pub professional: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// The Learner's primary reason for seeking learning. 
/// Serves as context for other evaluations.
pub enum Objective {
    ImproveCurrentSkills,
    NewJobPromotional,
    NewJobLateral,
    CareerChange,
    PersonalInterest,
    MandatoryLearning,
    Other ( String ),
}