use crate::grammar::get_he_stem;
use crate::grammar::get_minä_stem;
use crate::grammar::get_passive_stem;
use crate::grammar::transforms::*;
use crate::verb::TransformLogEntry;
use crate::verb::Verb;

pub fn first_person_singular_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    get_he_stem(verb);

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
    get_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    get_he_stem(verb);

    verb.transform(|verb| {
        if verb.infinitive == "olla" {
            return Some(TransformLogEntry {
                action: String::from("Use the special third person plural form 'ovat'"),
                new_text: String::from("ovat"),
            });
        } else {
            return add_personal_ending(verb, Person::ThirdPlural);
        }
    });
}

pub fn passive_positive(verb: &mut Verb) {
    get_passive_stem(verb);

    verb.transform(|verb| {
        return append_ending(verb, format!("{}n", verb.vowels.a).as_str(), "passive");
    });
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    get_minä_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdPlural);
    });
}

pub fn passive_negative(verb: &mut Verb) {
    get_passive_stem(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}
