use crate::grammar::Mood;

pub mod perfect;
pub mod present;

pub static MOOD_STRUCT: Mood = Mood {
    present: Some(&present::TENSE_STRUCT),
    perfect: Some(&perfect::TENSE_STRUCT),
    imperfect: None,
    pluperfect: None,
};
