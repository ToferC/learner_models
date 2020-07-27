mod models;

use chrono::prelude::*;
use serde_json::{Result, Value};

use rand::distributions::{Normal, Distribution};

use std::fs::File;
use std::collections::HashMap;

use fake::{Faker, Fake};

use models::{Learner, Registration, Location, PhysicalInfrastructure};

fn random_gen_quality(quality: f64) -> f64 {

    let normal = Normal::new(quality as f64, 2.0);
    let v = normal.sample(&mut rand::thread_rng());

    v

}

fn main() {

    // Create vec of potential locations
    let mut locations = HashMap::new();

    let pdp = Location {
        id: 300,
        street_number: 186,
        address: String::from("Place du Portage"),
        city: String::from("Gatineau"),
        province: String::from("Quebec"),
        timezone_offset: 0,
    };

    let lsl = Location {
        id: 400,
        street_number: 286,
        address: String::from("Sussex"),
        city: String::from("Ottawa"),
        province: String::from("Ontario"),
        timezone_offset: 0,
    };

    locations.insert(300, pdp);
    locations.insert(400, lsl);

    // Create vec of physical infrastructures
    let room1 = PhysicalInfrastructure {
        id: 1000,
        location_id: 300,
        opening_hours: String::from("9h00"),
        closing_hours: String::from("17h00"),
        capacity: 24,
        wifi: Some(30),
        cost_per_hour: 300.0,
        map_url: String::from("map_url_one"),

        mock_quality: 0.3,
        mock_accessible: random_gen_quality(0.3),
        mock_comfort: random_gen_quality(0.3),
        mock_cleanliness: random_gen_quality(0.3),
        mock_pleasant: random_gen_quality(0.3),
        mock_professional: random_gen_quality(0.3),
    };

    let room2 = PhysicalInfrastructure {
        id: 1001,
        location_id: 300,
        opening_hours: String::from("9h00"),
        closing_hours: String::from("17h00"),
        capacity: 24,
        wifi: Some(30),
        cost_per_hour: 300.0,
        map_url: String::from("map_url_one"),

        mock_quality: 0.6,
        mock_accessible: random_gen_quality(0.6),
        mock_comfort: random_gen_quality(0.6),
        mock_cleanliness: random_gen_quality(0.6),
        mock_pleasant: random_gen_quality(0.6),
        mock_professional: random_gen_quality(0.6),
    };

    let room3 = PhysicalInfrastructure {
        id: 2000,
        location_id: 400,
        opening_hours: String::from("9h00"),
        closing_hours: String::from("17h00"),
        capacity: 24,
        wifi: Some(30),
        cost_per_hour: 300.0,
        map_url: String::from("map_url_one"),

        mock_quality: 0.8,
        mock_accessible: random_gen_quality(0.8),
        mock_comfort: random_gen_quality(0.8),
        mock_cleanliness: random_gen_quality(0.8),
        mock_pleasant: random_gen_quality(0.8),
        mock_professional: random_gen_quality(0.8),
    };

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
