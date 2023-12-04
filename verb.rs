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
    pub log: Vec<TransformLogEntry>,
}

impl Verb {
    pub fn transform(&mut self, transform_func: fn(&mut Verb) -> TransformLogEntry) {
        let entry = transform_func(self);
        self.text = entry.new_text.clone();
        self.log.push(entry);
        return;
    }

    pub fn print_log(&self) {
        for e in self.log.iter() {
            println!("{} | {}", e.new_text, e.action);
        }
    }
}

pub fn create_verb(
    infinitive: String,
    verb_type: VerbType,
    strong_stem: String,
    weak_stem: String,
    imperfect_stem: Option<String>,
    has_short_past_participle: bool,
) -> Verb {
    return Verb {
        text: infinitive.clone(),
        verb_type: verb_type,
        infinitive: infinitive,
        strong_stem: strong_stem,
        weak_stem: weak_stem,
        imperfect_stem: imperfect_stem,
        has_short_past_participle: has_short_past_participle,
        log: Vec::new(),
    };
}
