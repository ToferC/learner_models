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
    THRESHOLD, random_gen_quality, Statement, Issue};

use serde::{Serialize, Deserialize};

const COMPLETION_RATE: f64 = 0.99;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// An over-arching evaluation structure for a LearningProduct.
/// Contains MicroEvaluations.
pub struct Evaluation {
    pub id: u32,
    pub objective: Objective,

    // Before learning
    #[dummy(faker = "1..4")]
    pub current_skill: usize,

    #[dummy(faker = "4..11")]
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

impl Evaluation {
    pub fn new(
        id: u32,
        objective: Objective,
        current_skill: usize,
        desired_skill: usize,
        end_skill: usize,
        comments: String,
        date_stamp: String,
    ) -> Self {
        Evaluation {
            id: id,
            objective: objective,
            current_skill: current_skill,
            desired_skill: desired_skill,
            end_skill: end_skill,
            comments: comments,
            date_stamp: date_stamp,
            micro_evaluations: Vec::new(),
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Short, focused evaluations for Modules within a LearningProduct.
/// Contains several optional evaluations depending on the specific 
/// Module.
pub struct MicroEvaluation {
    pub id: u32,

    #[dummy(faker = "1..11")]
    pub module: usize,    

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    pub rapid_response: Option<RapidResponse>,

    // Hashmap of a LearningProductive for a module mapped to an 
    // assessment of how the module met that objective
    pub learning_obj_eval: Option<Vec<(LearningObjective, LearningObjectiveResponse)>>,
    pub physical_eval: Option<PhysicalEval>,
    pub digital_eval: Option<DigitalEval>,
    pub personnel_eval: Option<PersonnelEval>,
    pub sent: bool,
    pub seen: bool,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, Copy)]
/// Learner assessment of whether a Module met a specific 
/// LearningProductive.
pub enum LearningObjectiveResponse {
    NotMeet,
    Meet,
    Exceeded,
}

impl MicroEvaluation {
    pub fn generate_micro_eval(id: u32, module: &Module, learner_openness: f64, learner_exclusion: f64, date_stamp: String) -> MicroEvaluation {

        // get mock module qualities
        
        let mut module_qualities = [
            module.mock_quality,
            module.mock_clear,
            module.mock_entertaining,
            module.mock_relevant,
            module.mock_infomative,
            module.mock_useful,
            module.mock_inclusive,
            module.mock_difficulty,
            module.mock_length,
        ];

        // Modify for issues in module
        for issue in &module.issues {
            match issue {
                Issue::Clear => module_qualities[1] -= 0.2,
                Issue::Entertaining => module_qualities[2] -= 0.2,
                Issue::Relevant => module_qualities[3] -= 0.2,
                Issue::Informative => module_qualities[4] -= 0.2,
                Issue::Useful => module_qualities[5] -= 0.2,
                Issue::Inclusive => module_qualities[6] -= 0.2,
            }
        };

        // Modify sense of inclusion based on mock_learner_exclusion
        // If you've never been discriminated against, you probably don't see 
        // the problem
        if learner_exclusion > 0.2 {
            module_qualities[6] = module_qualities[6] - learner_exclusion;
        };

        // set Rng

        let mut rng = rand::thread_rng();

        // set basic variables for MicroEvaluation completion

        let sent: bool = true;
        let mut seen: bool = true;
        let mut completed: bool = true;

        /*
        // Determine whether evaluation is completed or not
        if rng.gen_range(0.01, 1.0) < 0.2 {
            seen = false;
        };

        if rng.gen_range(0.01, 1.0) > COMPLETION_RATE {
            completed = false;
        };
        */

        let mut rr: Option<RapidResponse> = None;
        
        if completed {
            rr = Some(RapidResponse::generate_response(&module_qualities, learner_openness));
        };

        let mut lo: Option<Vec<(LearningObjective, LearningObjectiveResponse)>> = None;
        
        let mut learning_obj: Vec<(LearningObjective, LearningObjectiveResponse)> = Vec::new();
        
        for l in &module.learning_objectives {
            let target = learner_openness * module.mock_quality;
            
            let n = rng.gen_range(0.01, 1.0);
            
            if n < target - 0.4 {
                learning_obj.push((l.clone(), LearningObjectiveResponse::Exceeded));
            } else if n > target - 0.4 && n < target + 0.3 {
                learning_obj.push((l.clone(), LearningObjectiveResponse::Meet));
            } else {
                learning_obj.push((l.clone(), LearningObjectiveResponse::NotMeet));
            };
        };

        if completed {
            lo = Some(learning_obj);
        };
        
        MicroEvaluation {
            id: id,
            module: module.id as usize,
            date_stamp: date_stamp,
            rapid_response: rr,
            learning_obj_eval: lo,
            physical_eval: None,
            digital_eval: None,
            personnel_eval: None,
            sent: sent,
            seen: seen,
            completed: completed,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, Copy)]
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

        let mut mean_quality: f64 = 0.0;

        for v in module_qualities {
            // learner openness determines the quality value that
            // the learner is willing to recognize
            let x = v * learner_openness;

            // could subtract THRESHOLD from x...
            mean_quality += x;


            if rng.gen_range(0.01, 1.00) < x {
                qual_responses.push(true)
            } else {
                qual_responses.push(false)
            };
        };

        mean_quality = mean_quality / module_qualities.len() as f64;

        // rating is based off overall quality, modified by learner openness + 3 with a max of 10
        // Assumption: people don't like giving bad reviews
        let mut rating = (mean_quality * 1.5) * learner_openness * 10.0 + 4.0;

        if rating > 10.0 {
            rating = 10.0;
        }
        
        let mut recommend: bool = false;

        if rating > 7.0 {
            recommend = true;
        };

        // deal with too long / short
        let mut too_long: bool = false;
        let mut too_short: bool = false;

        if module_qualities[7] < 0.5 {
            if rng.gen_range(0.01, 1.0) < 0.5 {
                too_long = true; 
            } else {
                too_short = true;
            }
        };

        // deal with too easy / hard
        let mut too_hard: bool = false;
        let mut too_easy: bool = false;

        if module_qualities[8] < 0.5 {
            if rng.gen_range(0.01, 1.0) < 0.5 {
                too_hard = true; 
            } else {
                too_easy = true;
            }
        };

        RapidResponse {
            would_recommend: recommend,
            rating_1_10: rating as usize,
            clear: qual_responses[1],
            entertaining: qual_responses[2],
            relevant: qual_responses[3],
            informative: qual_responses[4],
            useful: qual_responses[5],
            inclusive: qual_responses[6],
            too_easy: too_easy,
            too_difficult: too_hard,
            too_long: too_long,
            too_short: too_short,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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
            comfortable: qual_responses[1],
            professional: qual_responses[2],
            pleasant: qual_responses[3],
            accessible: qual_responses[4],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Optional true or false responses for evaluating Personnel
pub struct PersonnelEval {
    pub pleasant: bool,
    pub helpful: bool,
    pub professional: bool,
    pub inclusive: bool,
    pub knowledgeable: bool,
}

impl PersonnelEval {
    pub fn generate_response(module_qualities: &[f64; 5], learner_openness: f64) -> PersonnelEval {

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
            inclusive: qual_responses[3],
            knowledgeable: qual_responses[4],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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
            professional: qual_responses[1],
            accessible: qual_responses[2],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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