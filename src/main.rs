mod models;

use chrono::prelude::*;
use serde_json::{Result, Value};

use std::fs::File;
use std::collections::HashMap;

use fake::{Faker, Fake};

use models::{Learner, Registration, Location, 
    PhysicalInfrastructure, test_plot,
    DigitalInfrastructure, Personnel, Group, DeliveryRole,
    LearningProduct, Module, Audience, Role, BusinessLine,
    Status, LearningStyle, LearningObjective, ContentType,
    Statement, Verb, Offering, Evaluation, MicroEvaluation};

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

    digi_inf.push(d1);
    digi_inf.push(d2);

    // Create vec of personnel
    let mut personnel = Vec::new();

    let p1 = Personnel::new(100, String::from("Alpha"), String::from("Alka"), 0.55, DeliveryRole::Facilitator, Group::EC, 6, 90_000, 1);
    let p2 = Personnel::new(102, String::from("Beatrice"), String::from("Beta"), 0.78, DeliveryRole::Operations, Group::AS, 4, 60_000, 2);

    personnel.push(p1);
    personnel.push(p2);

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

    /*
    // Product 2 - in-person learning

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

    */

    // Create learner creation and registration loop

    // Create vecs for sim
    let mut offerings: Vec<Offering> = Vec::new();

    let mut learners: Vec<Learner> = Vec::new();

    let mut registrations: Vec<Registration> = Vec::new();

    let mut evaluations: Vec<Evaluation> = Vec::new();

    // Create offerings

    for i in 1..10 {
        let o = Offering::new(
            777 + i, 
            100,
            888 + i, 
            String::from(format!("2020-06-0{}", i)),
            false, 
            true,
        );

        offerings.push(o);
    }

    // create learners and registrations to offerings

    for (i, o) in offerings.iter().enumerate() {

        for p in 1..500 {
            // create learners
            let mut l: Learner = Faker.fake();

            let l_id = i as u32 * 1000 + p;
            l.id = l_id;

            learners.push(l);


            // create registration
            let mut r = Registration::new(
                i as u32 * 10_000 + p,
                l_id, 
                o.start_date.to_owned(), 
                o.id, 
                true, 
                false);

            // create evaluations
            

        }

    }

    println!("EVALUATION");
    
    let me = MicroEvaluation::generate_micro_eval(100, &lp1_m1, 0.55, String::from("2020-06-01"));

    println!("{:?}", me);


    // Push evaluation data into vecs


    //let l: Learner = Faker.fake();

    let r: Registration = Faker.fake();

    //println!("{:?}", l);

    serde_json::to_writer(&File::create("test.json").unwrap(), &r);
    //println!("{:?}", serialized);

    // Plot results
    let p = test_plot();
    
    let p = match p {
        Ok(_plot) => println!("Plot complete"),
        Err(error) => panic!("Problem plotting: {:}", error),
    };
}
