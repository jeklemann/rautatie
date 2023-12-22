use std::io;

use crate::grammar::dispatch::{
    dispatch_participle_form_func, dispatch_verb_form_func, MoodSelection, ParticipleSelection,
    PersonSelection, TenseSelection,
};
use crate::{verb::Verb, wiktionary::get_verb_from_wiktionary};

enum Gamemode {
    MainMenu,
    PrintSteps,
    Guessing,
    Quit,
}

fn prompt_verb() -> Verb {
    println!("Enter a verb:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let verb = get_verb_from_wiktionary(input.trim());
        match verb {
            Some(verb) => return verb,
            None => {
                println!("Verb not found. Ensure you are connected to the internet and you entered the verb's infinitive.")
            }
        }
    }
}

fn prompt_verb_form(verb: &mut Verb) {
    loop {
        println!("Select a mood");
        println!("1 - Indicative");
        println!("2 - Imperative");
        println!("3 - Conditional");

        let mut mood: Option<MoodSelection> = None;
        while let None = mood {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");

            match input.trim() {
                "1" => mood = Some(MoodSelection::Indicative),
                "2" => mood = Some(MoodSelection::Imperative),
                "3" => mood = Some(MoodSelection::Conditional),
                _ => {
                    println!("Please input a valid option")
                }
            }
        }

        println!("Select a tense");
        println!("1 - Present");
        println!("2 - Imperfect");
        println!("3 - Perfect");
        println!("4 - Pluperfect");

        let mut tense: Option<TenseSelection> = None;
        while let None = tense {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");

            match input.trim() {
                "1" => tense = Some(TenseSelection::Present),
                "2" => tense = Some(TenseSelection::Imperfect),
                "3" => tense = Some(TenseSelection::Perfect),
                "4" => tense = Some(TenseSelection::Pluperfect),
                _ => {
                    println!("Please input a valid option")
                }
            }
        }

        println!("Select a person");
        println!("1s - First Singular");
        println!("1p - First Plural");
        println!("2s - Second Singular");
        println!("2p - Second Plural");
        println!("3s - Third Singular");
        println!("3p - Third Plural");
        println!("p - Passive");

        let mut person: Option<PersonSelection> = None;
        while let None = person {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");

            match input.trim() {
                "1s" => person = Some(PersonSelection::FirstSingular),
                "1p" => person = Some(PersonSelection::FirstPlural),
                "2s" => person = Some(PersonSelection::SecondSingular),
                "2p" => person = Some(PersonSelection::SecondPlural),
                "3s" => person = Some(PersonSelection::ThirdSingular),
                "3p" => person = Some(PersonSelection::ThirdPlural),
                "p" => person = Some(PersonSelection::Passive),
                _ => {
                    println!("Please input a valid option")
                }
            }
        }

        println!("Positive or negative?");
        println!("p - Positive");
        println!("n - Negative");

        let mut is_negative: Option<bool> = None;
        while let None = is_negative {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");

            match input.trim() {
                "n" => is_negative = Some(true),
                "p" => is_negative = Some(false),
                _ => {
                    println!("Please input a valid option")
                }
            }
        }

        let success = dispatch_verb_form_func(
            verb,
            mood.unwrap(),
            tense.unwrap(),
            person.unwrap(),
            is_negative.unwrap(),
        );
        if success {
            return;
        } else {
            println!("This form is not valid, please select a new one");
        }
    }
}

fn prompt_participle_form(verb: &mut Verb) {
    println!("Select a participle");
    println!("1 - Present Active");
    println!("2 - Past Active");
    println!("3 - Present Passive");
    println!("4 - Past Passive");
    println!("5 - Agent");

    let mut participle: Option<ParticipleSelection> = None;
    while let None = participle {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim() {
            "1" => participle = Some(ParticipleSelection::PresentActive),
            "2" => participle = Some(ParticipleSelection::PastActive),
            "3" => participle = Some(ParticipleSelection::PresentPassive),
            "4" => participle = Some(ParticipleSelection::PastPassive),
            "5" => participle = Some(ParticipleSelection::Agent),
            _ => {
                println!("Please input a valid option")
            }
        }
    }

    dispatch_participle_form_func(verb, participle.unwrap());
}

fn prompt_form(verb: &mut Verb) {
    println!("Participle or conjugated form?");
    println!("p - Participle");
    println!("c - Conjugated form");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim() {
            "p" => {
                prompt_participle_form(verb);
                return;
            }
            "c" => {
                prompt_verb_form(verb);
                return;
            }
            _ => {
                println!("Please input a valid option")
            }
        }
    }
}

fn play_print_steps_mode() {
    let mut verb = prompt_verb();

    prompt_form(&mut verb);

    verb.print_log();
}

fn play_guess_mode() {
    let mut verb = prompt_verb();

    prompt_form(&mut verb);

    for log_entry in verb.log.iter() {
        println!("Enter how you think this verb is formed:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading guess");

        if guess.trim() == verb.text {
            println!("You got it! Here are the full steps:");
            println!();

            verb.print_log();
            return;
        } else {
            println!("Incorrect! Here is the next step:");
            println!("{} | {}", log_entry.new_text, log_entry.action);
            println!();
        }
    }

    println!("That was the last step. Try studying this a bit more.");
}

fn play_again_prompt(current_gamemode: &mut Gamemode) {
    println!("Type m for main menu");
    println!("Type q for main menu");
    println!("Type anything else to play again");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    match input.trim() {
        "m" => *current_gamemode = Gamemode::MainMenu,
        "q" => *current_gamemode = Gamemode::Quit,
        _ => {}
    }
}

fn main_menu(current_gamemode: &mut Gamemode) {
    println!("Main Menu");
    println!("");
    println!("Select a mode: ");
    println!("1 - Print steps to form verb");
    println!("2 - Guess the form of the verb");
    println!("3 - Quit");

    while let Gamemode::MainMenu = current_gamemode {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim() {
            "1" => *current_gamemode = Gamemode::PrintSteps,
            "2" => *current_gamemode = Gamemode::Guessing,
            "3" => *current_gamemode = Gamemode::Quit,
            _ => {
                println!("Please input a valid option")
            }
        }
    }
}

pub fn init_menu() {
    let mut current_gamemode = Gamemode::MainMenu;

    loop {
        match current_gamemode {
            Gamemode::Quit => {
                println!("Quitting...");
                std::process::exit(0);
            }
            Gamemode::MainMenu => {
                main_menu(&mut current_gamemode);
            }
            Gamemode::Guessing => {
                play_guess_mode();
                play_again_prompt(&mut current_gamemode)
            }
            Gamemode::PrintSteps => {
                play_print_steps_mode();
                play_again_prompt(&mut current_gamemode)
            }
        }
    }
}
