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
    pub id: u32,

    pub user: User,

    #[dummy(faker = "(Faker, 3..5)")]
    pub badges: Vec<BadgeAssertion>,

    pub demographics: DemographicData,

    #[dummy(faker = "(Faker, 3..5)")]
    pub employment_status: Vec<EmploymentStatus>,

    #[dummy(faker = "(Faker, 3..5)")]
    pub experiences: Vec<Experience>,

    /// simulates the learner's openess and appreciation for the 
    /// learning products used as well as overall politeness and
    /// attitude. A low alignment represents a learner that will 
    /// not like anything, a high alignment learner will like 
    /// almost any experience.
    #[dummy(faker = "1..11")]
    mock_learner_alignment: usize,

    #[dummy(faker = "(Faker, 4..10)")]
    pub data_access_log: Vec<DataAccessLog>,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Badges based on open-badges
/// https://github.com/mozilla/openbadges-specification/blob/master/Assertion/latest.md
pub struct BadgeAssertion {
    pub id: u32,
    pub uid: String,

    #[dummy(faker = "CatchPhase(EN)")]
    pub badge: String,

    pub verify: String, // placeholder
    pub image: Image,

    #[dummy(faker = "Sentence(EN, 1..3)")]
    pub evidence: String,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2024, 1, 1).and_hms(9, 10, 11), Utc.ymd(2025,6,12).and_hms(9, 10, 11))")]
    pub expires: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Log for access of user data for audit and privacy purposes.
/// Will track each access point and return a JSON String of the
/// data accessed and rationale.
pub struct DataAccessLog {

    // user_id
    accessed_by_user_id: u32,
    rationale: AccessRationale,
    
    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    date_stamp: String,
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
    pub role: Role,
    
    #[dummy(faker = "CatchPhase(EN)")]
    pub community: String,
    
    pub audience: Audience,
    
    pub group: Group,

    #[dummy(faker = "1..7")]
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