mod models;

use models::{Learner, Location};

fn main() {

    let ott = Location {
        name: String::from("Ottawa"),
        address: String::from("278 Sussex"),
        city: String::from("Ottawa"),
        province: String::from("ON"),
        timezone_offset: 0,
    };

    let l = Learner {
        id: 0001,
        last_name: String::from("Decibel"),
        first_name: String::from("Danielle"),
        location: ott,
        demographics: vec!(),
        employment_status: vec!(),
        experiences: vec!(),
    };

    println!("{:?}", l);
}
