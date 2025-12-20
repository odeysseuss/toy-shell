mod cmds;
mod parser;
mod tokenizer;
mod utils;

use crate::parser::evaluate;
use crate::tokenizer::tokenize;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let toks: Vec<String> = tokenize(command.trim());
        if toks.is_empty() {
            continue;
        }
        evaluate(toks);
    }
}
