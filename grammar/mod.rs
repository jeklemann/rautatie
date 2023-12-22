use crate::verb::{Verb, VerbType};

use self::transforms::*;

mod transforms;

pub mod dispatch;
pub mod moods;
pub mod participles;

pub struct Mood {
    pub present: Option<&'static Tense>,
    pub imperfect: Option<&'static Tense>,
    pub perfect: Option<&'static Tense>,
    pub pluperfect: Option<&'static Tense>,
}

pub struct Tense {
    pub first_person_singular_positive: fn(&mut Verb),
    pub second_person_singular_positive: fn(&mut Verb),
    pub third_person_singular_positive: fn(&mut Verb),
    pub first_person_plural_positive: fn(&mut Verb),
    pub second_person_plural_positive: fn(&mut Verb),
    pub third_person_plural_positive: fn(&mut Verb),
    pub passive_positive: fn(&mut Verb),

    pub first_person_singular_negative: fn(&mut Verb),
    pub second_person_singular_negative: fn(&mut Verb),
    pub third_person_singular_negative: fn(&mut Verb),
    pub first_person_plural_negative: fn(&mut Verb),
    pub second_person_plural_negative: fn(&mut Verb),
    pub third_person_plural_negative: fn(&mut Verb),
    pub passive_negative: fn(&mut Verb),
}

#[macro_export]
macro_rules! gen_tense_struct {
    () => {
        pub static TENSE_STRUCT: crate::grammar::Tense = crate::grammar::Tense {
            first_person_singular_positive: first_person_singular_positive,
            second_person_singular_positive: second_person_singular_positive,
            third_person_singular_positive: third_person_singular_positive,
            first_person_plural_positive: first_person_plural_positive,
            second_person_plural_positive: second_person_plural_positive,
            third_person_plural_positive: third_person_plural_positive,
            passive_positive: passive_positive,

            first_person_singular_negative: first_person_singular_negative,
            second_person_singular_negative: second_person_singular_negative,
            third_person_singular_negative: third_person_singular_negative,
            first_person_plural_negative: first_person_plural_negative,
            second_person_plural_negative: second_person_plural_negative,
            third_person_plural_negative: third_person_plural_negative,
            passive_negative: passive_negative,
        };
    };
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
            verb.transform(get_weak_stem);
        }
        VerbType::TWO | VerbType::FIVE => {
            verb.transform(get_stem);
        }
        VerbType::THREE | VerbType::FOUR | VerbType::SIX => {
            verb.transform(get_strong_stem);
        }
    }
}

fn get_he_stem(verb: &mut Verb) {
    match verb.verb_type {
        VerbType::ONE | VerbType::THREE | VerbType::FOUR | VerbType::SIX => {
            verb.transform(get_strong_stem);
        }
        VerbType::TWO | VerbType::FIVE => {
            verb.transform(get_stem);
        }
    }
}

fn get_passive_stem(verb: &mut Verb) {
    if let VerbType::ONE = verb.verb_type {
        verb.transform(get_weak_stem);

        let last_char = verb.text.chars().last().unwrap();

        if last_char == 'a' || last_char == 'ä' {
            verb.transform(|verb| {
                return replace_ending(
                    verb,
                    r"[aä]",
                    verb.vowels.a.to_string().as_str(),
                    "e",
                    "the letter",
                );
            });
        }

        verb.transform(|verb| {
            append_ending(
                verb,
                format!("t{}", verb.vowels.a).as_str(),
                "verb type one passive stem",
            )
        });
    } else {
        verb.transform(get_infinitive);
    }
}
