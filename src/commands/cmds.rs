use crate::{
    commands::{
        builtins::{handle_builtins, is_builtin},
        handlers::{handle_pipe, handle_redir},
    },
    parser::parser::Parser,
    tokenizer::Token,
    utils::check_ext_cmd,
};
use std::process::Command;

#[derive(Debug)]
pub struct Cmd {
    pub name: String,
    pub args: Vec<String>,
    pub stdout: String,
    pub stderr: String,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            name: String::new(),
            args: Vec::new(),
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    pub fn handler(&mut self, cmd_toks: Vec<Token>, parser: Parser) {
        let cmd_strings: Vec<String> = cmd_toks.iter().map(|token| token.to_string()).collect();

        if cmd_strings.is_empty() {
            return;
        }

        self.name = cmd_strings[0].clone();
        self.args = if cmd_strings.len() > 1 {
            cmd_strings[1..].to_vec()
        } else {
            Vec::new()
        };

        if parser.pipe.commands.len() < 2 {
            if is_builtin(self.name.clone()) {
                handle_builtins(self);
                handle_redir(self, parser);
            } else {
                let (found, _) = check_ext_cmd(&self.name);
                if found {
                    self.handle_external();
                    handle_redir(self, parser);
                } else {
                    eprintln!("{}: command not found", self.name);
                }
            }
        } else {
            handle_pipe(self, parser);
        }
    }

    fn handle_external(&mut self) {
        let output = Command::new(self.name.clone())
            .args(&self.args)
            .output()
            .expect("Failed to execute");

        self.stdout = format!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
        if !output.stderr.is_empty() {
            self.stderr = format!("{}", String::from_utf8_lossy(&output.stderr).into_owned());
        }
    }
}

pub fn get_builtins() -> Vec<&'static str> {
    vec!["echo", "exit", "type", "pwd", "cd"]
}
