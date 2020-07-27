mod models;

use chrono::prelude::*;
use serde_json::{Result, Value};

use std::fs::File;
use std::collections::HashMap;

use fake::{Faker, Fake};

use models::{Learner, Registration, Location, PhysicalInfrastructure};

fn main() {

    // Create vec of potential locations
    let mut locations = HashMap::new();

    let pdp = Location::default();

    let lsl = Location::new(
        400,
        286,
        String::from("Sussex"),
        String::from("Ottawa"),
        String::from("Ontario"),
        0,
    );

    locations.insert(300, pdp);
    locations.insert(400, lsl);

    // Create vec of physical infrastructures
    let room1 = PhysicalInfrastructure::default();

    let room2 = PhysicalInfrastructure::new(
        1001,
        300,
        String::from("Room 2"),
        24,
        0.3,
    );

    let room3 = PhysicalInfrastructure::new(
        2000,
        400,
        String::from("Room 3"),
        100,
        0.8,
    );

    let mut physical_inf = HashMap::new();
    physical_inf.insert(1000, room1);
    physical_inf.insert(1001, room2);
    physical_inf.insert(2000, room3);




    //let l: Learner = Faker.fake();

    let r: Registration = Faker.fake();

    //println!("{:?}", l);

    serde_json::to_writer(&File::create("test.json").unwrap(), &r);
    //println!("{:?}", serialized);
}
