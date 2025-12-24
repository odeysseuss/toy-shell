mod commands;
mod editor;
mod parser;
mod tokenizer;
mod utils;

use crate::{commands::cmds::Cmd, parser::parser::Parser};
use editor::EditHelper;
use rustyline::{Editor, error::ReadlineError};
use std::process::exit;

pub fn evaluate(command: String) {
    let mut parser = Parser::new();
    let cmd_toks = parser.parse(command);
    let mut cmd = Cmd::new();
    cmd.handler(cmd_toks, parser);
}

fn main() -> rustyline::Result<()> {
    let mut editor: Editor<EditHelper, _> = Editor::new()?;
    editor.set_helper(Some(EditHelper));

    loop {
        let readline = editor.readline("$ ");
        match readline {
            Ok(line) => {
                evaluate(line);
            }
            Err(ReadlineError::Interrupted) => {
                exit(130);
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
