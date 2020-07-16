use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use rand::rngs::StdRng;
use rand::SeedableRng;

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::chrono::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug)]
/// A user of the system, employee or learner. Should be tied to
/// an employee profile at the GC level.
pub struct User {
    pub id: i64,

    pub last_name: String,

    pub first_name: String,

    pub work_email_addresses: Vec<String>,
    pub alternate_email_addresses: Vec<String>,
    pub twitter_handle: String,
    pub cell_phone: String,
    pub teams_account: u32,

    pub user_role: UserRole,

    pub activated_on: NaiveDateTime,
    pub deactivated_on: NaiveDateTime,
}

impl User {
    pub fn random() -> Self {

        let lan: String = LastName(EN).fake();
        let fin: String = FirstName(EN).fake();

        User {
            id: Faker.fake::<i64>(),
            last_name: lan.clone(),
            first_name: fin.clone(),
            work_email_addresses: vec![String::from(format!("{}.{}@workmail.com", &lan, &fin))],
            alternate_email_addresses: vec![String::from(format!("{}.{}@homemail.com", lan, fin))],
            twitter_handle: Faker.fake::<String>(),
            cell_phone: PhoneNumber(EN).fake(),
            teams_account: Faker.fake::<u32>(),
            user_role: UserRole::default(),
            activated_on: DateTime(EN).fake(),
            deactivated_on: DateTime(EN).fake(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Role for user
pub enum UserRole {
    Admin,
    SuperUser,
    Client,
    DataAnalyst,
}

impl UserRole {
    fn default() -> Self {
        UserRole::Client
    }
}