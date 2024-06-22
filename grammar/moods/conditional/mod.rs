use crate::grammar::{perfect::create_perfect_from_base, Mood};

pub mod present;

pub static MOOD_STRUCT: Mood = Mood {
    present: Some(&present::TENSE_STRUCT),
    perfect: Some(&create_perfect_from_base(&present::TENSE_STRUCT)),
    imperfect: None,
    pluperfect: None,
};
