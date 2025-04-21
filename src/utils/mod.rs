use lost_metrics_misc::Class;
use rand::{rng, Rng};


pub fn get_random_support() -> Class {
    let mut rng = rng();
    let supports = Class::get_supports();
    let index = rng.random_range(0..supports.len());

    supports[index]
}

pub fn get_random_dps() -> Class {
    let mut rng = rng();
    let supports = Class::get_dps();
    let index = rng.random_range(0..supports.len());

    supports[index]
}

pub fn get_random_duration(min_seconds: i64, max_seconds: i64) -> i64 {
    let mut rng = rng();
    let duration = rng.random_range(min_seconds..max_seconds);
    
    duration
}

pub fn to_mmss(duration: i64) -> String {
    let minutes = duration / 60;
    let seconds = duration % 60;
    format!("{:02}:{:02}", minutes, seconds)
}