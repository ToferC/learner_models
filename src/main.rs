mod models;

use chrono::prelude::*;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

use std::fs::File;

use fake::{Faker, Fake};

use models::{Learner, Registration, Location, 
    PhysicalInfrastructure, test_plot,
    DigitalInfrastructure, Personnel, Group, DeliveryRole,
    LearningProduct, Module, Audience, Role, BusinessLine,
    Status, LearningStyle, LearningObjective, ContentType,
    Statement, Verb, Offering, Evaluation, MicroEvaluation,
    random_gen_quality, LearningObjectiveResponse,
    Objective, Organization};

use models::evaluation::{DigitalEval, PhysicalEval, PersonnelEval};
use models::demographic::{Pronouns, Sexuality, Ethnicity};

const GENERATE_DATA: bool = false;

fn main() {

    // Create vec of potential locations
    let mut locations = Vec::new();

    let pdp = Location::default();

    let lsl = Location::new(
        400,
        286,
        String::from("Sussex"),
        String::from("Ottawa"),
        String::from("Ontario"),
        0,
    );

    locations.push(pdp);
    locations.push(lsl);

    // Create vec of physical infrastructures
    let room1 = PhysicalInfrastructure::new(
        1001,
        100,
        String::from("Room 1"),
        24,
        0.3,
    );

    let room2 = PhysicalInfrastructure::new(
        1002,
        200,
        String::from("Room 2"),
        24,
        0.6,
    );

    let room3 = PhysicalInfrastructure::new(
        2000,
        400,
        String::from("Room 3"),
        100,
        0.8,
    );

    let mut physical_inf = Vec::new();
    physical_inf.push(room1);
    physical_inf.push(room2);
    physical_inf.push(room3);


    println!("{:?}", locations);
    println!("{:?}", physical_inf);

    // create vec of digital infrastructure
    let mut digi_inf = Vec::new();

    let d1 = DigitalInfrastructure::new(
        100,
        String::from("Discover Data Video 1"),
        0.45,
        1000,
        10_000,
        0.7,
    );

    let d2 = DigitalInfrastructure::new(
        101,
        String::from("Discover Data Discussion"),
        0.45,
        1000,
        100,
        0.5,
    );

    let d3 = DigitalInfrastructure::new(
        311,
        String::from("Return to work Webcast"),
        0.45,
        1000,
        10_000,
        0.8,
    );

    digi_inf.push(d1);
    digi_inf.push(d2);
    digi_inf.push(d3);

    // Create vec of personnel
    let mut personnel = Vec::new();

    let p1 = Personnel::new(100, String::from("Alice"), String::from("Alpha"), 0.55, DeliveryRole::Facilitator, Group::EC, 6, 90_000, 1);
    let p2 = Personnel::new(102, String::from("Beatrice"), String::from("Beta"), 0.78, DeliveryRole::Operations, Group::AS, 4, 60_000, 2);
    let p3 = Personnel::new(103, String::from("Dorothy"), String::from("Delta"), 0.90, DeliveryRole::Producer, Group::IS, 6, 95_000, 3);

    personnel.push(p1);
    personnel.push(p2);
    personnel.push(p3);

    // Create Learning Products

    // Product 1 - online learning

    let mut lp1 = LearningProduct::new(
        100, 
        String::from("Discover Data"),
        String::from("I501"),
        String::from("Discover data is..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#DiscoverData"), 
        BusinessLine::DigitalAcademy,
        500,
        9,
        Status::Pilot,
    );

    // Add modules

    let mut lp1_m1 = Module::new(
        101, 
        String::from("I501-1"), 
        String::from("Intro to data"), 
        String::from("Data is the lifeblood of any organization..."), 
        vec![
            LearningStyle::Watch,
            ], 
        ContentType::Asyncronous, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("what data is and how it works"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("How data is used in the GC"),
                0.3)
            ),
            ], 
        90, 
        0.7,
    );

    
    lp1_m1.digital_infrastructure_id = Some(0);
    
    let mut lp1_m2 = Module::new(
        102, 
        String::from("I501-2"), 
        String::from("Data in Your Organization"), 
        String::from("Each department has its own strengths and..."), 
        vec![
            LearningStyle::Discuss,
            ], 
            ContentType::OnlineFacilitated, 
            vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("Examples of the digital standards in use"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Explain,
                String::from("Why applications of the standards make things better."),
                0.3)
            ),
            ], 
            45, 
            0.8,
        );
        
    // Add digital infrastructure index reference
    lp1_m2.digital_infrastructure_id = Some(1);
    
    // Add personnel index reference
    lp1_m2.personnel_ids = Some(vec![1]);
    
    lp1.modules.push(lp1_m1);
    lp1.modules.push(lp1_m2);
    
    
    // Product 2 - in-person learning
    // In person class

    let mut lp2 = LearningProduct::new(
        200, 
        String::from("Intro to Security"),
        String::from("P901"),
        String::from("Security is everyone's business..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#P901"), 
        BusinessLine::GCSkills,
        24,
        9,
        Status::Production,
    );

    // Add modules

    let mut lp2_m1 = Module::new(
        201, 
        String::from("P901-1"), 
        String::from("Security Policy in the GC"), 
        String::from("A solid understanding of the GC policy frame..."), 
        vec![
            LearningStyle::Classroom,
            ], 
        ContentType::InPersonFacilitated, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("The GC policy framwork around security"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("Roles and responsibilities on security"),
                0.3)
            ),
            ], 
        90, 
        0.55,
    );

    let mut lp2_m2 = Module::new(
        202, 
        String::from("P901-2"), 
        String::from("Physical Security Fundameentals"), 
        String::from("Security is critical in the office env..."), 
        vec![
            LearningStyle::Classroom,
            ], 
        ContentType::InPersonFacilitated, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("The key threats and mitigators for physical security"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("What to do in the case of an event"),
                0.3)
            ),
            ], 
        150, 
        0.9,
    );

    // Add physical infrastructure
    lp2_m1.physicial_infrastructure_id = Some(0);
    lp2_m2.physicial_infrastructure_id = Some(1);

    // add personnel
    lp2_m1.personnel_ids = Some(vec![0]);
    lp2_m2.personnel_ids = Some(vec![0]);

    lp2.modules.push(lp2_m1);
    lp2.modules.push(lp2_m2);

    // product 3 - event

    let mut lp3 = LearningProduct::new(
        300, 
        String::from("Returning to Work: What you need to know"),
        String::from("E311"),
        String::from("Many people have questions..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#ReturnToWork"), 
        BusinessLine::RespectfulInclusiveWorkplace, 
        3000,
        2,
        Status::Production,
    );

    // Add module

    let mut lp3_m1 = Module::new(
        301, 
        String::from("E311-1"), 
        String::from("Panel Discussion"), 
        String::from("Returning to work in the age of COVID-19..."), 
        vec![
            LearningStyle::Watch,
            ], 
        ContentType::Event, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("The GC plan for return to work"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("How to work with colleagues and corporate enablers"),
                0.3)
            ),
            ], 
        60, 
        0.90,
    );

    
    lp3_m1.digital_infrastructure_id = Some(2);
    lp3_m1.physicial_infrastructure_id = Some(2);
    lp3_m1.personnel_ids = Some(vec![2]);
    
    lp3.modules.push(lp3_m1);
    
    // Add learning products to vec
    let mut learning_products: Vec<LearningProduct> = Vec::new();

    learning_products.push(lp1.clone());
    learning_products.push(lp2.clone());
    learning_products.push(lp3.clone());

    for lp in &learning_products {
        println!("{:?}", lp);
    };

    // Create learner creation and registration loop

    // ID schema

    // physical_infrastructure_id: 1001, 1002, 2000
    // digi_infrastructure_id: 100, 101
    // personnel_id: 100, 102
    // learning_product_id: 100
    // module_id: 101, 102
    // offering_id: 777, 778
    // registration_id: 10_001 - 90_500
    // evaluation_id: 71_001 - 79_500

    // Create vecs for sim
    let mut offerings: Vec<Offering> = Vec::new();

    let mut learners: Vec<Learner> = Vec::new();

    let mut registrations: Vec<Registration> = Vec::new();

    let mut evaluations: Vec<Evaluation> = Vec::new();

    // Create offerings
    for (index, lp) in learning_products.iter().enumerate() {

        for i in 0..lp.number_of_offerings {
            let o = Offering::new(
                777 + lp.id + i as u32,
                index as u32, // would normally be lp.id, but we are looking for an index here
                String::from(format!("2020-06-0{}", i)),
                false, 
                true,
            );
    
            offerings.push(o);
        }
    };

    // create learners and registrations to offerings
    let mut wtr = csv::Writer::from_path("data/evals.csv").unwrap();

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
                    random_gen_quality(l.mock_learner_openness.to_owned()), 
                    String::from(format!("2020-06-0{}", i)),
                );
    
                // deal with potential uncompleted evaluation
                let resp = &me.rapid_response.unwrap();

                // create physical evaluation if needed

                // Add physical_eval
                let mut pe_results: Vec<bool> = Vec::new();

                if let Some(p_id) = module.physicial_infrastructure_id {

                    let pi = physical_inf[p_id as usize].clone();
                    
                    let phys_qualities = [
                        pi.mock_cleanliness,
                        pi.mock_comfort,
                        pi.mock_professional,
                        pi.mock_pleasant,
                        pi.mock_accessible,
                    ];

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
                    
                    let digi_qualities = [
                        pi.mock_smooth,
                        pi.mock_professional,
                        pi.mock_accessible,
                    ];

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
                    
                    let personnel_qualities = [
                        pi.mock_pleasant,
                        pi.mock_helpful,
                        pi.mock_professionalism,
                        pi.mock_inclusive,
                        pi.mock_knowledgeable,
                    ];

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
                let percent_completed = (random_gen_quality(module.mock_quality).min(1.0) * 10.0).round() / 10.0;

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
                    l.demographics.ethnicicty.clone(),
                    l.demographics.person_with_disability,
                    r.id,
                    o.id,
                    learning_products[o.learning_product_id as usize].code.to_owned(),
                    learning_products[o.learning_product_id as usize].business_line.clone(),
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

    // Save micro-evals to file for review

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
    

    // Plot results
    /*
    let p = test_plot();
    
    let p = match p {
        Ok(_plot) => println!("Plot complete"),
        Err(error) => panic!("Problem plotting: {:}", error),
    };
    */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvalCSV {
    pub eval_id: u32,

    // Learner
    pub learner_id: u32,
    pub audience: Audience,
    pub group: Group,
    pub level: usize,
    pub role: Role,
    pub organization: Organization,
    pub pronouns: Pronouns,
    pub sexuality: Sexuality,
    pub ethnicity: Ethnicity,
    pub person_with_disability: bool,

    // Product
    pub registration_id: u32,
    pub offering_id: u32,
    pub learning_product: String,
    pub business_line: BusinessLine,
    pub module: usize, 
    pub date_stamp: String,
    pub objective: Objective,

    // Skill
    pub current_skill: u32,
    pub desired_skill: u32,
    pub final_skill: u32,

    // Evaluation
    pub recommend: bool, 
    pub rating: usize, 
    pub clear: bool, 
    pub entertaining: bool,
    pub relevant: bool, 
    pub informative: bool, 
    pub useful: bool, 
    pub inclusive: bool, 
    pub easy: bool, 
    pub difficult: bool, 
    pub long: bool, 
    pub short: bool,
    pub percent_completed: f64,
    pub lo_1: LearningObjectiveResponse, 
    pub lo_2: LearningObjectiveResponse,

    // Physical Eval
    pub physical_space: bool,
    pub physical_clean: bool,
    pub physical_comfortable: bool,
    pub physical_professional: bool,
    pub physical_pleasant: bool,
    pub physical_accessible: bool,

    // Digital Eval
    pub digital_content: bool,
    pub digital_smooth: bool,
    pub digital_professional: bool,
    pub digital_accessible: bool,

    // Personnel Eval
    pub personnel_present: bool,
    pub personnel_pleasant: bool,
    pub personnel_helpful: bool,
    pub personnel_professional: bool,
    pub personnel_inclusive: bool,
    pub personnel_knowledgeable: bool,
}

impl EvalCSV {
    pub fn new(
        eval_id: u32,
        learner_id: u32,
        audience: Audience,
        group: Group,
        level: usize,
        role: Role,
        organization: Organization,
        pronouns: Pronouns,
        sexuality: Sexuality,
        ethnicity: Ethnicity,
        person_with_disability: bool,
        registration_id: u32,
        offering_id: u32,
        learning_product: String,
        business_line: BusinessLine,
        module: usize, 
        date_stamp: String,
        objective: Objective,
        current_skill: u32,
        desired_skill: u32,
        final_skill: u32,
        recommend: bool, 
        rating: usize, 
        clear: bool, 
        entertaining: bool,
        relevant: bool, 
        informative: bool, 
        useful: bool, 
        inclusive: bool, 
        easy: bool, 
        difficult: bool, 
        long: bool, 
        short: bool, 
        percent_completed: f64,
        lo_1: LearningObjectiveResponse, 
        lo_2: LearningObjectiveResponse,
        // Physical Eval
        physical_space: bool,
        physical_clean: bool,
        physical_comfortable: bool,
        physical_professional: bool,
        physical_pleasant: bool,
        physical_accessible: bool,

        // Digital Eval
        digital_content: bool,
        digital_smooth: bool,
        digital_professional: bool,
        digital_accessible: bool,

        // Personnel Eval
        personnel_present: bool,
        personnel_pleasant: bool,
        personnel_helpful: bool,
        personnel_professional: bool,
        personnel_inclusive: bool,
        personnel_knowledgeable: bool,
    ) -> Self {
        EvalCSV {
            eval_id: eval_id,
            learner_id: learner_id,
            audience: audience,
            group: group,
            level: level,
            role: role,
            organization: organization,
            pronouns: pronouns,
            sexuality: sexuality,
            ethnicity: ethnicity,
            person_with_disability: person_with_disability,
            registration_id: registration_id,
            offering_id: offering_id,
            learning_product: learning_product,
            business_line: business_line,
            module: module, 
            date_stamp: date_stamp,
            objective: objective,
            current_skill: current_skill,
            desired_skill: desired_skill,
            final_skill: final_skill,
            recommend: recommend, 
            rating: rating, 
            clear: clear, 
            entertaining: entertaining,
            relevant: relevant, 
            informative: informative, 
            useful: useful, 
            inclusive: inclusive, 
            easy: easy, 
            difficult: difficult, 
            long: long, 
            short: short,
            percent_completed: percent_completed, 
            lo_1: lo_1, 
            lo_2: lo_2,
            physical_space: physical_space,
            physical_clean: physical_clean,
            physical_comfortable: physical_comfortable,
            physical_professional: physical_professional,
            physical_pleasant: physical_pleasant,
            physical_accessible: physical_accessible,
            digital_content: digital_content,
            digital_smooth: digital_smooth,
            digital_professional: digital_professional,
            digital_accessible: digital_accessible,
            personnel_present: personnel_present,
            personnel_pleasant: personnel_pleasant,
            personnel_helpful: personnel_helpful,
            personnel_professional: personnel_professional,
            personnel_inclusive: personnel_inclusive,
            personnel_knowledgeable: personnel_knowledgeable,
        }
    }
}
