mod cmds;
mod parser;
mod tokenizer;
mod utils;

use crate::parser::evaluate;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        evaluate(command);
    }
}
