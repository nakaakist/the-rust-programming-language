pub fn to_uppercase_all(s: &str) -> String {
    s.to_uppercase()
}

pub fn to_uppercase_word_beggining(s: &str) -> String {
    let mut result: String = String::new();
    let mut should_upper = true;
    for c in s.chars() {
        if c.is_ascii_whitespace() {
            should_upper = true;
            result.push(c)
        } else {
            if should_upper == true {
                result.push(char_to_uppercase(c));
                should_upper = false;
            } else {
                result.push(c);
            }
        }
    }

    result
}

fn char_to_uppercase(c: char) -> char {
    match c.is_ascii_alphabetic() {
        true => c.to_ascii_uppercase(),
        false => c,
    }
}
