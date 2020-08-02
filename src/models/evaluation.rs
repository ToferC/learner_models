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
    pub fn generate_response(module_qualities: &[f64; 9], learner_openness: f64) -> RapidResponse {

        let mut qual_responses: Vec<bool> = Vec::new();

        let mut rng = rand::thread_rng();

        for v in module_qualities {
            // learner openness determines the quality value that
            // the learner is willing to recognize
            let x = v * learner_openness;

            if rng.gen_range(0.01, 1.00) < x - THRESHOLD {
                qual_responses.push(true)
            } else {
                qual_responses.push(false)
            };
        };



        RapidResponse {
            would_recommend: true,
            rating_1_10: qual_responses.iter().filter(|x| *x == &true).count(),
            clear: qual_responses[1],
            entertaining: qual_responses[2],
            relevant: qual_responses[3],
            informative: qual_responses[4],
            useful: qual_responses[5],
            inclusive: qual_responses[6],
            too_easy: qual_responses[7],
            too_difficult: !qual_responses[7],
            too_long: qual_responses[8],
            too_short: !qual_responses[8],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating 
/// PhysicalInfrastructure
pub struct PhysicalEval {
    pub clean: bool,
    pub pleasant: bool,
    pub comfortable: bool,
    pub professional: bool,
    pub accessible: bool,
}

impl PhysicalEval {
    pub fn generate_response(module_qualities: &[f64; 5], learner_openness: f64) -> PhysicalEval{

        let mut qual_responses: Vec<bool> = Vec::new();

        let mut rng = rand::thread_rng();

        for v in module_qualities {
            // learner openness determines the quality value that
            // the learner is willing to recognize
            let x = v * learner_openness;

            if rng.gen_range(0.01, 1.00) < x {
                qual_responses.push(true)
            } else {
                qual_responses.push(false)
            };
        };

        PhysicalEval {
            clean: qual_responses[0],
            pleasant: qual_responses[1],
            comfortable: qual_responses[2],
            professional: qual_responses[3],
            accessible: qual_responses[4],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating Personnel
pub struct PersonnelEval {
    pub pleasant: bool,
    pub helpful: bool,
    pub professional: bool,
}

impl PersonnelEval {
    pub fn generate_response(module_qualities: &[f64; 3], learner_openness: f64) -> PersonnelEval {

        let mut qual_responses: Vec<bool> = Vec::new();

        let mut rng = rand::thread_rng();

        for v in module_qualities {
            // learner openness determines the quality value that
            // the learner is willing to recognize
            let x = v * learner_openness;

            if rng.gen_range(0.01, 1.00) < x {
                qual_responses.push(true)
            } else {
                qual_responses.push(false)
            };
        };

        PersonnelEval {
            pleasant: qual_responses[0],
            helpful: qual_responses[1],
            professional: qual_responses[2],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Optional true or false responses for evaluating 
/// DigitalInfrastructure
pub struct DigitalEval {
    pub smooth: bool,
    pub accessible: bool,
    pub professional: bool,
}

impl DigitalEval {
    pub fn generate_response(module_qualities: &[f64; 3], learner_openness: f64) -> DigitalEval{

        let mut qual_responses: Vec<bool> = Vec::new();

        let mut rng = rand::thread_rng();

        for v in module_qualities {
            // learner openness determines the quality value that
            // the learner is willing to recognize
            let x = v * learner_openness;

            if rng.gen_range(0.01, 1.00) < x {
                qual_responses.push(true)
            } else {
                qual_responses.push(false)
            };
        };

        DigitalEval {
            smooth: qual_responses[0],
            accessible: qual_responses[1],
            professional: qual_responses[2],
        }
    }
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