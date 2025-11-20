use std::collections::BTreeSet;
use std::io::{self, Write};
use rand::Rng;


pub const ROWS: usize = 6;
const CELLS: usize = 5;

const GREEN: &str = "\x1b[42;90m";
const YELLOW: &str = "\x1b[43;90m";
const GRAY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";

pub trait Draw {
    fn render(&self);
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum CellState {
    CorrectPosition,
    IncorrectPosition,
    Invalid,
    #[default]
    Empty,
}

pub struct Dictionary {
    words: BTreeSet<String>,
}

pub struct Keyboard {
    rows: ([char; 10], [char; 9], [char; 7]),
    letter_state: LetterState,
}

pub struct Cell {
    pub letter: String,
    pub state: CellState,
}

pub struct Row {
    pub cells: [Cell; CELLS],
}

#[derive(Default)]
pub struct Grid {   
}

pub struct Game {
    pub word: String,
    pub grid: Grid,
    pub keyboard: Keyboard,
    game_state: GameState,
}

pub struct LetterState {
    letter_arr: [CellState; 26],
}

pub struct GameState {
    pub rows: [Row; ROWS],
}

impl GameState {
    fn new() -> Self {
        GameState {
            rows: std::array::from_fn(|_| Row::new()),
        }
    }
}

impl Dictionary {
    pub fn new(words: BTreeSet<String>) -> Self {
        Dictionary {
            words,
        }
    }

    pub fn validate(&self, input_string: &String) -> Result<(), String> {
        if !self.words.contains(input_string) {
            return Err(format!("{} not in word list", input_string))
        }
        Ok(())
    }

    pub fn random_word(&self) -> String {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.words.len());
        let random_word = self.words.iter().nth(index).unwrap();
        random_word.to_owned()
    }
}

impl Draw for Keyboard {
    fn render(&self) {

        for c in self.rows.0 {
            let s = self.letter_state.state_for(c);
            let s = keyboard_char(c, s);
            print!(" {s}")
        }

        print!("\n ");

        for c in self.rows.1 {
            let s = self.letter_state.state_for(c);
            let s = keyboard_char(c, s);
            print!(" {s}")
        }

        print!(" \n  ");

        for c in self.rows.2 {
            let s = self.letter_state.state_for(c);
            let s = keyboard_char(c, s);
            print!(" {s}")
        }
    }
}

impl Keyboard {
    fn new() -> Self {
        Keyboard {
            rows: (
                ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
                ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
                ['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
            ),
            letter_state: LetterState::new()
        }
    }
}

impl LetterState {
    fn new() -> Self {
        Self {
            letter_arr: std::array::from_fn(|_| CellState::default()),
        }
    }

    fn set_state(&mut self, c: char, s: CellState) {
        let i = Self::index(c) as usize;
        let current = self.letter_arr[i];

        // only upgrade states: Empty -> anything, IncorrectPosition -> CorrectPosition
        match (current, s) {
            (CellState::Empty, _) => self.letter_arr[i] = s,
            (CellState::IncorrectPosition, CellState::CorrectPosition) => self.letter_arr[i] = s,
            _ => {} // Don't downgrade
        }
    }

    fn state_for(&self, c: char) -> &CellState {
        &self.letter_arr[LetterState::index(c) as usize]
    }

    fn index(c: char) -> u8 {
        c as u8 - 65
    }
}

impl Game {
    pub fn new(word: String) -> Game {
        Game {
            word,
            grid: Grid::default(),
            keyboard: Keyboard::new(),
            game_state: GameState::new(),
        }
    }

    pub fn update(&mut self, row: usize, col: usize, state: CellState, current: String) {
        let cell = &mut self.game_state.rows[row].cells[col];
        let c = current.as_bytes()[0] as char;
        cell.state = state;
        cell.letter = current.clone();
        self.keyboard.letter_state.set_state(c, state);
    }

    pub fn print_share(&self) {
        print!("{}", self.grid.share(&self.game_state));
    }

    pub fn has_won(&self, index: usize) -> bool {
        let r = &self.game_state.rows[index];
        for c in &r.cells {
            if c.state != CellState::CorrectPosition {
                return false;
            }
        }
        true
    }
}

impl Draw for Game {
    fn render(&self) {
        self.grid.print_word_grid(&self.game_state);
        println!();
        self.keyboard.render();
        println!();
    }
}

impl Row {
    fn new() -> Self {
        Row {
            cells: std::array::from_fn(|_| Cell {
                letter: String::new(),
                state: CellState::default(),
            }),
        }
    }
}

impl Grid {
    fn print_word_grid(&self, game_state: &GameState) {
        clear_screen();
        for r in &game_state.rows {
            print!("            ");
            for c in &r.cells {
                let s = grid_char(c.letter.as_str(), &c.state);
                print!("{s}")
            }
            println!()
        }
        io::stdout().flush().unwrap();
    }

    pub fn print_result(&self, word: String) {
        println!("The word was: {}", word);
    }

    fn share(&self, game_state: &GameState) -> String {
        let mut result = String::new();
        for r in &game_state.rows {
            for c in &r.cells {
                result.push_str(&Grid::emoji_for(&c.state));
            }
            result.push('\n');
        }
        result
    }

    pub fn emoji_for(state: &CellState) -> String {
        let emoji = match state {
            CellState::CorrectPosition => "🟩",
            CellState::IncorrectPosition => "🟨",
            CellState::Invalid => "⬜",
            CellState::Empty => "⬛️",
        };
        format!("{}", emoji)
    }
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

fn format_char_with_state(letter: &str, state: &CellState, empty_display: &str) -> String {
    match state {
        CellState::CorrectPosition => format!("{} {} {}", GREEN, letter, RESET),
        CellState::IncorrectPosition => format!("{} {} {}", YELLOW, letter, RESET),
        CellState::Invalid => format!("{} {} {}", GRAY, letter, RESET),
        CellState::Empty => empty_display.to_string(),
    }
}

pub fn keyboard_char(letter: char, state: &CellState) -> String {
    format_char_with_state(&letter.to_string(), state, &format!(" {letter} "))
}

pub fn grid_char(letter: &str, state: &CellState) -> String {
    format_char_with_state(letter, state, " _ ")
}
