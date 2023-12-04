use crate::verb::{TransformLogEntry, Verb};

pub fn get_infinitive(verb: &Verb) -> Option<TransformLogEntry> {
    return Some(TransformLogEntry {
        action: String::from("Get the infinitive"),
        new_text: verb.infinitive.clone(),
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

enum Person {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
}

fn add_personal_ending(verb: &Verb, person: Person) -> Option<TransformLogEntry> {
    let (ending, ending_name) = match person {
        Person::FirstSingular => (String::from("n"), "first person singular"),
        Person::SecondSingular => (String::from("t"), "second person singular"),
        Person::ThirdSingular => (String::from("e"), "third person singular"),
        Person::FirstPlural => (String::from("mme"), "first person plural"),
        Person::SecondPlural => (String::from("tte"), "second person plural"),
        Person::ThirdPlural => {
            let harmonized_a = verb.infinitive.chars().last().unwrap();
            let vat_ending = format!("v{}t", harmonized_a);
            (vat_ending, "third person plural")
        }
    };

    return Some(TransformLogEntry {
        action: format!("Add the {} ending '{}'", ending_name, ending),
        new_text: format!("{}{}", verb.text, ending),
    });
}

pub fn add_first_person_singular_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::FirstSingular);
}

pub fn add_second_person_singular_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::SecondSingular);
}

pub fn add_third_person_singular_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::ThirdSingular);
}

pub fn add_first_person_plural_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::FirstPlural);
}

pub fn add_second_person_plural_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::SecondPlural);
}

pub fn add_third_person_plural_ending(verb: &Verb) -> Option<TransformLogEntry> {
    return add_personal_ending(verb, Person::ThirdPlural);
}

pub fn add_type_one_passive_marker(verb: &Verb) -> Option<TransformLogEntry> {
    let harmonized_a = verb.infinitive.chars().last().unwrap();

    return Some(TransformLogEntry {
        action: format!("Add 't{}' to the end for type 1", harmonized_a),
        new_text: format!("{}t{}", verb.text, harmonized_a),
    });
}

pub fn modify_stem_for_type_one_passive(verb: &Verb) -> Option<TransformLogEntry> {
    let mut chars = verb.text.chars();
    let harmonized_a = chars.next_back().unwrap();

    return Some(TransformLogEntry {
        action: format!(
            "Replace final '{}' with 'e' on verb type one if it exists in type 1",
            harmonized_a
        ),
        new_text: format!("{}e", chars.as_str()),
    });
}

pub fn add_present_passive_ending(verb: &Verb) -> Option<TransformLogEntry> {
    let harmonized_a = verb.infinitive.chars().last().unwrap();

    return Some(TransformLogEntry {
        action: format!("Add the passive ending '{}n'", harmonized_a),
        new_text: format!("{}{}n", verb.text, harmonized_a),
    });
}

fn prepend_personal_negative(verb: &Verb, person: Person) -> Option<TransformLogEntry> {
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

pub fn prepend_first_person_singular_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::FirstSingular);
}

pub fn prepend_second_person_singular_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::SecondSingular);
}

pub fn prepend_third_person_singular_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::ThirdSingular);
}

pub fn prepend_first_person_plural_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::FirstPlural);
}

pub fn prepend_second_person_plural_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::SecondPlural);
}

pub fn prepend_third_person_plural_negative(verb: &Verb) -> Option<TransformLogEntry> {
    return prepend_personal_negative(verb, Person::ThirdPlural);
}
