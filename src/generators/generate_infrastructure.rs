use crate::models::{Location, 
    PhysicalInfrastructure, PhysIssue, DigiIssue, PersonnelIssue,
    DigitalInfrastructure, Personnel, Group, DeliveryRole,
    LearningProduct, Module, Audience, Role, BusinessLine,
    Status, LearningStyle, LearningObjective, ContentType, LearningObjectiveResponse,
    Objective, Organization};

pub fn generate_physical_inf() -> Vec<PhysicalInfrastructure> {

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
        vec![PhysIssue::Comfort],
    );

    let room2 = PhysicalInfrastructure::new(
        1002,
        200,
        String::from("Room 2"),
        24,
        0.6,
        vec![PhysIssue::Cleanliness],
    );

    let room3 = PhysicalInfrastructure::new(
        2000,
        400,
        String::from("Room 3"),
        100,
        0.8,
        vec![PhysIssue::Accessible],
    );

    let mut physical_inf = Vec::new();
    physical_inf.push(room1);
    physical_inf.push(room2);
    physical_inf.push(room3);


    println!("{:?}", locations);
    println!("{:?}", physical_inf);

    physical_inf
}

pub fn generate_digi_inf() -> Vec<DigitalInfrastructure> {

    // create vec of digital infrastructure
    let mut digi_inf = Vec::new();

    let d1 = DigitalInfrastructure::new(
        100,
        String::from("Discover Data Video 1"),
        0.45,
        1000,
        10_000,
        0.7,
        vec![DigiIssue::Smooth],
    );

    let d2 = DigitalInfrastructure::new(
        101,
        String::from("Discover Data Discussion"),
        0.45,
        1000,
        100,
        0.5,
        vec![DigiIssue::Professional],
    );

    let d3 = DigitalInfrastructure::new(
        311,
        String::from("Return to work Webcast"),
        0.45,
        1000,
        10_000,
        0.8,
        vec![DigiIssue::Accessible],
    );

    digi_inf.push(d1);
    digi_inf.push(d2);
    digi_inf.push(d3);

    digi_inf

}

pub fn generate_personnel() -> Vec<Personnel> {
    // Create vec of personnel
    let mut personnel = Vec::new();

    // Faculty for P901
    let p1 = Personnel::new(
        100, 
        String::from("Alice"), 
        String::from("Alpha"), 
        0.55, 
        DeliveryRole::Facilitator, 
        Group::EC, 
        6, 
        90_000, 
        1,
        vec![PersonnelIssue::Helpful],
    );
    
    // Faculty for Discover Digital - module 102
    let p2 = Personnel::new(
        102, 
        String::from("Beatrice"), 
        String::from("Beta"), 
        0.78, 
        DeliveryRole::Operations, 
        Group::AS, 
        4, 
        60_000, 
        2,
        vec![PersonnelIssue::Inclusive],
    );

    // Faculty for E311
    let p3 = Personnel::new(
        103, 
        String::from("Dorothy"), 
        String::from("Delta"), 
        0.90, 
        DeliveryRole::Speaker, 
        Group::IS, 
        6, 
        95_000, 
        3,
        vec![PersonnelIssue::Professional],
    );


    personnel.push(p1);
    personnel.push(p2);
    personnel.push(p3);

    personnel
}