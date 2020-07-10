mod models;

use chrono::prelude::*;

use models::{Learner, User, UserRole};

fn main() {

    let u = User {
        id: 0001,
        last_name: String::from("Decibel"),
        first_name: String::from("Danielle"),
        work_email_addresses: vec!(String::from("work_dd@email.com")),
        alternate_email_addresses: vec!(String::from("home_dd@email.com")),
        user_role: UserRole::Client,
        activated_on: NaiveDateTime::parse_from_str(
            "2020-01-01 23:12:04", 
            "%Y-%m-%d %H:%M:%S").unwrap(),
        deactivated_on: NaiveDateTime::parse_from_str(
            "2020-01-01 23:12:04", 
            "%Y-%m-%d %H:%M:%S").unwrap(),
    };

    let l = Learner {
        id: 0100044,
        user: u,
        badges: vec!(),
        demographics: vec!(),
        employment_status: vec!(),
        experiences: vec!(),
        data_access_log: vec!(),
    };

    println!("{:?}", l);
}
