mod grammar;
mod verb;
mod wiktionary;

use verb::*;
use wiktionary::get_verb_from_wiktionary;

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

    let verb = get_verb_from_wiktionary("tuntea");
    println!("{:#?}", verb);
}
