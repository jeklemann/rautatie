mod grammar;
mod verb;
mod wiktionary;

use wiktionary::get_verb_from_wiktionary;

use crate::grammar::moods::*;
use crate::grammar::participles::*;

fn main() {
    let Some(mut verb) = get_verb_from_wiktionary("epäillä") else {
        println!("Verb not found");
        return;
    };
    //println!("{:#?}", verb);

    //indicative::imperfect::first_person_singular_positive(&mut verb);
    past_active_participle(&mut verb, true);
    verb.print_log();
    //println!("{:#?}", verb);
}
