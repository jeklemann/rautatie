use crate::grammar::get_he_stem;
use crate::grammar::get_minä_stem;
use crate::grammar::transforms::*;
use crate::verb::{Verb, VerbType};

fn get_passive_stem(verb: &mut Verb) {
    if let VerbType::ONE = verb.verb_type {
        verb.transform(get_weak_stem);

        let last_char = verb.text.chars().last().unwrap();

        if last_char == 'a' || last_char == 'ä' {
            verb.transform(modify_stem_for_type_one_passive);
        }

        verb.transform(add_type_one_passive_marker);
    } else {
        verb.transform(get_infinitive);
    }
}

pub fn first_person_singular_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(add_first_person_singular_ending);
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(add_second_person_singular_ending);
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    get_he_stem(verb);

    verb.transform(add_third_person_singular_ending);
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(add_first_person_plural_ending);
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(add_second_person_plural_ending);
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    get_he_stem(verb);

    verb.transform(add_third_person_plural_ending);
}

pub fn passive_positive(verb: &mut Verb) {
    get_passive_stem(verb);

    verb.transform(add_present_passive_ending);
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(prepend_first_person_singular_negative);
}

pub fn passive_negative(verb: &mut Verb) {
    get_passive_stem(verb);

    verb.transform(prepend_third_person_singular_negative);
}
