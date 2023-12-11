use crate::grammar::participles::*;
use crate::grammar::transforms::*;
use crate::verb::Verb;

pub fn first_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past first person singular positive",
            super::imperfect::first_person_singular_positive,
        );
    });
}

pub fn second_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past second person singular positive",
            super::imperfect::second_person_singular_positive,
        );
    });
}

pub fn third_person_singular_positive(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person singular positive",
            super::imperfect::third_person_singular_positive,
        );
    });
}

pub fn first_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past first person plural positive",
            super::imperfect::first_person_plural_positive,
        );
    });
}

pub fn second_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past second person plural positive",
            super::imperfect::second_person_plural_positive,
        );
    });
}

pub fn third_person_plural_positive(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person plural positive",
            super::imperfect::third_person_plural_positive,
        );
    });
}

pub fn passive_positive(verb: &mut Verb) {
    past_passive_participle(verb);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person singular positive",
            super::imperfect::third_person_singular_positive,
        );
    });
}

pub fn first_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past first person singular negative",
            super::imperfect::first_person_singular_negative,
        );
    });
}

pub fn second_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past second person singular negative",
            super::imperfect::second_person_singular_negative,
        );
    });
}

pub fn third_person_singular_negative(verb: &mut Verb) {
    past_active_participle(verb, false);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person singular negative",
            super::imperfect::third_person_singular_negative,
        );
    });
}

pub fn first_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past first person plural negative",
            super::imperfect::first_person_plural_negative,
        );
    });
}

pub fn second_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past second person plural negative",
            super::imperfect::second_person_plural_negative,
        );
    });
}

pub fn third_person_plural_negative(verb: &mut Verb) {
    past_active_participle(verb, true);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person plural negative",
            super::imperfect::third_person_plural_negative,
        );
    });
}

pub fn passive_negative(verb: &mut Verb) {
    past_passive_participle(verb);

    verb.transform(|verb| {
        return prepend_olla_form(
            verb,
            "past third person singular negative",
            super::imperfect::third_person_singular_negative,
        );
    });
}
