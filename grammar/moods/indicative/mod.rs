use crate::grammar::{perfect::create_perfect_from_base, Mood};

pub mod imperfect;
pub mod present;

pub static MOOD_STRUCT: Mood = Mood {
    present: Some(&present::TENSE_STRUCT),
    perfect: Some(&create_perfect_from_base(&present::TENSE_STRUCT)),
    imperfect: Some(&imperfect::TENSE_STRUCT),
    pluperfect: Some(&create_perfect_from_base(&imperfect::TENSE_STRUCT)),
};
