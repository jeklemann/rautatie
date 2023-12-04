use wiktionary::wiktionary::get_verb_from_wiktionary;

use crate::verb::verb::*;

mod verb;
mod wiktionary;

fn main() {
    let mut visit = create_verb(
        String::from("käydä"),
        VerbType::TWO,
        String::from("käy"),
        String::from("käy"),
        None,
        false,
    );

    visit.transform(|verb| {
        return TransformLogEntry {
            action: String::from("Get strong stem"),
            new_text: verb.strong_stem.clone(),
        };
    });
    visit.print_log();
    println!("{:#?}", visit);
}
