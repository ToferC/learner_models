#[derive(Debug)]
/// A user of the system, employee or learner
pub struct User {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email_address: String,
    pub user_role: UserRole,
}

#[derive(Debug)]
pub enum UserRole {
    Admin,
    SuperUser,
    Client,
    DataAnalyst,
}