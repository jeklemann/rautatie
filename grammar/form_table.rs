use super::moods::*;
use super::participles::*;
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

// This sucks ass, I should probably do some stuff with traits for the tenses but I did not have time to figure this out
// Maybe it is more performant though?
pub fn dispatch_verb_form_func(
    verb: &mut Verb,
    mood: MoodSelection,
    tense: TenseSelection,
    person: PersonSelection,
    is_negative: bool,
) -> bool {
    match mood {
        MoodSelection::Indicative => match tense {
            TenseSelection::Present => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        indicative::present::first_person_singular_negative(verb)
                    } else {
                        indicative::present::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        indicative::present::second_person_singular_negative(verb)
                    } else {
                        indicative::present::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        indicative::present::third_person_singular_negative(verb)
                    } else {
                        indicative::present::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        indicative::present::first_person_plural_negative(verb)
                    } else {
                        indicative::present::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        indicative::present::second_person_plural_negative(verb)
                    } else {
                        indicative::present::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        indicative::present::third_person_plural_negative(verb)
                    } else {
                        indicative::present::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        indicative::present::passive_negative(verb)
                    } else {
                        indicative::present::passive_positive(verb)
                    }
                }
            },
            TenseSelection::Imperfect => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        indicative::imperfect::first_person_singular_negative(verb)
                    } else {
                        indicative::imperfect::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        indicative::imperfect::second_person_singular_negative(verb)
                    } else {
                        indicative::imperfect::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        indicative::imperfect::third_person_singular_negative(verb)
                    } else {
                        indicative::imperfect::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        indicative::imperfect::first_person_plural_negative(verb)
                    } else {
                        indicative::imperfect::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        indicative::imperfect::second_person_plural_negative(verb)
                    } else {
                        indicative::imperfect::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        indicative::imperfect::third_person_plural_negative(verb)
                    } else {
                        indicative::imperfect::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        indicative::imperfect::passive_negative(verb)
                    } else {
                        indicative::imperfect::passive_positive(verb)
                    }
                }
            },
            TenseSelection::Perfect => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        indicative::perfect::first_person_singular_negative(verb)
                    } else {
                        indicative::perfect::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        indicative::perfect::second_person_singular_negative(verb)
                    } else {
                        indicative::perfect::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        indicative::perfect::third_person_singular_negative(verb)
                    } else {
                        indicative::perfect::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        indicative::perfect::first_person_plural_negative(verb)
                    } else {
                        indicative::perfect::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        indicative::perfect::second_person_plural_negative(verb)
                    } else {
                        indicative::perfect::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        indicative::perfect::third_person_plural_negative(verb)
                    } else {
                        indicative::perfect::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        indicative::perfect::passive_negative(verb)
                    } else {
                        indicative::perfect::passive_positive(verb)
                    }
                }
            },
            TenseSelection::Pluperfect => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        indicative::pluperfect::first_person_singular_negative(verb)
                    } else {
                        indicative::pluperfect::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        indicative::pluperfect::second_person_singular_negative(verb)
                    } else {
                        indicative::pluperfect::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        indicative::pluperfect::third_person_singular_negative(verb)
                    } else {
                        indicative::pluperfect::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        indicative::pluperfect::first_person_plural_negative(verb)
                    } else {
                        indicative::pluperfect::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        indicative::pluperfect::second_person_plural_negative(verb)
                    } else {
                        indicative::pluperfect::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        indicative::pluperfect::third_person_plural_negative(verb)
                    } else {
                        indicative::pluperfect::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        indicative::pluperfect::passive_negative(verb)
                    } else {
                        indicative::pluperfect::passive_positive(verb)
                    }
                }
            },
        },
        MoodSelection::Imperative => match tense {
            TenseSelection::Present => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        imperative::present::first_person_singular_negative(verb)
                    } else {
                        imperative::present::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        imperative::present::second_person_singular_negative(verb)
                    } else {
                        imperative::present::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        imperative::present::third_person_singular_negative(verb)
                    } else {
                        imperative::present::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        imperative::present::first_person_plural_negative(verb)
                    } else {
                        imperative::present::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        imperative::present::second_person_plural_negative(verb)
                    } else {
                        imperative::present::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        imperative::present::third_person_plural_negative(verb)
                    } else {
                        imperative::present::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        imperative::present::passive_negative(verb)
                    } else {
                        imperative::present::passive_positive(verb)
                    }
                }
            },
            TenseSelection::Perfect => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        imperative::perfect::first_person_singular_negative(verb)
                    } else {
                        imperative::perfect::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        imperative::perfect::second_person_singular_negative(verb)
                    } else {
                        imperative::perfect::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        imperative::perfect::third_person_singular_negative(verb)
                    } else {
                        imperative::perfect::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        imperative::perfect::first_person_plural_negative(verb)
                    } else {
                        imperative::perfect::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        imperative::perfect::second_person_plural_negative(verb)
                    } else {
                        imperative::perfect::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        imperative::perfect::third_person_plural_negative(verb)
                    } else {
                        imperative::perfect::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        imperative::perfect::passive_negative(verb)
                    } else {
                        imperative::perfect::passive_positive(verb)
                    }
                }
            },
            _ => return false,
        },
        MoodSelection::Conditional => match tense {
            TenseSelection::Present => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        conditional::present::first_person_singular_negative(verb)
                    } else {
                        conditional::present::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        conditional::present::second_person_singular_negative(verb)
                    } else {
                        conditional::present::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        conditional::present::third_person_singular_negative(verb)
                    } else {
                        conditional::present::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        conditional::present::first_person_plural_negative(verb)
                    } else {
                        conditional::present::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        conditional::present::second_person_plural_negative(verb)
                    } else {
                        conditional::present::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        conditional::present::third_person_plural_negative(verb)
                    } else {
                        conditional::present::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        conditional::present::passive_negative(verb)
                    } else {
                        conditional::present::passive_positive(verb)
                    }
                }
            },
            TenseSelection::Perfect => match person {
                PersonSelection::FirstSingular => {
                    if is_negative {
                        conditional::perfect::first_person_singular_negative(verb)
                    } else {
                        conditional::perfect::first_person_singular_positive(verb)
                    }
                }
                PersonSelection::SecondSingular => {
                    if is_negative {
                        conditional::perfect::second_person_singular_negative(verb)
                    } else {
                        conditional::perfect::second_person_singular_positive(verb)
                    }
                }
                PersonSelection::ThirdSingular => {
                    if is_negative {
                        conditional::perfect::third_person_singular_negative(verb)
                    } else {
                        conditional::perfect::third_person_singular_positive(verb)
                    }
                }
                PersonSelection::FirstPlural => {
                    if is_negative {
                        conditional::perfect::first_person_plural_negative(verb)
                    } else {
                        conditional::perfect::first_person_plural_positive(verb)
                    }
                }
                PersonSelection::SecondPlural => {
                    if is_negative {
                        conditional::perfect::second_person_plural_negative(verb)
                    } else {
                        conditional::perfect::second_person_plural_positive(verb)
                    }
                }
                PersonSelection::ThirdPlural => {
                    if is_negative {
                        conditional::perfect::third_person_plural_negative(verb)
                    } else {
                        conditional::perfect::third_person_plural_positive(verb)
                    }
                }
                PersonSelection::Passive => {
                    if is_negative {
                        conditional::perfect::passive_negative(verb)
                    } else {
                        conditional::perfect::passive_positive(verb)
                    }
                }
            },
            _ => {
                return false;
            }
        },
    };
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
