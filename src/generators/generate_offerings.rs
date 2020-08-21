use crate::models::{Offering, LearningProduct};

pub fn generate_offerings(learning_products: &Vec<LearningProduct>) -> Vec<Offering> {
    let mut offerings: Vec<Offering> = Vec::new();

    // Create offerings
    for (index, lp) in learning_products.iter().enumerate() {

        let mut date = chrono::NaiveDate::parse_from_str("2020-06-01", "%Y-%m-%d").unwrap();

        for i in 0..lp.number_of_offerings {

            date = date.checked_add_signed(chrono::Duration::days(i as i64)).unwrap();

            let date_string = date.format("%Y-%m-%d");

            let o = Offering::new(
                777 + lp.id + i as u32,
                index as u32, // would normally be lp.id, but we are looking for an index here
                date_string.to_string(),
                false, 
                true,
            );
            offerings.push(o);
        }
    };

    offerings
}