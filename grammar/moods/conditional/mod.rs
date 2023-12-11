use crate::{
    grammar::{
        get_he_stem,
        transforms::{add_imperfect_marker_for_type_two, append_ending, replace_ending},
    },
    verb::{Verb, VerbType},
};

pub mod perfect;
pub mod present;

pub fn get_conditional_stem(verb: &mut Verb) {
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
