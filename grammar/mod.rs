use crate::verb::{Verb, VerbType};

use self::transforms::*;

mod transforms;

pub mod dispatch;
pub mod moods;
pub mod participles;
pub mod perfect;

pub struct Mood {
    pub present: Option<&'static dyn Tense>,
    pub imperfect: Option<&'static dyn Tense>,
    pub perfect: Option<&'static dyn Tense>,
    pub pluperfect: Option<&'static dyn Tense>,
}

pub trait Tense: Sync + Send {
    fn first_person_singular_positive(&self, verb: &mut Verb);
    fn second_person_singular_positive(&self, verb: &mut Verb);
    fn third_person_singular_positive(&self, verb: &mut Verb);
    fn first_person_plural_positive(&self, verb: &mut Verb);
    fn second_person_plural_positive(&self, verb: &mut Verb);
    fn third_person_plural_positive(&self, verb: &mut Verb);
    fn passive_positive(&self, verb: &mut Verb);

    fn first_person_singular_negative(&self, verb: &mut Verb);
    fn second_person_singular_negative(&self, verb: &mut Verb);
    fn third_person_singular_negative(&self, verb: &mut Verb);
    fn first_person_plural_negative(&self, verb: &mut Verb);
    fn second_person_plural_negative(&self, verb: &mut Verb);
    fn third_person_plural_negative(&self, verb: &mut Verb);
    fn passive_negative(&self, verb: &mut Verb);
}

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
            verb.transform(&get_weak_stem);
        }
        VerbType::TWO | VerbType::FIVE => {
            verb.transform(&get_stem);
        }
        VerbType::THREE | VerbType::FOUR | VerbType::SIX => {
            verb.transform(&get_strong_stem);
        }
    }
}

fn get_he_stem(verb: &mut Verb) {
    match verb.verb_type {
        VerbType::ONE | VerbType::THREE | VerbType::FOUR | VerbType::SIX => {
            verb.transform(&get_strong_stem);
        }
        VerbType::TWO | VerbType::FIVE => {
            verb.transform(&get_stem);
        }
    }
}

fn get_passive_stem(verb: &mut Verb) {
    if let VerbType::ONE = verb.verb_type {
        verb.transform(&get_weak_stem);

        let last_char = verb.text.chars().last().unwrap();

        if last_char == 'a' || last_char == 'ä' {
            verb.transform(&|verb| {
                return replace_ending(
                    verb,
                    r"[aä]",
                    verb.vowels.a.to_string().as_str(),
                    "e",
                    "the letter",
                );
            });
        }

        verb.transform(&|verb| {
            append_ending(
                verb,
                format!("t{}", verb.vowels.a).as_str(),
                "verb type one passive stem",
            )
        });
    } else {
        verb.transform(&get_infinitive);
    }
}
