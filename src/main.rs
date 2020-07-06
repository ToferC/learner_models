mod learner;

use learner::Learner;

fn main() {
    let l = Learner {
        id: 0001,
        last_name: String::from("Decibel"),
        first_name: String::from("Danielle"),
        demographics: vec!(),
        employment_status: vec!(), 
    };

    println!("{:?}", l);
}
