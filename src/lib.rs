pub mod grid;

pub const WORDS: &str = include_str!("words.txt");

pub use grid::{clear_screen};
use grid::{CellState, ROWS};
use grid::{Dictionary, Game, Draw};
use std::io;
use std::process::exit;
use std::thread::{sleep};
use std::time::Duration;

pub fn run(word: String, dictionary: &Dictionary) {
    let mut game = Game::new(word.clone());
    game.render();

    for row in 0..ROWS {
        println!();

        let input_string = read_line();

        match dictionary.validate(&input_string) {
            Ok(()) => {},
            Err(s) => {
                println!("{s}");
                sleep(Duration::from_secs(1));
                game.render();
                continue;
            },
        }

        let mut word = game.word.as_bytes().to_vec();
        let input = input_string.as_bytes();

        for col in 0..5 {
            let current = input[col];
            let state = match word.iter().position(|&x| x == current) {
                Some(index) if index == col => {
                    // masking the word as there may be duplicate letters...
                    // yes, that needs a better description... AGAIN = _GAIN after the first match.
                    word[col] = '_' as u8;
                    CellState::CorrectPosition
                }
                Some(index) => {
                    word[index] = '_' as u8;
                    CellState::IncorrectPosition
                }
                None => CellState::Invalid,
            };
            game.update(row, col, state, (current as char).to_string());
        }

        if game.has_won(row) {
            let text = match row {
                0 => {"Genius!"}
                1 => {"Magnificent!"}
                2 => {"Impressive!"}
                3 => {"Splendid!"}
                4 => {"Great!"}
                5 => {"Phew!"}
                _ => { "WAT?"}
            };
            clear_screen();
            game.render();
            println!("\n{:^38}\n\n", text);
            game.print_share();
            exit(0);
        }

        game.render();
    }

    game.grid.print_result(word);
}

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_ascii_uppercase()
}

