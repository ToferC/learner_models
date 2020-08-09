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
        10_000,
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

    let p1 = Personnel::new(100, String::from("Alpha"), String::from("Alka"), 0.55, DeliveryRole::Facilitator, Group::EC, 6, 90_000, 1);
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
        Status::Production,
    );

    // Add modules

    let mut lp2_m1 = Module::new(
        101, 
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
        101, 
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
        Status::Production,
    );

    // Add module

    let mut lp3_m1 = Module::new(
        101, 
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

    // Add learning products to vec
    let mut learning_products: Vec<LearningProduct> = Vec::new();

    learning_products.push(lp1.clone());
    learning_products.push(lp2.clone());
    learning_products.push(lp3.clone());

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

    let mut digi_eval: Vec<DigitalEval> = Vec::new();

    let mut phys_eval: Vec<PhysicalEval> = Vec::new();

    let mut pers_eval: Vec<PersonnelEval> = Vec::new();

    // Create offerings
    for i in 1..10 {
        let o = Offering::new(
            777 + i, 
            100,
            String::from(format!("2020-06-0{}", i)),
            false, 
            true,
        );

        offerings.push(o);
    }

    // create learners and registrations to offerings
    let mut wtr = csv::Writer::from_path("data/evals.csv").unwrap();

    println!("generate offerings");

    for (i, o) in offerings.iter_mut().enumerate() {

        for p in 1..500 {
            // create learners
            
            let mut l: Learner = Faker.fake();
            
            let l_id = i as u32 * 1000 + p;
            l.id = l_id;

            println!("generate learner {}", l_id);
            
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

            for (n, module) in lp1.modules.iter().enumerate() {

                let me = MicroEvaluation::generate_micro_eval(
                    100 + n as u32,
                    &module, 
                    random_gen_quality(l.mock_learner_openness.to_owned()), 
                    String::from(format!("2020-06-0{}", i)),
                );
    
                // deal with potential uncompleted evaluation
                let resp = &me.rapid_response.unwrap();

                // Handle code here

                e.micro_evaluations.push(me.clone());

                benefit += resp.rating_1_10 as f64 / 10.0;

                let mut learning_obj_eval_results = Vec::new();

                if let Some(lo_eval) = me.learning_obj_eval.clone() {
                    for le in lo_eval {
                        learning_obj_eval_results.push(le.1);
                    }
                };

                let eCSV = EvalCSV::new(
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
                    lp1.code.to_owned(),
                    me.module, 
                    me.date_stamp.to_owned(),
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
                    learning_obj_eval_results[0],
                    learning_obj_eval_results[1],
                );

                // write to CSV
                wtr.serialize(&eCSV).unwrap();
            };

            e.end_skill = (e.end_skill as f64 + benefit) as usize;

            evaluations.push(e);


            // create data for analysis

        }
        
    }
    
    println!("Close CSV");

    wtr.flush().unwrap();

    // Save micro-evals to file for review

    /*
    
    println!("Write JSON 2");
    serde_json::to_writer(&File::create("data/offerings.json").unwrap(), &offerings).unwrap();
    
    println!("Write JSON 3");
    serde_json::to_writer(&File::create("data/registrations.json").unwrap(), &registrations).unwrap();
    
    println!("Write JSON 4");
    serde_json::to_writer(&File::create("data/evaluations.json").unwrap(), &evaluations).unwrap();
        
    println!("Write JSON 1");
    serde_json::to_writer(&File::create("data/learners.json").unwrap(), &learners).unwrap();
    
    */

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
    pub module: usize, 
    pub date_stamp: String,

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
    pub lo_1: LearningObjectiveResponse, 
    pub lo_2: LearningObjectiveResponse,
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
        module: usize, 
        date_stamp: String,
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
        lo_1: LearningObjectiveResponse, 
        lo_2: LearningObjectiveResponse,
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
            module: module, 
            date_stamp: date_stamp,
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
            lo_1: lo_1, 
            lo_2: lo_2,
        }
    }
}
