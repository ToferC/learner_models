use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug)]
/// An over-arching evaluation structure for a LearningObject.
/// Contains MicroEvaluations.
pub struct Evaluation {
    pub id: i64,
    pub objective: Objective,
    pub current_skill: usize,
    pub desired_skill: usize,
    pub end_skill: usize,
    pub comments: String,
    pub date_stamp: NaiveDate,
    pub micro_evaluations: Vec<MicroEvaluation>,
}

#[derive(Debug)]
/// Short, focused evaluations for Modules within a LearningObject.
/// Contains several optional evaluations depending on the specific Module.
pub struct MicroEvaluation {
    pub id: i64,
    pub module: usize,
    pub date_stamp: NaiveDate,
    pub rapid_response: RapidResponse,
    pub learning_obj_eval: HashMap<String, LearningObjectiveResponse>,
    pub physical_eval: Option<PhysicalEval>,
    pub digital_eval: Option<DigitalEval>,
    pub personnel_eval: Option<PersonnelEval>,
    pub sent: bool,
    pub seen: bool,
    pub completed: bool,
}

#[derive(Debug)]
/// Learner assessment of whether a Module met a specific LearningObjective.
pub enum LearningObjectiveResponse {
    NotMeet,
    Meet,
    Exceeded,
}

#[derive(Debug)]
/// Core component of MicroEvaluation with two mandatory and an array of optional 
/// true or false responses.
pub struct RapidResponse {
    pub would_recommend: bool,
    pub rating_1_10: usize,

    // Positive
    pub accessible: bool,
    pub clear: bool,
    pub entertaining: bool,
    pub relevant: bool,
    pub informative: bool,
    pub insightful: bool,
    pub useful: bool,

    // Negative
    pub too_easy: bool,
    pub too_difficult: bool,
    pub too_long: bool,
    pub too_short: bool,
}

#[derive(Debug)]
/// Optional true or false responses for evaluating PhysicalInfrastructure
pub struct PhysicalEval {
    pub module: usize,
    pub clean: bool,
    pub pleasant: bool,
    pub comfortable: bool,
    pub professional: bool,
    pub accessible: bool,
}

#[derive(Debug)]
/// Optional true or false responses for evaluating Personnel
pub struct PersonnelEval {
    pub module: usize,
    pub pleasant: bool,
    pub helpful: bool,
    pub professional: bool,
}

#[derive(Debug)]
/// Optional true or false responses for evaluating DigitalInfrastructure
pub struct DigitalEval {
    pub smooth: bool,
    pub accessible: bool,
    pub professional: bool,
}

#[derive(Debug)]
/// The Learner's primary reason for seeking learning. Serves as context for
/// other evaluations.
pub enum Objective {
    ImproveCurrentSkills,
    NewJobPromotional,
    NewJobLateral,
    CareerChange,
    PersonalInterest,
    MandatoryLearning,
    Other ( String ),
}