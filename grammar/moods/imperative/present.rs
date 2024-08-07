use crate::grammar::dispatch::dispatch_verb_form_func;
use crate::grammar::dispatch::MoodSelection;
use crate::grammar::dispatch::PersonSelection;
use crate::grammar::dispatch::TenseSelection;
use crate::grammar::get_minä_stem;
use crate::grammar::transforms::*;
use crate::grammar::Tense;
use crate::verb::TransformLogEntry;
use crate::verb::{Verb, VerbType};

fn get_imperative_stem(verb: &mut Verb) {
    get_infinitive(verb);

    verb.transform(&|verb| {
        let num_chars_to_remove = match verb.verb_type {
            VerbType::TWO | VerbType::THREE => 2,
            VerbType::ONE | VerbType::FOUR | VerbType::FIVE | VerbType::SIX => 1,
        };

        let stem_end = verb
            .text
            .char_indices()
            .nth_back(num_chars_to_remove - 1)
            .unwrap()
            .0;

        return Some(TransformLogEntry {
            action: format!(
                "Replace last {} characters with 'k' to form the imperative stem",
                num_chars_to_remove
            ),
            new_text: format!("{}k", &verb.text[..stem_end]),
        });
    });
}

fn get_imperative_connegative(verb: &mut Verb) {
    get_imperative_stem(verb);

    verb.transform(&|verb| {
        return append_ending(
            verb,
            String::from(verb.vowels.o).as_str(),
            format!("imperative connegative ending '{}'", verb.vowels.o).as_str(),
        );
    });
}

pub struct ImperativePresentTense;

impl Tense for ImperativePresentTense {
    fn first_person_singular_positive(&self, verb: &mut Verb) {
        verb.transform(&invalid_form);
    }

    fn second_person_singular_positive(&self, verb: &mut Verb) {
        get_minä_stem(verb);
    }

    fn third_person_singular_positive(&self, verb: &mut Verb) {
        get_imperative_stem(verb);

        verb.transform(&|verb| {
            return add_imperative_personal_ending(verb, Person::ThirdSingular);
        });
    }

    fn first_person_plural_positive(&self, verb: &mut Verb) {
        get_imperative_stem(verb);

        verb.transform(&|verb| {
            return add_imperative_personal_ending(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_positive(&self, verb: &mut Verb) {
        get_imperative_stem(verb);

        verb.transform(&|verb| {
            return add_imperative_personal_ending(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_positive(&self, verb: &mut Verb) {
        get_imperative_stem(verb);

        verb.transform(&|verb| {
            return add_imperative_personal_ending(verb, Person::ThirdPlural);
        });
    }

    fn passive_positive(&self, verb: &mut Verb) {
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
                "ending 'iin'",
                format!("{0}k{1}{1}n", verb.vowels.a, verb.vowels.o).as_str(),
                "passive imperative ending",
            );
        });
    }

    fn first_person_singular_negative(&self, verb: &mut Verb) {
        verb.transform(&invalid_form);
    }

    fn second_person_singular_negative(&self, verb: &mut Verb) {
        get_minä_stem(verb);

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::SecondSingular);
        });
    }

    fn third_person_singular_negative(&self, verb: &mut Verb) {
        get_imperative_connegative(verb);

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::ThirdSingular);
        });
    }

    fn first_person_plural_negative(&self, verb: &mut Verb) {
        get_imperative_connegative(verb);

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_negative(&self, verb: &mut Verb) {
        get_imperative_connegative(verb);

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_negative(&self, verb: &mut Verb) {
        get_imperative_connegative(verb);

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::ThirdPlural);
        });
    }

    fn passive_negative(&self, verb: &mut Verb) {
        // Based off the positive form, just remove the ending
        self.passive_positive(verb);

        verb.transform(&|verb| {
            let stem_end = verb.text.char_indices().nth_back(1).unwrap().0;

            return Some(TransformLogEntry {
                action: String::from("Remove ending 'on'"),
                new_text: String::from(&verb.text[..stem_end]),
            });
        });

        verb.transform(&|verb| {
            return prepend_imperative_personal_negative(verb, Person::ThirdSingular);
        });
    }
}
