use chrono::prelude::*;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

use std::fs::File;

use fake::{Faker, Fake};

use crate::models::{Learner, Registration, Location, 
    PhysicalInfrastructure, test_plot,
    DigitalInfrastructure, Personnel, Group, DeliveryRole,
    LearningProduct, Module, Audience, Role, BusinessLine,
    Status, LearningStyle, LearningObjective, ContentType,
    Statement, Verb, Offering, Evaluation, MicroEvaluation,
    random_gen_quality, LearningObjectiveResponse,
    Objective, Organization};

use crate::models::evaluation::{DigitalEval, PhysicalEval, PersonnelEval};
use crate::models::learning_product::{Issue};
use crate::models::infrastructure::{PhysIssue, DigiIssue};
use crate::models::personnel::{PersonnelIssue};
use crate::models::demographic::{Pronouns, Sexuality, Ethnicity, Language};

use super::generate_csv::EvalCSV;

// round an integer down to the nearest v
fn round(i: u32, v: u32) -> u32 {
    let r = (i as f64/v as f64).floor() * v as f64;

    r as u32
}

pub fn run_simulation(
    learning_products: Vec<LearningProduct>,
    mut offerings: Vec<Offering>,
    physical_inf: Vec<PhysicalInfrastructure>,
    digi_inf: Vec<DigitalInfrastructure>,
    personnel: Vec<Personnel>,
    GENERATE_DATA: bool,
) {
    // create learners and registrations to offerings
    let mut wtr = csv::Writer::from_path("data/evals.csv").unwrap();

    // Create vecs for data
    let mut learners: Vec<Learner> = Vec::new();

    let mut registrations: Vec<Registration> = Vec::new();

    let mut evaluations: Vec<Evaluation> = Vec::new();

    println!("generate offerings");

    for (i, o) in offerings.iter_mut().enumerate() {

        for p in 0..learning_products[o.learning_product_id as usize].capacity {
            // create learners
            
            let mut l: Learner = Faker.fake();
            
            let l_id = i as u32 * 1000 + p;
            l.id = l_id;
            
            // create registration
            let mut r = Registration::new(
                i as u32 * 10_000 + p,
                l_id, 
                o.start_date.to_owned(), 
                o.id, 
                true, 
                false);
                
            learners.push(l.clone());
            registrations.push(r.clone());
            // create primary evaluation

            // csv of evaluations

            let obj: Objective = Faker.fake();

            // generate learner skills and objectives

            let base_skill = random_gen_quality(0.3);

            let target_skill = random_gen_quality(base_skill).max(&base_skill + 0.2);

            let e_id = 70000 + i as u32 * 1000 + p as u32;

            o.evaluation_ids.push(e_id);

            let mut e: Evaluation = Evaluation::new(
                e_id, 
                obj, 
                (base_skill * 10.0) as usize, 
                (target_skill * 10.0) as usize, 
                (base_skill * 10.0) as usize, 
                String::from("Interesting"), 
                String::from(format!("2020-06-0{}", i)),
            );

            // track benefit of learning
            let mut benefit: f64 = 0.0;

            // create micro-eval for each module

            for (n, module) in learning_products[o.learning_product_id as usize].modules.iter().enumerate() {

                let mut me = MicroEvaluation::generate_micro_eval(
                    100 + n as u32,
                    &module, 
                    l.mock_learner_openness.to_owned(),
                    l.mock_exclusion.to_owned(),
                    l.employment_status[0].audience.clone(),
                    String::from(format!("2020-06-0{}", i)),
                );
    
                // deal with potential uncompleted evaluation
                let resp = &me.rapid_response.unwrap();

                // create physical evaluation if needed

                // Add physical_eval
                let mut pe_results: Vec<bool> = Vec::new();

                if let Some(p_id) = module.physicial_infrastructure_id {

                    let pi = physical_inf[p_id as usize].clone();
                    
                    let mut phys_qualities = [
                        pi.mock_cleanliness,
                        pi.mock_comfort,
                        pi.mock_professional,
                        pi.mock_pleasant,
                        pi.mock_accessible,
                    ];

                    if l.demographics.person_with_disability {
                        phys_qualities[4] = phys_qualities[4] - (l.mock_exclusion * 2.0);
                    } else {
                        // people just unaware of issues
                        phys_qualities[4] += 0.20;
                    };

                    // Modify for issues in Physical
                    for issue in &pi.issues {
                        match issue {
                            PhysIssue::Cleanliness => phys_qualities[0] -= 0.2,
                            PhysIssue::Comfort => phys_qualities[1] -= 0.2,
                            PhysIssue::Professional => phys_qualities[2] -= 0.2,
                            PhysIssue::Pleasant => phys_qualities[3] -= 0.2,
                            PhysIssue::Accessible => phys_qualities[4] -= 0.2,
                        }
                    }

                    let new_p_eval = PhysicalEval::generate_response(
                        &phys_qualities,
                        l.mock_learner_openness,
                    );

                    pe_results.push(true); // There is a physical eval
                    pe_results.push(new_p_eval.clean);
                    pe_results.push(new_p_eval.comfortable);
                    pe_results.push(new_p_eval.professional);
                    pe_results.push(new_p_eval.pleasant);
                    pe_results.push(new_p_eval.accessible);

                    me.physical_eval = Some(new_p_eval);
                }else {
                    for _ in 0..7 {
                        pe_results.push(false);
                    };
                };

                // create digital evaluation if needed

                let mut de_results: Vec<bool> = Vec::new();

                if let Some(p_id) = module.digital_infrastructure_id {

                    let pi =digi_inf[p_id as usize].clone();
                    
                    let mut digi_qualities = [
                        pi.mock_smooth,
                        pi.mock_professional,
                        pi.mock_accessible,
                    ];

                    if l.demographics.person_with_disability {
                        digi_qualities[2] = digi_qualities[2] - (l.mock_exclusion * 2.0);
                    } else {
                        // people just unaware of issues
                        digi_qualities[2] += 0.20;
                    };

                    // Modify for issues in Physical
                    for issue in &pi.issues {
                        match issue {
                            DigiIssue::Smooth => digi_qualities[0] -= 0.2,
                            DigiIssue::Professional => digi_qualities[1] -= 0.2,
                            DigiIssue::Accessible => digi_qualities[2] -= 0.2,
                        }
                    }

                    let new_digi_eval = DigitalEval::generate_response(
                        &digi_qualities,
                        l.mock_learner_openness,
                    );

                    de_results.push(true); // There is a digital eval
                    de_results.push(new_digi_eval.smooth);
                    de_results.push(new_digi_eval.professional);
                    de_results.push(new_digi_eval.accessible);

                    me.digital_eval = Some(new_digi_eval);
                }else {
                    for _ in 0..4 {
                        de_results.push(false);
                    };
                };

                // create personnel evaluation if needed

                let mut pers_e_results: Vec<bool> = Vec::new();

                if let Some(p_id) = &module.personnel_ids {

                    let pi = personnel[p_id[0] as usize].clone();
                    
                    let mut personnel_qualities = [
                        pi.mock_pleasant,
                        pi.mock_helpful,
                        pi.mock_professionalism,
                        pi.mock_inclusive,
                        pi.mock_knowledgeable,
                    ];

                    // Modify for issues in Personnel
                    for issue in &pi.issues {
                        match issue {
                            PersonnelIssue::Pleasant => personnel_qualities[0] -= 0.2,
                            PersonnelIssue::Helpful => personnel_qualities[1] -= 0.2,
                            PersonnelIssue::Professional => personnel_qualities[2] -= 0.2,
                            PersonnelIssue::Inclusive => personnel_qualities[3] -= 0.2,
                            PersonnelIssue::Knowledgeable => personnel_qualities[4] -= 0.2,
                            _ => (),
                        }
                    }

                    if l.mock_exclusion > 0.2 {
                        personnel_qualities[3] = personnel_qualities[3] - l.mock_exclusion;
                    };

                    let new_pers_eval = PersonnelEval::generate_response(
                        &personnel_qualities,
                        l.mock_learner_openness,
                    );

                    pers_e_results.push(true); // There is a digital eval
                    pers_e_results.push(new_pers_eval.pleasant);
                    pers_e_results.push(new_pers_eval.helpful);
                    pers_e_results.push(new_pers_eval.professional);
                    pers_e_results.push(new_pers_eval.inclusive);
                    pers_e_results.push(new_pers_eval.knowledgeable);

                    me.personnel_eval = Some(new_pers_eval);
                } else {
                    for _ in 0..6 {
                        pers_e_results.push(false);
                    };
                };

                // add micro-evaluation to evaluation
                e.micro_evaluations.push(me.clone());

                // determine learning objectives and benefit

                benefit += resp.rating_1_10 as f64 / 10.0;

                let mut learning_obj_eval_results = Vec::new();

                if let Some(lo_eval) = me.learning_obj_eval.clone() {
                    for le in lo_eval {
                        learning_obj_eval_results.push(le.1);
                    }
                };

                // mock percent of the module that were completed by the learner
                let mut percent_completed = round((random_gen_quality(module.mock_quality).min(1.0) * 100.0) as u32, 10) as f64 / 100.0;
                let mut threshold: u32;

                // reduct % completed depending on length of module and threshold
                
                match module.content {
                    ContentType::Asyncronous => threshold = 30,
                    ContentType::Conference => threshold = 90,
                    ContentType::InPersonFacilitated => threshold = 45,
                    ContentType::InPersonUnfacilitated => threshold = 45,
                    ContentType::LearningAid => threshold = 5,
                    ContentType::OnlineFacilitated => threshold = 45,
                    ContentType::Event => threshold = 60,
                    ContentType::Video => threshold = 3,
                    ContentType::Conference => threshold = 60,
                    ContentType::Podcast => threshold = 10,
                }

                if module.duration_minutes > threshold {
                    // example: 60 - 45 = 15 / 4.5 = 3 * 5 = 15 / 100.0 = 0.15
                    let too_long_mod = round(((module.duration_minutes - threshold) as f64 / (module.duration_minutes as f64 / 10.0)) as u32 * 5, 5) as f64 / 100.0;
    
                    if module.duration_minutes > threshold {
                        percent_completed = round(((percent_completed * (1.0 - too_long_mod.min(0.3))) * 100.0) as u32, 10) as f64 / 100.0;
                    }
                };

                // create CSV
                let e_csv = EvalCSV::new(
                    e.id,
                    l.id,
                    l.employment_status[0].audience.clone(),
                    l.employment_status[0].group.clone(),
                    l.employment_status[0].level,
                    l.employment_status[0].role.clone(),
                    l.employment_status[0].organization.clone(),
                    l.demographics.pronouns.clone(),
                    l.demographics.sexuality.clone(),
                    l.demographics.ethnicity.clone(),
                    l.demographics.primary_official_language.clone(),
                    l.demographics.person_with_disability,
                    l.mock_learner_openness,
                    l.mock_exclusion,
                    r.id,
                    o.id,
                    learning_products[o.learning_product_id as usize].code.to_owned(),
                    learning_products[o.learning_product_id as usize].business_line.clone(),
                    module.content.to_owned(),
                    module.duration_minutes,
                    me.module, 
                    me.date_stamp.to_owned(),
                    e.objective.clone(),
                    e.current_skill as u32,
                    e.desired_skill as u32,
                    e.current_skill as u32 + benefit as u32,
                    resp.would_recommend, 
                    resp.rating_1_10,
                    resp.clear, 
                    resp.entertaining, 
                    resp.relevant, 
                    resp.informative,
                    resp.useful, 
                    resp.inclusive, 
                    resp.too_easy, 
                    resp.too_difficult,
                    resp.too_long, 
                    resp.too_short, 
                    percent_completed,
                    learning_obj_eval_results[0],
                    learning_obj_eval_results[1],
                    pe_results[0],
                    pe_results[1],
                    pe_results[2],
                    pe_results[3],
                    pe_results[4],
                    pe_results[5],
                    de_results[0],
                    de_results[1],
                    de_results[2],
                    de_results[3],
                    pers_e_results[0],
                    pers_e_results[1],
                    pers_e_results[2],
                    pers_e_results[3],
                    pers_e_results[4],
                    pers_e_results[5],
                );

                // write to CSV
                wtr.serialize(&e_csv).unwrap();
            };

            e.end_skill = (e.end_skill as f64 + benefit) as usize;

            evaluations.push(e);


            // create data for analysis

        }
        
    }
    
    println!("Close CSV");

    wtr.flush().unwrap();

    if GENERATE_DATA == true {
        println!("Write JSON 2");
        serde_json::to_writer(&File::create("data/offerings.json").unwrap(), &offerings).unwrap();
        
        println!("Write JSON 3");
        serde_json::to_writer(&File::create("data/registrations.json").unwrap(), &registrations).unwrap();
        
        println!("Write JSON 4");
        //serde_json::to_writer(&File::create("data/evaluations.json").unwrap(), &evaluations).unwrap();
            
        println!("Write JSON 1");
        //serde_json::to_writer(&File::create("data/learners.json").unwrap(), &learners).unwrap();
    };
}