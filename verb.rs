#[derive(Debug)]
pub enum VerbType {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
}

#[derive(Debug)]
pub struct TransformLogEntry {
    pub action: String,
    pub new_text: String,
}

#[derive(Debug)]
pub struct Verb {
    pub text: String,
    pub verb_type: VerbType,
    pub infinitive: String,
    pub strong_stem: String,
    pub weak_stem: String,
    pub imperfect_stem: Option<String>,
    pub has_short_past_participle: bool,
    pub has_a_to_o_transformation: bool,
    pub log: Vec<TransformLogEntry>,
}

impl Verb {
    pub fn transform(&mut self, transform_func: fn(&Verb) -> Option<TransformLogEntry>) {
        if let Some(entry) = transform_func(self) {
            self.text = entry.new_text.clone();
            self.log.push(entry);
        }
        return;
    }

    pub fn print_log(&self) {
        match self.verb_type {
            VerbType::ONE => {
                println!("Verb type 1:\nMinä stem is weak")
            }
            VerbType::TWO => {
                println!("Verb type 2:\nMinä stem is strong")
            }
            VerbType::THREE => {
                println!("Verb type 3:\nMinä stem is strong")
            }
            VerbType::FOUR => {
                println!("Verb type 4:\nMinä stem is strong")
            }
            VerbType::FIVE => {
                println!("Verb type 5:\nMinä stem is strong")
            }
            VerbType::SIX => {
                println!("Verb type 6:\nMinä stem is strong")
            }
        }

        println!("\nSteps taken:");
        for e in self.log.iter() {
            println!("{} | {}", e.new_text, e.action);
        }

        println!("\nFinal form is '{}'", self.text);
    }
}

pub fn create_verb(
    infinitive: String,
    verb_type: VerbType,
    strong_stem: String,
    weak_stem: String,
    imperfect_stem: Option<String>,
    has_short_past_participle: bool,
    has_a_to_o_transformation: bool,
) -> Verb {
    return Verb {
        text: infinitive.clone(),
        verb_type: verb_type,
        infinitive: infinitive,
        strong_stem: strong_stem,
        weak_stem: weak_stem,
        imperfect_stem: imperfect_stem,
        has_short_past_participle: has_short_past_participle,
        has_a_to_o_transformation: has_a_to_o_transformation,
        log: Vec::new(),
    };
}
