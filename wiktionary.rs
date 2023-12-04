use crate::grammar::utils::gradate_t_char;
use crate::verb::{create_verb, Verb, VerbType};
use regex::Regex;
use std::error::Error;

fn parse_conj_macro_tokens(tokens: Vec<&str>) -> Option<Verb> {
    if tokens.len() == 0 {
        return None;
    }

    // https://en.wiktionary.org/wiki/Wiktionary:Finnish_verb_inflection-table_templates
    let (infinitive, verb_type, strong_stem, weak_stem, imperfect_stem) = match tokens[0] {
        "sanoa" => {
            let infinitive = format!("{}{}{}{}", tokens[1], tokens[2], tokens[4], tokens[5]);
            let strong = format!("{}{}{}", tokens[1], tokens[2], tokens[4]);
            let weak = format!("{}{}{}", tokens[1], tokens[3], tokens[4]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "muistaa" => {
            let infinitive = format!("{}{}{}{}", tokens[1], tokens[2], tokens[4], tokens[4]);
            let strong = format!("{}{}{}", tokens[1], tokens[2], tokens[4]);
            let weak = format!("{}{}{}", tokens[1], tokens[3], tokens[4]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        // We will only handle the top listed case
        "huutaa" | "saartaa" => {
            let gradated_t = gradate_t_char(tokens[1]);

            let infinitive = format!("{}t{}{}", tokens[1], tokens[2], tokens[2]);
            let strong = format!("{}t{}", tokens[1], tokens[2]);
            let weak = format!("{}{}{}", tokens[1], gradated_t, tokens[2]);
            let imperfect = format!("{}si", tokens[1]);
            (infinitive, VerbType::ONE, strong, weak, Some(imperfect))
        }
        "soutaa" => {
            let gradated_t = gradate_t_char(tokens[1]);

            let infinitive = format!("{}t{}{}", tokens[1], tokens[2], tokens[2]);
            let strong = format!("{}t{}", tokens[1], tokens[2]);
            let weak = format!("{}{}{}", tokens[1], gradated_t, tokens[2]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "kaivaa" => {
            let infinitive = format!("{}{}aa", tokens[1], tokens[2]);
            let strong = format!("{}{}a", tokens[1], tokens[2]);
            let weak = format!("{}{}a", tokens[1], tokens[3]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "laskea" => {
            let infinitive = format!("{}{}e{}", tokens[1], tokens[2], tokens[4]);
            let strong = format!("{}{}e", tokens[1], tokens[2]);
            let weak = format!("{}{}e", tokens[1], tokens[3]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "tuntea" => {
            let gradated_t = gradate_t_char(tokens[1]);

            let infinitive = format!("{}te{}", tokens[1], tokens[2]);
            let strong = format!("{}te", tokens[1]);
            let weak = format!("{}{}e", tokens[1], gradated_t);
            let imperfect = format!("{}si", tokens[1]);
            (infinitive, VerbType::ONE, strong, weak, Some(imperfect))
        }
        "lähteä" => {
            let infinitive = format!("{}hte{}", tokens[1], tokens[2]);
            let strong = format!("{}hte", tokens[1]);
            let weak = format!("{}hde", tokens[1]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "sallia" => {
            let infinitive = format!("{}{}i{}", tokens[1], tokens[2], tokens[4]);
            let strong = format!("{}{}i", tokens[1], tokens[2]);
            let weak = format!("{}{}i", tokens[1], tokens[3]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        "voida" | "saada" | "juoda" | "tupakoida" => {
            let infinitive = format!("{}d{}", tokens[1], tokens[2]);
            let strong = format!("{}", tokens[1]);
            let weak = strong.clone();
            (infinitive, VerbType::TWO, strong, weak, None)
        }
        "käydä" => {
            let infinitive = format!("{}d{}", tokens[1], tokens[2]);
            let strong = format!("{}", tokens[1]);
            let weak = strong.clone();
            let imperfect = format!("{}vi", tokens[1]);
            (infinitive, VerbType::TWO, strong, weak, Some(imperfect))
        }
        "rohkaista" => {
            let infinitive = format!("{}{}{}st{}", tokens[1], tokens[2], tokens[4], tokens[5]);
            let strong = format!("{}{}{}se", tokens[1], tokens[3], tokens[4]);
            let weak = format!("{}{}{}se", tokens[1], tokens[2], tokens[4]);
            (infinitive, VerbType::THREE, strong, weak, None)
        }
        "tulla" => {
            let stem_ending = tokens[4].chars().last().unwrap();
            let infinitive = format!(
                "{}{}{}{}{}",
                tokens[1], tokens[2], tokens[4], stem_ending, tokens[5]
            );
            let strong = format!("{}{}{}e", tokens[1], tokens[3], tokens[4]);
            let weak = format!("{}{}{}e", tokens[1], tokens[2], tokens[4]);
            (infinitive, VerbType::THREE, strong, weak, None)
        }
        "valita" => {
            let infinitive = format!("{}t{}", tokens[1], tokens[2]);
            let strong = format!("{}tse", tokens[1]);
            let weak = strong.clone();
            (infinitive, VerbType::FIVE, strong, weak, None)
        }
        "juosta" => {
            let infinitive = format!("{}st{}", tokens[1], tokens[2]);
            let strong = format!("{}kse", tokens[1]);
            let weak = format!("{}kse", tokens[1]);
            (infinitive, VerbType::THREE, strong, weak, None)
        }
        "nähdä" => {
            let infinitive = format!("{}hd{}", tokens[1], tokens[2]);
            let strong = format!("{}ke", tokens[1]);
            let weak = format!("{}e", tokens[1]);
            (infinitive, VerbType::TWO, strong, weak, None)
        }
        "vanheta" => {
            let infinitive = format!("{}{}{}t{}", tokens[1], tokens[2], tokens[4], tokens[5]);
            let strong = format!("{}{}{}ne", tokens[1], tokens[3], tokens[4]);
            let weak = format!("{}{}{}ne", tokens[1], tokens[2], tokens[4]);
            (infinitive, VerbType::SIX, strong, weak, None)
        }
        "salata" => {
            let infinitive = format!("{}{}{}t{}", tokens[1], tokens[2], tokens[4], tokens[4]);
            let strong = format!("{}{}{}{}", tokens[1], tokens[3], tokens[4], tokens[4]);
            let weak = format!("{}{}{}{}", tokens[1], tokens[2], tokens[4], tokens[4]);
            (infinitive, VerbType::FOUR, strong, weak, None)
        }
        "katketa" | "selvitä" => {
            let infinitive = format!("{}{}{}t{}", tokens[1], tokens[2], tokens[4], tokens[5]);
            let strong = format!("{}{}{}{}", tokens[1], tokens[3], tokens[4], tokens[5]);
            let weak = format!("{}{}{}{}", tokens[1], tokens[2], tokens[4], tokens[5]);
            (infinitive, VerbType::FOUR, strong, weak, None)
        }
        "taitaa" => {
            let infinitive = format!("{}t{}{}", tokens[1], tokens[2], tokens[2]);
            let strong = format!("{}t{}", tokens[1], tokens[2]);
            let weak = format!("{}d{}", tokens[1], tokens[2]);
            (infinitive, VerbType::ONE, strong, weak, None)
        }
        &_ => {
            return None;
        }
    };

    return Some(create_verb(
        infinitive,
        verb_type,
        strong_stem,
        weak_stem,
        imperfect_stem,
        tokens[0] == "taitaa",
        tokens[0] == "saartaa" || tokens[0] == "kaivaa",
    ));
}

fn fetch_wiktionary_page(verb: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://en.wiktionary.org/wiki/{}?action=raw&ctype=text/css",
        verb
    );
    let resp = reqwest::blocking::get(url)?.text()?;
    return Ok(resp);
}

fn parse_wiktionary_page(page_text: String) -> Option<Verb> {
    let re = Regex::new(r"\{\{fi-conj-(.+)\}\}").unwrap();
    let Some(caps) = re.captures(page_text.as_str()) else {
        return None;
    };

    if caps.len() < 2 {
        return None;
    }

    let tokens: Vec<&str> = caps[1].split("|").collect();

    return parse_conj_macro_tokens(tokens);
}

pub fn get_verb_from_wiktionary(verb: &str) -> Option<Verb> {
    let page_text = match fetch_wiktionary_page(verb) {
        Ok(value) => value,
        Err(err) => {
            println!("{:#?}", err);
            return None;
        }
    };
    return parse_wiktionary_page(page_text);
}
