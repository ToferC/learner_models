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
    Status};

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
        String::from("Discover Data Video 2"),
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

    let mut lp1 = LearningProduct::new(
        100, 
        String::from("Discover Digital"), 
        String::from("Discover digital is..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#DiscoverDigital"), 
        BusinessLine::DigitalAcademy, 
        Status::Pilot,
    );

    // Add modules

    let mut lp2 = LearningProduct::new(
        100, 
        String::from("Intro to Security"), 
        String::from("Security is everyone's business..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#P901"), 
        BusinessLine::GCSkills, 
        Status::Production,
    );

    // Add modules

    // Create learner loop


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
