use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use rand::rngs::{StdRng};
use rand::{SeedableRng, Rng};
use rand::distributions::{Distribution, Standard};

use fake::{Dummy, Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::faker::chrono::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug)]
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
    pub pronouns: Pronouns,
    pub transgender: bool,
    pub ethnicicty: Ethnicity,
}

impl DemographicData {
    pub fn random() -> Self {
        let l1: Language = rand::random();
        let l2: Language = rand::random();
        let l3 = l2.clone();

        let s: Sexuality = rand::random();
        let p: Pronouns = rand::random();
        let e: Ethnicity = rand::random();

        DemographicData {
            date_stamp: Date(EN).fake(),
            date_of_birth: Date(EN).fake(),
            native_language: l1,
            primary_official_language: l2,
            communication_language: l3,
            sexuality: s,
            pronouns: p,
            transgender: Faker.fake::<bool>(),
            ethnicicty: e,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the person's statement on their sexuality.
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

impl Distribution<Sexuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexuality {
        match rng.gen_range(0, 3) {
            0 => Sexuality::Heterosexual,
            1 => Sexuality::Homosexual,
            2 => Sexuality::Bisexual,
            3 => Sexuality::NoAnswer,
            _ => Sexuality::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the person's statement on their gender and
/// pronoun preferences.
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    Other (String),
    NoAnswer,
}

impl Distribution<Pronouns> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pronouns {
        match rng.gen_range(0, 3) {
            0 => Pronouns::HeHim,
            1 => Pronouns::SheHer,
            2 => Pronouns::TheyThem,
            3 => Pronouns::NoAnswer,
            _ => Pronouns::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

impl Distribution<Ethnicity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ethnicity {
        match rng.gen_range(0, 6) {
            0 => Ethnicity::Asian,
            1 => Ethnicity::Black,
            2 => Ethnicity::Caucasian,
            3 => Ethnicity::Caucasian,
            4 => Ethnicity::Indigenous,
            5 => Ethnicity::Other( String::from("Maui") ),
            6 => Ethnicity::NoAnswer,
            _ => Ethnicity::NoAnswer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// Represents a language.
pub enum Language {
    English,
    French,
    Spanish,
    Mandarin,
    Japanese,
    LotsMore,
}

impl Distribution<Language> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Language {
        match rng.gen_range(0, 1) {
            0 => Language::French,
            1 => Language::English,
            _ => Language::English,
        }
    }
}