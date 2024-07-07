use crate::grammar::get_he_stem;
use crate::grammar::get_minä_stem;
use crate::grammar::get_passive_stem;
use crate::grammar::transforms::*;
use crate::grammar::Tense;
use crate::verb::TransformLogEntry;
use crate::verb::Verb;

pub struct IndicativePresentTense;

impl Tense for IndicativePresentTense {
    fn first_person_singular_positive(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::FirstSingular);
        });
    }

    fn second_person_singular_positive(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::SecondSingular);
        });
    }

    fn third_person_singular_positive(&self, verb: &mut Verb) {
        get_he_stem(verb);

        verb.transform(&|verb| {
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

    fn first_person_plural_positive(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_positive(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_positive(&self, verb: &mut Verb) {
        get_he_stem(verb);

        verb.transform(&|verb| {
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

    fn passive_positive(&self, verb: &mut Verb) {
        get_passive_stem(verb);

        verb.transform(&|verb| {
            return append_ending(verb, format!("{}n", verb.vowels.a).as_str(), "passive");
        });
    }

    fn first_person_singular_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::FirstSingular);
        });
    }

    fn second_person_singular_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::SecondSingular);
        });
    }

    fn third_person_singular_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdSingular);
        });
    }

    fn first_person_plural_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdPlural);
        });
    }

    fn passive_negative(&self, verb: &mut Verb) {
        get_passive_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdSingular);
        });
    }
}
