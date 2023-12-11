use crate::grammar::get_he_stem;
use crate::grammar::transforms::*;
use crate::verb::TransformLogEntry;
use crate::verb::{Verb, VerbType};

fn get_conditional_stem(verb: &mut Verb) {
    get_he_stem(verb);

    let last_char = verb.text.chars().last().unwrap();
    if last_char == 'i' {
        verb.transform(|verb| {
            return append_ending(verb, "si", "conditional");
        });
        return;
    }

    match verb.verb_type {
        VerbType::ONE => verb.transform(|verb| {
            if let Some(replacement_log) =
                replace_ending(verb, "e", "ending 'e'", "isi", "conditional ending")
            {
                return Some(replacement_log);
            } else {
                return append_ending(verb, "isi", "conditional ending");
            }
        }),
        VerbType::TWO => {
            verb.transform(add_imperfect_marker_for_type_two);
            verb.transform(|verb| {
                return append_ending(verb, "si", "conditional ending");
            })
        }
        VerbType::FOUR => {
            let end_chars: Vec<char> = verb.text.chars().rev().take(2).collect();
            // Check for a double vowel ending to stem, which is always a or ä
            if end_chars[0] == end_chars[1] {
                verb.transform(|verb| {
                    return replace_ending(
                        verb,
                        r"[aä]",
                        format!("ending '{}'", verb.vowels.a).as_str(),
                        "isi",
                        "conditional ending",
                    );
                });
            } else {
                verb.transform(|verb| return append_ending(verb, "isi", "conditional ending"))
            }
        }
        _ => verb.transform(|verb| {
            return replace_ending(verb, "e", "ending 'e'", "isi", "conditional ending");
        }),
    }
}

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
    // Don't add anything to this
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
