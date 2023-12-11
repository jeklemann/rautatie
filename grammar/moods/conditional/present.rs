use crate::grammar::get_he_stem;
use crate::grammar::get_minä_stem;
use crate::grammar::get_passive_stem;
use crate::grammar::transforms::*;
use crate::verb::TransformLogEntry;
use crate::verb::{Verb, VerbType};

use super::get_conditional_stem;

pub fn first_person_singular_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        if verb.infinitive == "olla" {
            return Some(TransformLogEntry {
                action: String::from("Use the special third person singular form 'on'"),
                new_text: String::from("on"),
            });
        } else {
            return add_personal_ending(verb, Person::ThirdSingular);
        }
    });
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::ThirdPlural);
    });
}

pub fn passive_positive(verb: &mut Verb) {
    // Based mostly off imperfect passive form
    crate::indicative::imperfect::passive_positive(verb);

    verb.transform(|verb| {
        return replace_ending(
            verb,
            "iin",
            "ending 'iin'",
            format!("{}isiin", verb.vowels.a).as_str(),
            "passive conditional ending",
        );
    });
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    get_conditional_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdPlural);
    });
}

pub fn passive_negative(verb: &mut Verb) {
    // Based off the positive form, just remove the ending
    passive_positive(verb);

    verb.transform(|verb| {
        let indices: Vec<usize> = verb
            .text
            .char_indices()
            .rev()
            .take(2)
            .map(|tuple| tuple.0)
            .collect();

        return Some(TransformLogEntry {
            action: String::from("Remove ending 'in'"),
            new_text: String::from(&verb.text[..indices[1]]),
        });
    });

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}
