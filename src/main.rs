mod cmds;
mod editor;
mod parser;
mod tokenizer;
mod utils;

use crate::parser::evaluate;
use editor::EditHelper;
use rustyline::{Editor, error::ReadlineError};

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
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
