use crate::grammar::get_he_stem;
use crate::grammar::get_minä_stem;
use crate::grammar::participles::*;
use crate::grammar::transforms::*;
use crate::verb::{Verb, VerbType};

fn add_imperfect_marker(verb: &mut Verb) {
    if verb.text.chars().last().unwrap() == 'i' {
        return;
    }

    match verb.verb_type {
        VerbType::ONE => {
            if verb.has_a_to_o_transformation {
                verb.transform(|verb| {
                    return replace_ending(verb, r"a", "ending 'a'", "oi", "imperfect marker");
                });
            } else {
                verb.transform(add_imperfect_marker_for_type_one)
            }
        }
        VerbType::TWO => verb.transform(add_imperfect_marker_for_type_two),
        VerbType::FOUR => verb.transform(|verb| {
            let harmonized_a = verb.text.chars().last().unwrap();
            return replace_ending(
                verb,
                r"[aä]",
                format!("ending '{}'", harmonized_a).as_str(),
                "si",
                "imperfect marker",
            );
        }),
        _ => verb.transform(|verb| {
            return replace_ending(verb, r"e", "ending 'e'", "i", "imperfect marker");
        }),
    }
}

fn get_imperfect_minä_stem(verb: &mut Verb) {
    if let Some(_) = verb.imperfect_stem.as_ref() {
        verb.transform(get_imperfect_stem);
        return;
    }

    get_minä_stem(verb);

    add_imperfect_marker(verb);
}

fn get_imperfect_he_stem(verb: &mut Verb) {
    if let Some(_) = verb.imperfect_stem.as_ref() {
        verb.transform(get_imperfect_stem);
        return;
    }

    get_he_stem(verb);

    add_imperfect_marker(verb);
}

pub fn first_person_singular_positive(verb: &mut Verb) {
    get_imperfect_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    get_imperfect_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    get_imperfect_he_stem(verb);
    // Don't add anything to this
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    get_imperfect_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    get_imperfect_minä_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    get_imperfect_he_stem(verb);

    verb.transform(|verb| {
        return add_personal_ending(verb, Person::ThirdPlural);
    });
}

pub fn passive_positive(verb: &mut Verb) {
    super::present::passive_positive(verb);

    // This can largely be explained as strengthening the grade of the passive ending itself
    // Type 2 has daan -> tiin (d -> t), Type 3 will have ll -> lt for example, (st doesn't change because it is protected by s)
    // Remaining types change from taan to ttiin (t -> tt)
    match verb.verb_type {
        VerbType::TWO => verb.transform(|verb| {
            return replace_ending(
                verb,
                "d[aä]{2}n",
                format!("present passive ending 'd{0}{0}n'", verb.vowels.a).as_str(),
                "tiin",
                "past passive",
            );
        }),
        VerbType::THREE => verb.transform(|verb| {
            let fourth_last_char = verb.text.chars().nth_back(4).unwrap();
            return replace_ending(
                verb,
                ".[aä]{2}n",
                format!(
                    "present passive ending '{0}{1}{1}n'",
                    fourth_last_char, verb.vowels.a
                )
                .as_str(),
                "tiin",
                "past passive",
            );
        }),
        _ => verb.transform(|verb| {
            return replace_ending(
                verb,
                "t[aä]{2}n",
                format!("past passive ending 't{0}{0}n'", verb.vowels.a).as_str(),
                "ttiin",
                "past passive",
            );
        }),
    }
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstSingular);
    });
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondSingular);
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::FirstPlural);
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::SecondPlural);
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdPlural);
    });
}

pub fn passive_negative(verb: &mut Verb) {
    past_passive_participle(verb);

    verb.transform(|verb| {
        return prepend_personal_negative(verb, Person::ThirdSingular);
    });
}
