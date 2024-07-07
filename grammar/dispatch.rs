use super::moods::conditional::present::ConditionalPresentTense;
use super::moods::imperative::present::ImperativePresentTense;
use super::moods::indicative::imperfect::IndicativeImperfectTense;
use super::moods::indicative::present::IndicativePresentTense;
use super::participles::*;
use super::perfect::create_perfect_from_base;
use super::Mood;
use crate::verb::Verb;

pub enum ParticipleSelection {
    PresentPassive,
    PresentActive,
    PastPassive,
    PastActive,
    Agent,
}

pub enum MoodSelection {
    Indicative,
    Imperative,
    Conditional,
}

pub enum TenseSelection {
    Present,
    Imperfect,
    Perfect,
    Pluperfect,
}

pub enum PersonSelection {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
    Passive,
}

struct TenseTable {
    conditional: Mood,
    imperative: Mood,
    indicative: Mood,
}

static TENSE_TABLE: TenseTable = TenseTable {
    conditional: Mood {
        present: Some(&ConditionalPresentTense {}),
        perfect: Some(&create_perfect_from_base(&ConditionalPresentTense {})),
        imperfect: None,
        pluperfect: None,
    },
    imperative: Mood {
        present: Some(&ImperativePresentTense {}),
        perfect: Some(&create_perfect_from_base(&ImperativePresentTense {})),
        imperfect: None,
        pluperfect: None,
    },
    indicative: Mood {
        present: Some(&IndicativePresentTense {}),
        perfect: Some(&create_perfect_from_base(&IndicativePresentTense {})),
        imperfect: Some(&IndicativeImperfectTense {}),
        pluperfect: Some(&create_perfect_from_base(&IndicativeImperfectTense {})),
    },
};

pub fn dispatch_verb_form_func(
    verb: &mut Verb,
    mood_sel: MoodSelection,
    tense_sel: TenseSelection,
    person_sel: PersonSelection,
    is_negative: bool,
) -> bool {
    let mood = match mood_sel {
        MoodSelection::Conditional => &TENSE_TABLE.conditional,
        MoodSelection::Imperative => &TENSE_TABLE.imperative,
        MoodSelection::Indicative => &TENSE_TABLE.indicative,
    };

    let Some(tense) = (match tense_sel {
        TenseSelection::Present => mood.present,
        TenseSelection::Imperfect => mood.imperfect,
        TenseSelection::Perfect => mood.perfect,
        TenseSelection::Pluperfect => mood.pluperfect,
    }) else {
        return false;
    };

    if is_negative {
        match person_sel {
            PersonSelection::FirstSingular => tense.first_person_singular_negative(verb),
            PersonSelection::SecondSingular => tense.second_person_singular_negative(verb),
            PersonSelection::ThirdSingular => tense.third_person_singular_negative(verb),
            PersonSelection::FirstPlural => tense.first_person_plural_negative(verb),
            PersonSelection::SecondPlural => tense.second_person_plural_negative(verb),
            PersonSelection::ThirdPlural => tense.third_person_plural_negative(verb),
            PersonSelection::Passive => tense.passive_negative(verb),
        }
    } else {
        match person_sel {
            PersonSelection::FirstSingular => tense.first_person_singular_positive(verb),
            PersonSelection::SecondSingular => tense.second_person_singular_positive(verb),
            PersonSelection::ThirdSingular => tense.third_person_singular_positive(verb),
            PersonSelection::FirstPlural => tense.first_person_plural_positive(verb),
            PersonSelection::SecondPlural => tense.second_person_plural_positive(verb),
            PersonSelection::ThirdPlural => tense.third_person_plural_positive(verb),
            PersonSelection::Passive => tense.passive_positive(verb),
        }
    }

    return true;
}

pub fn dispatch_participle_form_func(verb: &mut Verb, participle: ParticipleSelection) {
    match participle {
        ParticipleSelection::PresentActive => present_active_participle(verb),
        ParticipleSelection::PresentPassive => present_passive_participle(verb),
        ParticipleSelection::PastPassive => past_passive_participle(verb),
        ParticipleSelection::PastActive => past_active_participle(verb, false),
        ParticipleSelection::Agent => agent_participle(verb),
    };
}
