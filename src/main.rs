mod models;

use chrono::prelude::*;

use fake::{Faker, Fake};

use models::{Learner, User, DemographicData, Registration};

fn main() {

    let l: Learner = Faker.fake();

    let r: Registration = Faker.fake();

    println!("{:?}", l);

    println!("{:?}", r);
}
