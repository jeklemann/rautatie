pub fn gradate_t_char(previous_syllable: &str) -> String {
    let previous_char = previous_syllable.chars().last().unwrap();

    match previous_char {
        't' => {
            return String::new();
        }
        'n' | 'l' | 'r' => {
            return previous_char.to_string();
        }
        _ => {
            return 'd'.to_string();
        }
    }
}
