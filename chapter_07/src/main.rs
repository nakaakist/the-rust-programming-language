use chapter_07::str_utils;

fn main() {
    let sentence = "this is a sample sentence, including '日本語'.";

    println!("{}", str_utils::case::to_uppercase_all(sentence));
    println!("{}", str_utils::case::to_uppercase_word_beggining(sentence));
}
