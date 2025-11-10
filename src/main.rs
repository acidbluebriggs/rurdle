use rand::Rng;
use std::cmp::PartialEq;
use std::collections::BTreeSet;
use std::io::{self, Write};
use std::process::exit;
use std::thread::{sleep};
use std::time::Duration;

const WORDS: &str = include_str!("words.txt");
const GREEN: &str = "\x1b[42;90m";
const YELLOW: &str = "\x1b[43;90m";
const GRAY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";
const ROWS: usize  = 6;
const CELLS: usize  = 5;


#[derive(PartialEq)]
enum CellState {
    CorrectPosition,
    IncorrectPosition,
    Invalid,
    Empty,
}

struct Keyboard {

}

struct Cell {
    s: String,
    state: CellState,
}

struct Row {
    cells: [Cell; CELLS],
}

struct Grid {
    word: String,
    rows: [Row; ROWS],
}

impl Row {
    fn new() -> Self {
        Row {
            cells: std::array::from_fn(|_| Cell {
                s: String::new(),
                state: CellState::Empty,
            }),
        }
    }
}

impl Grid {
    fn new(word: String) -> Self {
        Grid {
            word,
            rows: std::array::from_fn(|_| Row::new()),
        }
    }

    fn check_state(&self, index: usize) -> bool {
        let r = &self.rows[index];
        for c in &r.cells {
            if c.state != CellState::CorrectPosition {
                return false;
            }
        }
        true
    }

    fn print_char(letter: &str, state: &CellState) {
        match state {
            CellState::CorrectPosition => {
                print!("{} {} {}", GREEN, letter, RESET)
            }
            CellState::IncorrectPosition => {
                print!("{} {} {}", YELLOW, letter, RESET)
            }
            CellState::Invalid => print!("{} {} {}", GRAY, letter, RESET),
            CellState::Empty => print!(" _ "),
        };
    }

    fn print(&self) {
        clear_screen();
        for r in &self.rows {
            for c in &r.cells {
                let _ = Grid::print_char(c.s.as_str(), &c.state);
            }
            println!()
        }
        io::stdout().flush().unwrap();
    }

    fn print_result(&self) {
        println!("The word was: {}", &self.word);
    }

    fn print_share(&self) {
        for r in &self.rows {
            for c in &r.cells {
                let _ = Grid::print_emoji(&c.state);
            }
            println!()
        }
    }

    fn print_emoji(state: &CellState) {
        let emoji = match state {
            CellState::CorrectPosition => "🟩",
            CellState::IncorrectPosition => "🟨",
            CellState::Invalid => "⬜",
            CellState::Empty => "⬛️"
        };
        print!(" {} ", emoji)
    }
}

fn main() {
    let word_set: BTreeSet<String> = WORDS.lines().map(|w| w.to_ascii_uppercase()).collect();
    let random_word = random_word(&word_set);

    clear_screen();
    // let random_word = "trade";
    run(random_word.to_string(), word_set);
}

fn run(word: String, word_set: BTreeSet<String>) {
    let mut g = Grid::new(word);

    g.print();

    let mut row = 0;

    while row < ROWS {
        let input_string = read_line();
        let input = input_string.as_bytes();

        if !word_set.contains(input_string.as_str()) {
            g.print();
            println!("invalid word");
            sleep(Duration::from_secs(1));
            g.print();
            continue;
        }

        let mut word = g.word.as_bytes().to_vec();

        for col in 0..5 {
            let current = input[col];
            let state = if current == word[col] {
                word[col] = '_' as u8;
                CellState::CorrectPosition
            } else if let Some(index) = word.iter().position(|&x| x == current) {
                word[index] = '_' as u8;
                CellState::IncorrectPosition
            } else {
                CellState::Invalid
            };

            g.rows[row].cells[col].state = state;
            g.rows[row].cells[col].s = (current as char).to_string();
        }

        g.print();

        if g.check_state(row) {
            let text = match row {
                0 => {"Genius!"}
                1 => {"Magnificent!"}
                2 => {"Impressive!"}
                3 => {"Splendid!"}
                4 => {"Great!"}
                5 => {"Phew!"}
                _ => panic!("This can't happen")
            };
            println!();
            println!("{text}");
            println!();
            // g.print_share();
            exit(0);
        }

        row += 1;
    }
    g.print_result();
    // g.print_share();
}

fn random_word(word_set: &BTreeSet<String>) -> String {
    let mut rng = rand::rng();
    let index = rng.random_range(0..word_set.len());
    let random_word = word_set.iter().nth(index).unwrap();
    random_word.to_owned()
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_ascii_uppercase()
}


