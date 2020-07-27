use rand::distributions::{Normal, Distribution};

pub fn random_gen_quality(quality: f64) -> f64 {

    let normal = Normal::new(quality as f64, 2.0);
    let v = normal.sample(&mut rand::thread_rng());

    v

}