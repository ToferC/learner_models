use chrono::prelude::*;

#[derive(Debug)]
/// A user of the system, employee or learner
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

#[derive(Debug)]
/// Role for user
pub enum UserRole {
    Admin,
    SuperUser,
    Client,
    DataAnalyst,
}