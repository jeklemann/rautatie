mod grammar;
mod verb;
mod wiktionary;

use wiktionary::get_verb_from_wiktionary;

use crate::grammar::*;

fn main() {
    let Some(mut verb) = get_verb_from_wiktionary("tietää") else {
        println!("Verb not found");
        return;
    };
    //println!("{:#?}", verb);

    indicative::present::passive_negative(&mut verb);
    verb.print_log();
    //println!("{:#?}", verb);
}
