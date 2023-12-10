use crate::verb::{Verb, VerbType};

use super::{
    get_passive_stem,
    moods::indicative,
    transforms::{
        add_active_past_participle_marker_for_type_three, append_ending, get_infinitive,
        replace_ending,
    },
};

pub fn past_active_participle(verb: &mut Verb, is_plural: bool) {
    verb.transform(get_infinitive);

    match verb.verb_type {
        VerbType::ONE => verb.transform(|verb| {
            return replace_ending(
                verb,
                verb.vowels.a.to_string().as_str(),
                format!("ending '{}'", verb.vowels.a).as_str(),
                format!("n{}t", verb.vowels.u).as_str(),
                "past active participle marker",
            );
        }),
        VerbType::TWO => verb.transform(|verb| {
            return replace_ending(
                verb,
                format!("d{}", verb.vowels.a).as_str(),
                format!("ending 'd{}'", verb.vowels.a).as_str(),
                format!("n{}t", verb.vowels.u).as_str(),
                "past active participle marker",
            );
        }),
        VerbType::THREE => verb.transform(add_active_past_participle_marker_for_type_three),
        VerbType::FOUR | VerbType::FIVE | VerbType::SIX => verb.transform(|verb| {
            return replace_ending(
                verb,
                format!("t{}", verb.vowels.a).as_str(),
                format!("ending 't{}'", verb.vowels.a).as_str(),
                format!("nn{}t", verb.vowels.u).as_str(),
                "past active participle marker",
            );
        }),
    }

    if is_plural {
        verb.transform(|verb| {
            return replace_ending(
                verb,
                "[yu]t",
                format!("ending '{}t'", verb.vowels.u).as_str(),
                "eet",
                "plural ending",
            );
        })
    }
}

pub fn past_passive_participle(verb: &mut Verb) {
    indicative::imperfect::passive_positive(verb);

    verb.transform(|verb| {
        return replace_ending(
            verb,
            "iin",
            "past positive ending 'iin'",
            String::from(verb.vowels.u).as_str(),
            "past passive participle ending",
        );
    });
}
