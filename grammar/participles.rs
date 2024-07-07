use crate::grammar::dispatch::dispatch_verb_form_func;
use crate::grammar::dispatch::MoodSelection;
use crate::grammar::dispatch::PersonSelection;
use crate::grammar::dispatch::TenseSelection;
use crate::grammar::get_he_stem;
use crate::grammar::transforms::*;
use crate::verb::{Verb, VerbType};

pub fn past_active_participle(verb: &mut Verb, is_plural: bool) {
    verb.transform(&get_infinitive);

    match verb.verb_type {
        VerbType::ONE => verb.transform(&|verb| {
            if verb.has_short_past_participle {
                return replace_ending(
                    verb,
                    format!("t{0}{0}", verb.vowels.a).as_str(),
                    format!("ending 't{0}{0}'", verb.vowels.a).as_str(),
                    format!("nn{}t", verb.vowels.u).as_str(),
                    "past active participle marker",
                );
            } else {
                return replace_ending(
                    verb,
                    verb.vowels.a.to_string().as_str(),
                    format!("ending '{}'", verb.vowels.a).as_str(),
                    format!("n{}t", verb.vowels.u).as_str(),
                    "past active participle marker",
                );
            }
        }),
        VerbType::TWO => verb.transform(&|verb| {
            return replace_ending(
                verb,
                format!("d{}", verb.vowels.a).as_str(),
                format!("ending 'd{}'", verb.vowels.a).as_str(),
                format!("n{}t", verb.vowels.u).as_str(),
                "past active participle marker",
            );
        }),
        VerbType::THREE => verb.transform(&add_active_past_participle_marker_for_type_three),
        VerbType::FOUR | VerbType::FIVE | VerbType::SIX => verb.transform(&|verb| {
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
        verb.transform(&|verb| {
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
    // Based mostly off indicative imperfect passive form
    dispatch_verb_form_func(
        verb,
        MoodSelection::Indicative,
        TenseSelection::Imperfect,
        PersonSelection::Passive,
        false,
    );

    verb.transform(&|verb| {
        return replace_ending(
            verb,
            "iin",
            "past positive ending 'iin'",
            String::from(verb.vowels.u).as_str(),
            "past passive participle ending",
        );
    });
}

pub fn present_active_participle(verb: &mut Verb) {
    get_he_stem(verb);

    verb.transform(&|verb| {
        return append_ending(
            verb,
            format!("v{}", verb.vowels.a).as_str(),
            "present active participle",
        );
    });
}

pub fn agent_participle(verb: &mut Verb) {
    get_he_stem(verb);

    verb.transform(&|verb| {
        return append_ending(
            verb,
            format!("m{}", verb.vowels.a).as_str(),
            "agent participle",
        );
    });
}

pub fn present_passive_participle(verb: &mut Verb) {
    // Based mostly off indicative imperfect passive form
    dispatch_verb_form_func(
        verb,
        MoodSelection::Indicative,
        TenseSelection::Imperfect,
        PersonSelection::Passive,
        false,
    );

    verb.transform(&|verb| {
        return replace_ending(
            verb,
            "iin",
            "past positive ending 'iin'",
            format!("{0}v{0}", verb.vowels.a).as_str(),
            "present passive participle ending",
        );
    });
}
