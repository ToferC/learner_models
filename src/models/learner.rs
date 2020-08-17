use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use rand::{Rng};

use super::{Experience, Image, Location, User, DemographicData};

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::chrono::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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
    /// attitude. A low openness represents a learner that will 
    /// not like anything, a high openness learner will like 
    /// almost any experience.
    #[dummy(faker = "0.4..0.8")]
    pub mock_learner_openness: f64,

    #[dummy(faker = "(Faker, 4..10)")]
    pub data_access_log: Vec<DataAccessLog>,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Reasons for accessing user data
pub enum AccessRationale {
    AggregatedReporting,
    IdentifiableReporting,
    AutomatedQuery,
    CustomQuery,
    Audit,
    UserDataRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents the employement and work status of an employee
/// at a certain point in time. Part of a vector under the learner.
pub struct EmploymentStatus {
    pub date_stamp: String,
    pub role: Role,
    pub community: String,
    pub audience: Audience,
    pub group: Group,
    pub level: usize,
    pub organization: Organization,
    pub work_location: Location,
}

impl Dummy<Faker> for EmploymentStatus {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {

        let mut group: Group;
        let mut level: usize;
        let organization: Organization = Faker.fake();
        let work_location: Location = Faker.fake();

        let mut audience: Audience = Faker.fake();

        match audience {
            Audience::Manager => {
                level = 7;
                group = Faker.fake();
            },
            Audience::Leader => {
                group = Group::EX;
                level = rng.gen_range(1, 4);
            },
            Audience::SeniorLeader => {
                group = Group::EX;
                level = rng.gen_range(4, 6);
            },
            _ => {
                group = Faker.fake();
                level = rng.gen_range(1, 6);
            },
        };

        EmploymentStatus {
            date_stamp: String::from("2020-04-01"),
            role: Faker.fake(),
            community: CatchPhase(EN).fake(),
            audience: audience,
            group: group,
            level: level,
            organization: organization,
            work_location: work_location,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents a Government of Canada pay group
pub enum Group {
    EC,
    AS,
    PM,
    FB,
    CR,
    PE,
    IS,
    FI,
    EX,
    LotsMore,
}

impl Dummy<Faker> for Group {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..8).fake_with_rng(rng);
        
        match i {
            0 => Group::EC,
            1 => Group::AS,
            2 => Group::PM,
            3 => Group::FB,
            4 => Group::CR,
            5 => Group::PE,
            6 => Group::IS,
            7 => Group::FI,
            _ => Group::PM,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents a target audience
pub enum Audience {
    Employee,
    Manager,
    Specialist,
    Leader,
    SeniorLeader,
}

impl Dummy<Faker> for Audience {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: f64 = (0.01..0.99).fake_with_rng(rng);
        
        match i {
            x if x < 0.75 => Audience::Employee,
            x if x < 0.85 => Audience::Manager,
            x if x < 0.95 => Audience::Specialist,
            x if x < 0.98 => Audience::Leader,
            x if x < 0.99 => Audience::SeniorLeader,
            _ => Audience::Employee,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Represents a GC department or agency. Could include PRI or other
/// data if appropriately secured. Could also include data on org type
/// (line, policy, granting, etc.)
pub enum Organization {
    CSPS,
    ESDC,
    CBSA,
    TC,
    ISED,
    CRA,
    FIN,
    TBS,
    NRCan,
    StatsCan,
    DFO,
    PS,
    PSC,
    PSPC,
    IRCC,
}