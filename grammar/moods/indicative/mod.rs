use crate::grammar::Mood;

pub mod imperfect;
pub mod perfect;
pub mod pluperfect;
pub mod present;

pub static MOOD_STRUCT: Mood = Mood {
    present: Some(&present::TENSE_STRUCT),
    perfect: Some(&perfect::TENSE_STRUCT),
    imperfect: Some(&imperfect::TENSE_STRUCT),
    pluperfect: Some(&pluperfect::TENSE_STRUCT),
};
