use chrono::prelude::*;

#[derive(Debug)]
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
pub struct MicroEvaluation {
    pub id: i64,
    pub date_stamp: NaiveDate,
    pub rapid_response: RapidResponse,
    pub physical_eval: Option<PhysicalEval>,
    pub digital_eval: Option<DigitalEval>,
    pub personnel_eval: Option<PersonnelEval>,
}

#[derive(Debug)]
pub struct RapidResponse {
    pub met_learning_objective: bool,
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
pub struct PhysicalEval {
    pub clean: bool,
    pub pleasant: bool,
    pub comfortable: bool,
    pub professional: bool,
    pub accessible: bool,
}

#[derive(Debug)]
pub struct PersonnelEval {
    pub clean: bool,
    pub pleasant: bool,
    pub helpful: bool,
    pub professional: bool,
}

#[derive(Debug)]
pub struct DigitalEval {
    pub smooth: bool,
    pub accessible: bool,
    pub professional: bool,
}

#[derive(Debug)]
pub enum Objective {
    ImproveCurrentSkills,
    NewJob_Promotional,
    NewJob_Lateral,
    CareerChange,
    PersonalInterest,
    MandatoryLearning,
    Other ( String ),
}