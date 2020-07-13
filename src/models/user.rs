use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use rand::rngs::StdRng;
use rand::SeedableRng;

use fake::Dummy;
use fake::faker::name::en::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// A user of the system, employee or learner. Should be tied to
/// an employee profile at the GC level.
pub struct User {
    #[dummy(faker = "1000..")]
    pub id: i64,

    #[dummy(faker = "LastName()")]
    pub last_name: String,

    #[dummy(faker = "LastName()")]
    pub first_name: String,

    pub work_email_addresses: Vec<String>,
    pub alternate_email_addresses: Vec<String>,
    pub twitter_handle: String,
    pub cell_phone: String,
    pub teams_account: i64,
    pub user_role: UserRole,
    pub activated_on: NaiveDateTime,
    pub deactivated_on: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
/// Role for user
pub enum UserRole {
    Admin,
    SuperUser,
    Client,
    DataAnalyst,
}