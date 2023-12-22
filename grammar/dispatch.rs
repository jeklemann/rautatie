use super::participles::*;
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

struct Moods {
    conditional: &'static Mood,
    imperative: &'static Mood,
    indicative: &'static Mood,
}

static MOODS: Moods = Moods {
    conditional: &super::moods::conditional::MOOD_STRUCT,
    imperative: &super::moods::imperative::MOOD_STRUCT,
    indicative: &super::moods::indicative::MOOD_STRUCT,
};

pub fn dispatch_verb_form_func(
    verb: &mut Verb,
    mood_sel: MoodSelection,
    tense_sel: TenseSelection,
    person_sel: PersonSelection,
    is_negative: bool,
) -> bool {
    let mood = match mood_sel {
        MoodSelection::Conditional => MOODS.conditional,
        MoodSelection::Imperative => MOODS.imperative,
        MoodSelection::Indicative => MOODS.indicative,
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
            PersonSelection::FirstSingular => (tense.first_person_singular_negative)(verb),
            PersonSelection::SecondSingular => (tense.second_person_singular_negative)(verb),
            PersonSelection::ThirdSingular => (tense.third_person_singular_negative)(verb),
            PersonSelection::FirstPlural => (tense.first_person_plural_negative)(verb),
            PersonSelection::SecondPlural => (tense.second_person_plural_negative)(verb),
            PersonSelection::ThirdPlural => (tense.third_person_plural_negative)(verb),
            PersonSelection::Passive => (tense.passive_negative)(verb),
        }
    } else {
        match person_sel {
            PersonSelection::FirstSingular => (tense.first_person_singular_positive)(verb),
            PersonSelection::SecondSingular => (tense.second_person_singular_positive)(verb),
            PersonSelection::ThirdSingular => (tense.third_person_singular_positive)(verb),
            PersonSelection::FirstPlural => (tense.first_person_plural_positive)(verb),
            PersonSelection::SecondPlural => (tense.second_person_plural_positive)(verb),
            PersonSelection::ThirdPlural => (tense.third_person_plural_positive)(verb),
            PersonSelection::Passive => (tense.passive_positive)(verb),
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
