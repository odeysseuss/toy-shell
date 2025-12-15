mod cmds;
mod tokenizer;
mod utils;

use crate::cmds::{cd_cmd, cmd_type, exec_ext_cmd};
use crate::tokenizer::tokenize;
use crate::utils::check_ext_cmd;

use std::{
    env,
    io::{self, Write},
};

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

        match toks[0].as_str() {
            "exit" => break,
            "echo" => {
                if toks.len() > 1 {
                    println!("{}", toks[1..].join(" "));
                } else {
                    println!();
                }
            }
            "type" => {
                if toks.len() > 1 {
                    cmd_type(&toks[1]);
                }
            }
            "pwd" => {
                println!(
                    "{}",
                    env::current_dir().expect("Failed to get cwd").display()
                );
            }
            "cd" => {
                if toks.len() > 1 {
                    cd_cmd(&toks[1]);
                }
            }
            _ => {
                let (found, _) = check_ext_cmd(&toks[0]);
                if found {
                    exec_ext_cmd(&toks[0], toks[1..].to_vec());
                } else {
                    println!("{}: command not found", toks[0]);
                }
            }
        }
    }
}
