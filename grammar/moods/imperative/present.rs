use crate::gen_tense_struct;
use crate::grammar::get_minä_stem;
use crate::grammar::transforms::*;
use crate::verb::TransformLogEntry;
use crate::verb::{Verb, VerbType};

fn get_imperative_stem(verb: &mut Verb) {
    get_infinitive(verb);

    verb.transform(|verb| {
        let num_chars_to_remove = match verb.verb_type {
            VerbType::TWO | VerbType::THREE => 2,
            VerbType::ONE | VerbType::FOUR | VerbType::FIVE | VerbType::SIX => 1,
        };

        let stem_end = verb
            .text
            .char_indices()
            .nth_back(num_chars_to_remove - 1)
            .unwrap()
            .0;

        return Some(TransformLogEntry {
            action: format!(
                "Replace last {} characters with 'k' to form the imperative stem",
                num_chars_to_remove
            ),
            new_text: format!("{}k", &verb.text[..stem_end]),
        });
    });
}

fn get_imperative_connegative(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(|verb| {
        return append_ending(
            verb,
            String::from(verb.vowels.o).as_str(),
            format!("imperative connegative ending '{}'", verb.vowels.o).as_str(),
        );
    });
}

pub fn first_person_singular_positive(verb: &mut Verb) {
    verb.transform(invalid_form);
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    get_minä_stem(verb);
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(|verb| {
        return add_imperative_personal_ending(verb, Person::ThirdSingular);
    });
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(|verb| {
        return add_imperative_personal_ending(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(|verb| {
        return add_imperative_personal_ending(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(|verb| {
        return add_imperative_personal_ending(verb, Person::ThirdPlural);
    });
}

pub fn passive_positive(verb: &mut Verb) {
    // Based mostly off imperfect passive form
    crate::grammar::moods::indicative::imperfect::passive_positive(verb);

    verb.transform(|verb| {
        return replace_ending(
            verb,
            "iin",
            "ending 'iin'",
            format!("{0}k{1}{1}n", verb.vowels.a, verb.vowels.o).as_str(),
            "passive imperative ending",
        );
    });
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    verb.transform(invalid_form);
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    get_imperative_connegative(verb);

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::ThirdSingular);
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    get_imperative_connegative(verb);

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    get_imperative_connegative(verb);

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    get_imperative_connegative(verb);

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::ThirdPlural);
    });
}

pub fn passive_negative(verb: &mut Verb) {
    // Based off the positive form, just remove the ending
    passive_positive(verb);

    verb.transform(|verb| {
        let stem_end = verb.text.char_indices().nth_back(1).unwrap().0;

        return Some(TransformLogEntry {
            action: String::from("Remove ending 'on'"),
            new_text: String::from(&verb.text[..stem_end]),
        });
    });

    verb.transform(|verb| {
        return prepend_imperative_personal_negative(verb, Person::ThirdSingular);
    });
}

gen_tense_struct!();
