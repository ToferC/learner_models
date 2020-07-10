use chrono::prelude::*;

use super::{Experience, Image, Location, User};

#[derive(Debug)]
/// Represents a user as learner. Much of this data should come from OCHRO.
/// Learner data could be developed as part of a permission-based
/// system that allowed the learner to have full control over their data
/// and allow the learning systems to request acccess with specific
/// use cases.
pub struct Learner {
    pub id: i64,
    pub user: User,
    pub badges: Vec<BadgeAssertion>,
    pub demographics: DemographicData,
    pub employment_status: Vec<EmploymentStatus>,
    pub experiences: Vec<Experience>,
    pub data_access_log: Vec<DataAccessLog>,
}

#[derive(Debug)]
/// Badges based on open-badges
/// https://github.com/mozilla/openbadges-specification/blob/master/Assertion/latest.md
pub struct BadgeAssertion {
    pub id: i64,
    pub uid: String,
    pub recipient: Learner,
    pub badge: String,
    pub verify: String, // placeholder
    pub image: Image,
    pub evidence: String,
    pub expires: NaiveDate,
}

#[derive(Debug)]
/// Log for access of user data for audit and privacy purposes.
/// Will track each access point and return a JSON String of the
/// data accessed and rationale.
pub struct DataAccessLog {
    accessed_by: User,
    rationale: AccessRationale,
    date_stamp: NaiveDateTime,
    data_accessed: String,
}

#[derive(Debug)]
/// Reasons for accessing user data
pub enum AccessRationale {
    AggregatedReporting,
    IdentifiableReporting,
    AutomatedQuery,
    CustomQuery,
    Audit,
    UserDataRequest,
}

#[derive(Debug)]
/// Represents the employement and work status of an employee
/// at a certain point in time. Part of a vector under the learner.
pub struct EmploymentStatus {
    pub date_stamp: NaiveDate,
    pub group: Group,
    pub role: Role,
    pub community: String,
    pub audience: Audience,
    pub level: usize,
    pub organization: Organization,
    pub work_location: Location,
}

#[derive(Debug)]
/// Represents a target audience
pub enum Audience {
    Employee,
    Manager,
    Specialist,
    Leader,
    SeniorLeader,
}

#[derive(Debug)]
/// Represents the occupational role of a person
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
/// Represents additional demographic and preference details about a 
/// person. This data would be protected B and would be treated 
/// as secure data. It should come from a central trusted source (OCHRO)
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
/// Represents the person's statement on their sexuality.
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

#[derive(Debug)]
/// Represents the person's statement on their gender and
/// pronoun preferences.
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

#[derive(Debug)]
/// Represents the person's ethnic identification.
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
/// Represents a Government of Canada pay group
pub enum Group {
    EC,
    AS,
    PM,
    FB,
    CR,
    PE,
    IS,
    LotsMore,
}

#[derive(Debug)]
/// Represents a language.
pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
    LotsMore,
}

#[derive(Debug)]
/// Represents a GC department or agency. Could include PRI or other
/// data if appropriately secured. Could also include data on org type
/// (line, policy, granting, etc.)
pub struct Organization {
    pub name: String,
    pub url: String,
    pub acronym: String,
}