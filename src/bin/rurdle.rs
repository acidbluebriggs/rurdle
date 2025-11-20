use rurdle::{WORDS, run};
use std::collections::BTreeSet;
use std::env;
use rurdle::grid::Dictionary;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let is_hard_mode = args.contains(&"hard".to_string());
    let word_set: BTreeSet<String> = WORDS.lines().map(|w| w.to_ascii_uppercase()).collect();
    let dictionary = Dictionary::new(word_set);

    let random_word = if args.contains(&"debug".to_string()) {
        "RENEW".to_string()
    } else {
        dictionary.random_word()
    };

    run(random_word.to_string(), &dictionary);
}