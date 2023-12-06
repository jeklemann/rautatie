use lazy_static::lazy_static;
use regex::Regex;

use crate::verb::{TransformLogEntry, Verb};

pub fn get_infinitive(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the infinitive"),
        new_text: verb.infinitive.clone(),
    });
}

pub fn get_stem(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the stem (this type has no gradation)"),
        new_text: verb.strong_stem.clone(),
    });
}

pub fn get_strong_stem(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the strong stem"),
        new_text: verb.strong_stem.clone(),
    });
}

pub fn get_weak_stem(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the weak stem"),
        new_text: verb.weak_stem.clone(),
    });
}

pub fn get_imperfect_stem(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the special imperfect stem"),
        new_text: verb.imperfect_stem.as_ref().unwrap().clone(),
    });
}

pub enum Person {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
}

fn get_third_person_singular_ending(stem: &String) -> Option<String> {
    lazy_static! {
        static ref DIPHTHONG_ENDING: Regex = Regex::new(
            r"((aa)|(ee)|(ii)|(oo)|(uu)|(ää)|(öö)|(yy)|(uo)|(yö)|(ie)|([aeiouäöy][iuy]))$"
        )
        .unwrap();
    }

    if !DIPHTHONG_ENDING.is_match(stem) {
        let previous_vowel = stem.chars().last().unwrap();
        return Some(previous_vowel.to_string());
    } else {
        return None;
    }
}

pub fn append_ending(verb: &Verb, ending: &str, ending_name: &str) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: format!("Add the {} ending '{}'", ending_name, ending),
        new_text: format!("{}{}", verb.text, ending),
    });
}

pub fn add_personal_ending(verb: &Verb, person: Person) -> Option<TransformLogEntry> {
    let (ending, ending_name) = match person {
        Person::FirstSingular => (String::from("n"), "first person singular"),
        Person::SecondSingular => (String::from("t"), "second person singular"),
        Person::ThirdSingular => {
            if let Some(ending) = get_third_person_singular_ending(&verb.text) {
                (ending, "third person singular")
            } else {
                return None;
            }
        }
        Person::FirstPlural => (String::from("mme"), "first person plural"),
        Person::SecondPlural => (String::from("tte"), "second person plural"),
        Person::ThirdPlural => {
            let vat_ending = format!("v{}t", verb.vowels.a);
            (vat_ending, "third person plural")
        }
    };

    return append_ending(verb, ending.as_str(), ending_name);
}

pub fn prepend_personal_negative(verb: &Verb, person: Person) -> Option<TransformLogEntry> {
    let (negative, negative_name) = match person {
        Person::FirstSingular => ("en", "first person singular"),
        Person::SecondSingular => ("et", "second person singular"),
        Person::ThirdSingular => ("ei", "third person singular"),
        Person::FirstPlural => ("emme", "first person plural"),
        Person::SecondPlural => ("ette", "second person plural"),
        Person::ThirdPlural => ("eivät", "third person plural"),
    };

    return Some(TransformLogEntry {
        action: format!("Prepend the {} negative '{}'", negative_name, negative),
        new_text: format!("{} {}", negative, verb.text),
    });
}

pub fn replace_ending(
    verb: &Verb,
    replace: &str,
    replace_description: &str,
    with: &str,
    with_description: &str,
) -> Option<TransformLogEntry> {
    let re = Regex::new(format!(r"{}$", replace).as_str()).unwrap();
    let replaced = re.replace(verb.text.as_str(), with);
    if replaced == verb.text {
        return None;
    } else {
        return Some(TransformLogEntry {
            action: format!(
                "Replace {} with {} '{}'",
                replace_description, with_description, with
            ),
            new_text: replaced.into_owned(),
        });
    }
}

// Attempt to replace an unrounded vowel at the end, if it is not found, just add an i
pub fn add_imperfect_marker_for_type_one(verb: &Verb) -> Option<TransformLogEntry> {
    let res = replace_ending(verb, r"[aäei]", "unrounded vowel", "i", "imperfect marker");
    match res {
        Some(result) => Some(result),
        None => return append_ending(verb, "i", "imperfect marker"),
    }
}

pub fn add_imperfect_marker_for_type_two(verb: &Verb) -> Option<TransformLogEntry> {
    lazy_static! {
        static ref DIPHTHONG_ENDING_NO_I: Regex =
            Regex::new(r"((aa)|(ee)|(oo)|(uu)|(ää)|(öö)|(yy)|(uo)|(yö)|(ie)|([aeiouäöy][uy]))$")
                .unwrap();
    }

    if !DIPHTHONG_ENDING_NO_I.is_match(verb.text.as_str()) {
        return None;
    };

    // If we have two vowels, drop the first one, shift the second one back, and add an i
    let mut chars = verb.text.as_str().chars();
    let last = chars.next_back().unwrap();
    let second_last = chars.next_back().unwrap(); // Drop second to last character
    return Some(TransformLogEntry {
        action: format!(
            "Drop the letter '{}' and append 'i' to form the imperfect stem",
            second_last
        ),
        new_text: format!("{}{}i", chars.as_str(), last),
    });
}
