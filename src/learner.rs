use chrono::prelude::*;

pub struct Learner {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub employment_status: Vec<EmploymentStatus>,
    pub demographics: Vec<DemographicData>,
}

pub struct EmploymentStatus {
    pub date_stamp: NaiveDate,
    pub group: Group,
    pub level: usize,
    pub organization: Organization,
}

// secure data
pub struct DemographicData {
    pub date_stamp: NaiveDate,
    pub date_of_birth: NaiveDate,
    pub native_language: Language,
    pub primary_official_language,
    pub sexuality: Sexuality,
    pub pronous: Pronouns,
    pub transgender: bool,
    pub ethnicicty: Ethnicity,
}

pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

pub enum Ethnicity {
    Asian,
    Black,
    Caucasian,
    HispanicLatinx,
    Indigenous,
    Other(String),
    NoAnswer,
}

pub enum Group {
    EC,
    AS,
    PM,
    FB,
    CR,
    PE,
    IS,
}

pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
}

pub struct Organization {
    pub name: String,
    pub url: String,
    pub acronym: String,
}