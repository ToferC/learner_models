mod models;

use chrono::prelude::*;
use serde_json::{Result, Value};

use std::fs::File;

use fake::{Faker, Fake};

use models::{Learner, Registration};

fn main() {

    //let l: Learner = Faker.fake();

    let r: Registration = Faker.fake();

    //println!("{:?}", l);

    serde_json::to_writer(&File::create("test.json").unwrap(), &r);
    //println!("{:?}", serialized);
}
