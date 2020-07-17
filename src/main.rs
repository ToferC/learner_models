mod models;

use chrono::prelude::*;

use fake::{Faker, Fake};

use models::{Learner, User, DemographicData};

fn main() {

    let u = User::random();

    let d:  DemographicData = Faker.fake();

    let l = Learner {

        id: 0100044,
        user: u,
        badges: vec!(),
        demographics: d,
        employment_status: vec!(),
        experiences: vec!(),
        data_access_log: vec!(),
    };

    println!("{:?}", l);
}
