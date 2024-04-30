mod player;
mod map;
mod lexer;

use player::Player;
use map::Map;
use lexer::{Lexer, Token};

pub fn run() {
    let input = std::fs::read_to_string("player.blind.example").unwrap();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();

    println!("{:?}", tokens);
}
