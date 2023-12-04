use crate::verb::{Verb, VerbType};

use self::transforms::*;

mod transforms;

pub mod indicative;

pub fn gradate_t_char(previous_syllable: &str) -> String {
    let previous_char = previous_syllable.chars().last().unwrap();

    match previous_char {
        't' => {
            return String::new();
        }
        'n' | 'l' | 'r' => {
            return previous_char.to_string();
        }
        _ => {
            return 'd'.to_string();
        }
    }
}

fn get_minä_stem(verb: &mut Verb) {
    match verb.verb_type {
        VerbType::ONE => {
            verb.transform(get_weak_stem);
        }
        _ => {
            // All other types either use no gradation or use strong
            verb.transform(get_strong_stem);
        }
    }
}
