use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// A user of the system, employee or learner. Should be tied to
/// an employee profile at the GC level.
pub struct User {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub work_email_addresses: Vec<String>,
    pub alternate_email_addresses: Vec<String>,
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