use crate::grammar::get_passive_stem;
use crate::grammar::participles::past_active_participle;
use crate::grammar::transforms::*;
use crate::grammar::Tense;
use crate::verb::TransformLogEntry;
use crate::verb::Verb;

fn get_potential_stem(verb: &mut Verb) {
    // Mostly based off the past active participle stem
    past_active_participle(verb, true);

    verb.transform(&|verb| {
        let stem_end = verb.text.char_indices().nth_back(1).unwrap().0;

        return Some(TransformLogEntry {
            action: String::from("Remove ending 'et'"),
            new_text: String::from(&verb.text[..stem_end]),
        });
    })
}

pub struct PotentialPresentTense;

impl Tense for PotentialPresentTense {
    fn first_person_singular_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::FirstSingular);
        });
    }

    fn second_person_singular_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::SecondSingular);
        });
    }

    fn third_person_singular_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::ThirdSingular);
        });
    }

    fn first_person_plural_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_positive(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return add_personal_ending(verb, Person::ThirdPlural);
        });
    }

    fn passive_positive(&self, verb: &mut Verb) {
        get_passive_stem(verb);

        verb.transform(&|verb| return append_ending(verb, "neen", "potential passive"));
    }

    fn first_person_singular_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::FirstSingular);
        });
    }

    fn second_person_singular_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::SecondSingular);
        });
    }

    fn third_person_singular_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdSingular);
        });
    }

    fn first_person_plural_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::FirstPlural);
        });
    }

    fn second_person_plural_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::SecondPlural);
        });
    }

    fn third_person_plural_negative(&self, verb: &mut Verb) {
        get_potential_stem(verb);

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdPlural);
        });
    }

    fn passive_negative(&self, verb: &mut Verb) {
        // Based off the positive form, just remove the ending
        self.passive_positive(verb);

        verb.transform(&|verb| {
            let stem_end = verb.text.char_indices().nth_back(1).unwrap().0;

            return Some(TransformLogEntry {
                action: String::from("Remove ending 'en'"),
                new_text: String::from(&verb.text[..stem_end]),
            });
        });

        verb.transform(&|verb| {
            return prepend_personal_negative(verb, Person::ThirdSingular);
        });
    }
}
