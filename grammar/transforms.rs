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
