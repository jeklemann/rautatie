mod grammar;
mod verb;
mod wiktionary;

use wiktionary::get_verb_from_wiktionary;

use crate::grammar::moods::*;

fn main() {
    let Some(mut verb) = get_verb_from_wiktionary("viedä") else {
        println!("Verb not found");
        return;
    };
    //println!("{:#?}", verb);

    indicative::imperfect::first_person_singular_positive(&mut verb);
    verb.print_log();
    //println!("{:#?}", verb);
}
