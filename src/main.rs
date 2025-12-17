mod cmds;
mod tokenizer;
mod utils;

use crate::cmds::{cd_cmd, cmd_echo, cmd_pwd, cmd_type, exec_ext_cmd};
use crate::tokenizer::tokenize;
use crate::utils::{check_ext_cmd, write_to_file};

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

        let mut cmd_toks: Vec<String> = Vec::new();
        let mut stdout_file = None;

        let mut i = 0;
        while i < toks.len() {
            match toks[i].as_str() {
                ">" | "1>" => {
                    if i + 1 < toks.len() {
                        stdout_file = Some(toks[i + 1].clone());
                        i += 2; // skip > and filename
                    } else {
                        i += 1; // skip the redir if no filename
                    }
                }
                _ => {
                    cmd_toks.push(toks[i].clone());
                    i += 1;
                }
            }
        }

        match cmd_toks[0].as_str() {
            "exit" => break,
            "echo" => {
                let output = cmd_echo(cmd_toks[1..].to_vec());
                write_to_file(output, &stdout_file);
            }
            "type" => {
                if cmd_toks.len() > 1 {
                    let output = cmd_type(&cmd_toks[1]);
                    write_to_file(output, &stdout_file);
                }
            }
            "pwd" => {
                let output = cmd_pwd();
                write_to_file(output, &stdout_file);
            }
            "cd" => {
                if cmd_toks.len() > 1 {
                    cd_cmd(&cmd_toks[1]);
                }
            }
            _ => {
                let (found, _) = check_ext_cmd(&cmd_toks[0]);
                if found {
                    let output = exec_ext_cmd(&cmd_toks[0], cmd_toks[1..].to_vec());
                    write_to_file(output, &stdout_file);
                } else {
                    println!("{}: command not found", cmd_toks[0]);
                }
            }
        }
    }
}
