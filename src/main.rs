mod models;
mod generators;

use chrono::prelude::*;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

use std::fs::File;

use fake::{Faker, Fake};

use models::{test_plot, Offering};

use generators::generate_learning_products::generate_learning_products;
use generators::generate_infrastructure::{generate_physical_inf, generate_digi_inf, generate_personnel};
use generators::simulation::run_simulation;
use generators::generate_offerings::generate_offerings;

const GENERATE_DATA: bool = false;

fn main() {

    // Create vec of potential locations

    let physical_inf = generate_physical_inf();

    let digi_inf = generate_digi_inf();

    let personnel = generate_personnel();

    // Create Learning Products

    let mut learning_products = generate_learning_products(&physical_inf, &digi_inf, &personnel);

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
    let offerings = generate_offerings(&learning_products);

    // Run core simulation
    run_simulation(learning_products, offerings, physical_inf, digi_inf, personnel, GENERATE_DATA);    

    // Plot results
    /*
    let p = test_plot();
    
    let p = match p {
        Ok(_plot) => println!("Plot complete"),
        Err(error) => panic!("Problem plotting: {:}", error),
    };
    */
}
