use clap::Parser;
use rurdle::grid::Dictionary;
use rurdle::{WORDS, run};
use std::collections::BTreeSet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: Option<String>,
    #[arg(long)]
    hard: bool,
}

fn main() {
    let args: Args = Args::parse();
    let word_set: BTreeSet<String> = WORDS.lines().map(|w| w.to_ascii_uppercase()).collect();
    let dictionary = Dictionary::new(word_set);
    let random_word = fetch_word(args, &dictionary);
    run(random_word, &dictionary);
}

fn fetch_word(args: Args, dictionary: &Dictionary) -> String {
    if args.debug.is_some() {
        args.debug.unwrap().to_ascii_uppercase().to_string()
    } else {
        dictionary.random_word()
    }
}
