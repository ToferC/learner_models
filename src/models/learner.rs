use chrono::prelude::*;

use super::experience::Experience;
use super::location::Location;

#[derive(Debug)]
pub struct Learner {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub employment_status: Vec<EmploymentStatus>,
    pub demographics: Vec<DemographicData>,
    pub experiences: Vec<Experience>,
}

#[derive(Debug)]
pub struct EmploymentStatus {
    pub date_stamp: NaiveDate,
    pub group: Group,
    pub role: Role,
    pub audience: Audience,
    pub level: usize,
    pub organization: Organization,
    pub location: Location,
}

#[derive(Debug)]
pub enum Audience {
    Employee,
    Manager,
    Specialist,
    Leader,
    SeniorLeader,
}

#[derive(Debug)]
pub enum Role {
    All,
    Science,
    Audit,
    Policy,
    Operations,
    Legal,
    Security,
    ComputerScience,
    Regulatory,
    Administrative,
    Research,
    Finance,
    HumanResources,
}

// secure data
#[derive(Debug)]
pub struct DemographicData {
    pub date_stamp: NaiveDate,
    pub date_of_birth: NaiveDate,
    pub native_language: Language,
    pub primary_official_language: Language,
    pub sexuality: Sexuality,
    pub pronous: Pronouns,
    pub transgender: bool,
    pub ethnicicty: Ethnicity,
}

#[derive(Debug)]
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

#[derive(Debug)]
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

#[derive(Debug)]
pub enum Ethnicity {
    Asian,
    Black,
    Caucasian,
    HispanicLatinx,
    Indigenous,
    Other(String),
    NoAnswer,
}

#[derive(Debug)]
pub enum Group {
    EC,
    AS,
    PM,
    FB,
    CR,
    PE,
    IS,
}

#[derive(Debug)]
pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
}

#[derive(Debug)]
pub struct Organization {
    pub name: String,
    pub url: String,
    pub acronym: String,
}