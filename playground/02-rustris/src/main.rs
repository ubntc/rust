extern crate termion;

use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

mod tile;
mod game;

use game::Game;
use tile::Tile;

// macro_rules! echo {
//     ($($arg:tt)*) => {{
//         write!($($arg)*).unwrap_or_else(|err| {
//             eprintln!("Error writing to stdout: {}", err);
//         });
//     }};
// }

const CLR: &[u8] = b"\x1b[2J\x1b[H";
const CUR_HIDE: &[u8] = b"\x1b[?25l"; // Hide cursor
const CUR_SHOW: &[u8] = b"\x1b[?25h"; // Show cursor
const RST: &[u8] = b"\x1b[0m"; // Reset terminal
const BLD: &[u8] = b"\x1b[1m"; // Bold text
const RED: &[u8] = b"\x1b[31m"; // Red text
const GRN: &[u8] = b"\x1b[32m"; // Green text
const YEL: &[u8] = b"\x1b[33m"; // Yellow text
const BLU: &[u8] = b"\x1b[34m"; // Blue text
const MGT: &[u8] = b"\x1b[35m"; // Magenta text
const CYA: &[u8] = b"\x1b[36m"; // Cyan text
const WHT: &[u8] = b"\x1b[37m"; // White text
const BLK: &[u8] = b"\x1b[30m"; // Black text

struct GameTerm {
    // Add any fields you need for cleanup
    pub term: termion::raw::RawTerminal<std::io::Stdout>,
    pub size: (u16, u16),
}

impl GameTerm {
    pub fn new(size: (usize, usize)) -> Self {
        let term = stdout().into_raw_mode().unwrap();
        stdout().write_all(CUR_HIDE).unwrap();
        GameTerm { term, size: (size.0 as u16, size.1 as u16) }
    }

    fn clear(&self) {
        stdout().write_all(CLR).unwrap();
        self.draw_box(0,0, self.size.0 as usize, self.size.1 as usize);
    }

    fn message(&self, format: String) {
        stdout().write(format.as_bytes()).unwrap();
        stdout().flush().unwrap();
    }

    fn draw_box(&self, x: usize, y: usize, width: usize, height: usize) {
        let mut box_str = String::new();
        for i in 0..height {
            for j in 0..width {
                if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                    box_str.push('#');
                } else {
                    box_str.push(' ');
                }
            }
            box_str.push('\r');
            box_str.push('\n');

        }
        self.message(box_str);
    }
}

impl Drop for GameTerm {
    fn drop(&mut self) {
        println!("\r\nRestoring terminal settings...");

        stdout().write_all(CUR_SHOW).unwrap();

        self.term.suspend_raw_mode().unwrap_or_else(|err| {
            eprintln!("Error restoring terminal settings: {}", err);
        });
    }
}

fn main() {
    let mut game = Game::new();
    let term = GameTerm::new((game.board[0].len(), game.board.len()));


    // Game loop
    loop {
        // Draw the game state
        // ...

        term.message(format!("Score: {}, Press 'q' to quit\r", game.score));

        // Listen for key presses
        for c in stdin().keys() {
            term.clear();

            let k = c.unwrap();
            term.message(format!("Key pressed: {:?}\r", k));

            match k {
                Key::Char('w') | Key::Up => {
                    // Rotate the current tile
                    // ...
                }
                Key::Char('a') | Key::Left => {
                    // Move the current tile to the left
                    // ...
                }
                Key::Char('s') | Key::Down => {
                    // Move the current tile down (fast drop)
                    // ...
                }
                Key::Char('d') | Key::Right => {
                    // Move the current tile to the right
                    // ...
                }
                Key::Char('q') => {
                    // Quit the game
                    return;
                }
                _ => {}
            }
        }

        // Update the game state
        // ...
    }
}

fn echo(arg: &str) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", arg).unwrap_or_else(|err| {
        eprintln!("Error writing to stdout: {}", err);
    });
    stdout.flush().unwrap();
}
