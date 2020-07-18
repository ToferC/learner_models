mod models;

use chrono::prelude::*;

use fake::{Faker, Fake};

use models::{Learner, User, DemographicData};

fn main() {

    let l: Learner = Faker.fake();

    println!("{:?}", l);
}
