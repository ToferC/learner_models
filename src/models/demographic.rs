use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use rand::{Rng};

use fake::{Dummy, Fake, Faker};
use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::faker::boolean::en::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy, Clone)]
/// Represents additional demographic and preference details about a 
/// person. This data is needed to identify potential bias within our 
/// institutions. It would be protected B and would be treated 
/// as secure data. It should come from a central trusted source (OCHRO)
pub struct DemographicData {
    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: String,

    #[dummy(faker = "DateTimeBetween(EN, Utc.ymd(1965, 1, 1).and_hms(9, 10, 11), Utc.ymd(1998,6,12).and_hms(9, 10, 11))")]
    pub date_of_birth: String,
    
    pub native_language: Language,
    pub primary_official_language: Language,
    pub communication_language: Language,
    pub sexuality: Sexuality,
    pub pronouns: Pronouns,

    #[dummy(faker = "Boolean(5)")]
    pub transgender: bool,
    pub ethnicity: Ethnicity,

    #[dummy(faker = "Boolean(10)")]
    pub person_with_disability: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Represents the person's statement on their sexuality.
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

impl Dummy<Faker> for Sexuality {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: f64 = (0.01..0.10).fake_with_rng(rng);
        
        match i {
            i if i < 0.05 => Sexuality::NoAnswer,
            i if i < 0.80 => Sexuality::Heterosexual,
            i if i < 0.90 => Sexuality::Bisexual,
            i if i < 1.00 => Sexuality::Homosexual,
            _ => Sexuality::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Represents the person's statement on their gender and
/// pronoun preferences.
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

impl Dummy<Faker> for Pronouns {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: f64 = (0.01..1.00).fake_with_rng(rng);
        
        match i {
            i if i < 0.45 => Pronouns::HeHim,
            i if i < 0.90 => Pronouns::SheHer,
            i if i < 0.95 => Pronouns::TheyThem,
            i if i < 1.00 => Pronouns::NoAnswer,
            _ => Pronouns::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Dummy<Faker> for Ethnicity {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: f64 = (0.01..1.00).fake_with_rng(rng);
        
        match i {
            i if i < 0.05 => Ethnicity::NoAnswer,
            i if i < 0.80 => Ethnicity::Caucasian,
            i if i < 0.85 => Ethnicity::Asian,
            i if i < 0.90 => Ethnicity::Black,
            i if i < 0.95 => Ethnicity::HispanicLatinx,
            i if i < 1.00 => Ethnicity::Indigenous,
            _ => Ethnicity::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
/// Represents a language.
pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
    LotsMore,
}

impl Dummy<Faker> for Language {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..2).fake_with_rng(rng);
        
        match i {
            0 => Language::French,
            1 => Language::English,
            _ => Language::English,
        }
    }
}