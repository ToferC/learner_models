use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use super::{Experience, Image, Location, User, DemographicData};

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::chrono::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a user as learner. Much of this data should come from OCHRO.
/// Learner data could be developed as part of a permission-based
/// system that allowed the learner to have full control over their data
/// and allow the learning systems to request acccess with specific
/// use cases.
pub struct Learner {
    pub id: i64,
    pub user: User,

    #[dummy(faker = "(Faker, 3..5)")]
    pub badges: Vec<BadgeAssertion>,

    pub demographics: DemographicData,

    #[dummy(faker = "(Faker, 3..5)")]
    pub employment_status: Vec<EmploymentStatus>,

    #[dummy(faker = "(Faker, 3..5)")]
    pub experiences: Vec<Experience>,

    #[dummy(faker = "(Faker, 4..10)")]
    pub data_access_log: Vec<DataAccessLog>,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Badges based on open-badges
/// https://github.com/mozilla/openbadges-specification/blob/master/Assertion/latest.md
pub struct BadgeAssertion {
    pub id: i64,
    pub uid: String,

    #[dummy(faker = "Buzzword(EN)")]
    pub badge: String,

    pub verify: String, // placeholder
    pub image: Image,

    #[dummy(faker = "Sentence(EN, 1..3)")]
    pub evidence: String,
    pub expires: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Log for access of user data for audit and privacy purposes.
/// Will track each access point and return a JSON String of the
/// data accessed and rationale.
pub struct DataAccessLog {
    accessed_by: User,
    rationale: AccessRationale,
    date_stamp: NaiveDateTime,
    data_accessed: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Reasons for accessing user data
pub enum AccessRationale {
    AggregatedReporting,
    IdentifiableReporting,
    AutomatedQuery,
    CustomQuery,
    Audit,
    UserDataRequest,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a target audience
pub enum Audience {
    Employee,
    Manager,
    Specialist,
    Leader,
    SeniorLeader,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
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

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents a GC department or agency. Could include PRI or other
/// data if appropriately secured. Could also include data on org type
/// (line, policy, granting, etc.)
pub struct Organization {
    #[dummy(faker = "CompanyName(EN)")]
    pub name: String,
    pub url: String,
    #[dummy(faker = "Word(EN)")]
    pub acronym: String,
}