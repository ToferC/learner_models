use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use rand::{SeedableRng, Rng};

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::chrono::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A user of the system, employee or learner. Should be tied to
/// an employee profile at the GC level.
pub struct User {
    pub id: i64,

    #[dummy(faker = "Buzzword(EN)")]
    pub user_name: String,

    #[dummy(faker = "(Faker, 2..4)")]
    pub work_email_addresses: Vec<String>,

    #[dummy(faker = "(Faker, 2..4)")]
    pub alternate_email_addresses: Vec<String>,

    #[dummy(faker = "CatchPhase(EN)")]
    pub twitter_handle: String,

    #[dummy(faker = "PhoneNumber(EN)")]
    pub cell_phone: String,

    pub teams_account: u32,

    pub user_role: UserRole,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub activated_on: String,
    pub deactivated_on: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Role for user
pub enum UserRole {
    Admin,
    SuperUser,
    Client,
    DataAnalyst,
}

impl Dummy<Faker> for UserRole {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..7).fake_with_rng(rng);
        
        match i {
            0 => UserRole::Client,
            1 => UserRole::Client,
            2 => UserRole::Client,
            3 => UserRole::Client,
            4 => UserRole::Client,
            5 => UserRole::SuperUser,
            6 => UserRole::DataAnalyst,
            _ => UserRole::Admin,
        }
    }
}

impl UserRole {
    fn default() -> Self {
        UserRole::Client
    }
}