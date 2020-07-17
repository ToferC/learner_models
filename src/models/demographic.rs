use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use rand::{Rng};

use fake::{Dummy, Fake, Faker};
use fake::faker::chrono::raw::*;
use chrono::Utc;
use fake::faker::boolean::en::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Dummy)]
/// Represents additional demographic and preference details about a 
/// person. This data is needed to identify potential bias within our 
/// institutions. It would be protected B and would be treated 
/// as secure data. It should come from a central trusted source (OCHRO)
pub struct DemographicData {
    //#[dummy(faker = "DateBetween(EN, Utc.ymd(2020, 1, 1).and_hms(9, 10, 11), Utc.ymd(2020,6,12).and_hms(9, 10, 11))")]
    pub date_stamp: NaiveDateTime,
    pub date_of_birth: NaiveDate,
    pub native_language: Language,
    pub primary_official_language: Language,
    pub communication_language: Language,
    pub sexuality: Sexuality,
    pub pronouns: Pronouns,

    #[dummy(faker = "Boolean(5)")]
    pub transgender: bool,
    pub ethnicicty: Ethnicity,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the person's statement on their sexuality.
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Bisexual,
    NoAnswer,
}

impl Dummy<Faker> for Sexuality {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..4).fake_with_rng(rng);
        
        match i {
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

impl Dummy<Faker> for Pronouns {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..4).fake_with_rng(rng);
        
        match i {
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

impl Dummy<Faker> for Ethnicity {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..7).fake_with_rng(rng);
        
        match i {
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

impl Dummy<Faker> for Language {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..7).fake_with_rng(rng);
        
        match i {
            0 => Language::French,
            1 => Language::English,
            _ => Language::English,
        }
    }
}