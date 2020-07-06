use chrono::prelude::*;

pub struct Learner {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub employment_status: Vec<EmploymentStatus>,
}

pub struct EmploymentStatus {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub group: Group,
    pub level: usize,
    pub organization: Organization,
}

pub enum Group {
    EC,
    AS,
    PM,
    FB,
    AS,
    CR,
}

pub enum Organization {
    name: String,
    url: String,
    acronym: String,
}