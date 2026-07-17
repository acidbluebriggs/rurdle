pub mod grid;

pub const WORDS: &str = include_str!("words.txt");

pub use grid::clear_screen;
use grid::{CellState, ROWS};
use grid::{Draw, Game};
use std::collections::{HashMap};
use std::io;
use std::process::exit;

pub fn run(mut game: Game) {
    clear_screen();
    let mut row = 0;

    game.render();

    while row < ROWS {
        let input_string = read_line();

        match game.validate(&input_string) {
            Ok(()) => {}
            Err(s) => {
                game.print_message(s);
                continue;
            }
        }

        let guess_word = input_string.as_bytes();
        let mut solution = game.word.as_bytes().to_vec();
        let mut counts: HashMap<u8, u8> = HashMap::new();

        game.word.as_bytes().iter().for_each(|c| {
            counts.entry(*c).or_insert(0);
            counts.entry(*c).and_modify(|e| *e += 1);
        });

        for col in 0..5 {
            let guess_letter = guess_word[col];
            let state = if guess_letter == solution[col] {
                solution[col] = '_' as u8;
                counts.entry(guess_letter).and_modify(|e| *e -= 1);
                CellState::CorrectPosition
            } else {
                match counts.get_mut(&guess_letter) {
                    None | Some(0) => CellState::Invalid,
                    Some(count) => {
                        *count -= 1;
                        CellState::IncorrectPosition
                    }
                }
            };

            let r = (guess_letter as char).to_string();
            game.update(row, col, state, r);
        }

        if game.has_won(row) {
            let text = match row {
                0 => ("1/6", "Genius!"),
                1 => ("2/6", "Magnificent!"),
                2 => ("3/6", "Impressive!"),
                3 => ("4/6", "Splendid!"),
                4 => ("5/6", "Great!"),
                5 => ("6/6", "Phew!"),
                _ => ("?/?", "WAT?"),
            };

            clear_screen();
            game.render();
            print!("\n{:^38}\n", text.1);
            print!("{:^38}\n\n", text.0);
            game.print_share();
            exit(0);
        }

        game.render();
        row += 1
    }

    game.grid.print_result(game.word);
}



pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_ascii_uppercase()
}
