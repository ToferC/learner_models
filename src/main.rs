mod models;

use models::{Learner, Location, User, UserRole};

fn main() {

    let ott = Location {
        name: String::from("Ottawa"),
        address: String::from("278 Sussex"),
        city: String::from("Ottawa"),
        province: String::from("ON"),
        timezone_offset: 0,
    };

    let u = User {
        id: 0001,
        last_name: String::from("Decibel"),
        first_name: String::from("Danielle"),
        email_address: String::from("dd@email.com"),
        user_role: UserRole::Client,
    };

    let l = Learner {
        user: u,
        location: ott,
        demographics: vec!(),
        employment_status: vec!(),
        experiences: vec!(),
        data_access_log: vec!(),
    };

    println!("{:?}", l);
}
