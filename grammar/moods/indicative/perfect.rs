use crate::grammar::participles::*;
use crate::grammar::transforms::*;
use crate::verb::Verb;

pub fn first_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present first person singular positive",
            super::present::first_person_singular_positive,
        );
    });
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present second person singular positive",
            super::present::second_person_singular_positive,
        );
    });
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present third person singular positive",
            super::present::third_person_singular_positive,
        );
    });
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present first person plural positive",
            super::present::first_person_plural_positive,
        );
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present second person plural positive",
            super::present::second_person_plural_positive,
        );
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present third person plural positive",
            super::present::third_person_plural_positive,
        );
    });
}

pub fn passive_positive(verb: &mut Verb) {
    past_passive_participle(verb);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present passive positive",
            super::present::passive_positive,
        );
    });
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present first person singular negative",
            super::present::first_person_singular_negative,
        );
    });
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present second person singular negative",
            super::present::second_person_singular_negative,
        );
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present third person singular negative",
            super::present::third_person_singular_negative,
        );
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present first person plural negative",
            super::present::first_person_plural_negative,
        );
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present second person plural negative",
            super::present::second_person_plural_negative,
        );
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present third person plural negative",
            super::present::third_person_plural_negative,
        );
    });
}

pub fn passive_negative(verb: &mut Verb) {
    past_passive_participle(verb);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "present passive negative",
            super::present::passive_negative,
        );
    });
}
