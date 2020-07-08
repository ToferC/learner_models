use chrono::prelude::*;

use super::experience::Experience;
use super::location::Location;

#[derive(Debug)]
// Represents a user as learner
pub struct Learner {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub employment_status: Vec<EmploymentStatus>,
    pub demographics: Vec<DemographicData>,
    pub experiences: Vec<Experience>,
    pub location: Location,
}

#[derive(Debug)]
// Represents the employement and work status of an employee
// at a certain point in time. Part of a vector under the learner.
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
// Represents a target audience
pub enum Audience {
    Employee,
    Manager,
    Specialist,
    Leader,
    SeniorLeader,
}

#[derive(Debug)]
// Represents the occupational role of a person
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

#[derive(Debug)]
// Represents additional demographic and preference details about a person
// This data would be protected B and would be treated as secure data.
pub struct DemographicData {
    pub date_stamp: NaiveDate,
    pub date_of_birth: NaiveDate,
    pub native_language: Language,
    pub primary_official_language: Language,
    pub communication_language: Language,
    pub sexuality: Sexuality,
    pub pronous: Pronouns,
    pub transgender: bool,
    pub ethnicicty: Ethnicity,
}

#[derive(Debug)]
// Represents the person's statement on their sexuality.
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

#[derive(Debug)]
// Represents the person's statement on their gender and pronoun preferences.
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

#[derive(Debug)]
// Represents the person's ethnic identification.
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
// Represents a Government of Canada pay group
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
// Represents a language.
pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
}

#[derive(Debug)]
// Represents a GC department or agency
pub struct Organization {
    pub name: String,
    pub url: String,
    pub acronym: String,
}