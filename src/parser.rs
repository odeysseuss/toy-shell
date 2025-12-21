use crate::cmds::Cmd;
use crate::tokenizer::tokenize;
use std::process::exit;

#[derive(Debug)]
pub enum RedirState {
    StdOut,
    StdErr,
    StdOutAppend,
    StdErrAppend,
    Nil,
}

#[derive(Debug)]
pub struct Redir {
    pub redir_state: RedirState,
    pub stdout_file: String,
    pub stderr_file: String,
}

impl Redir {
    fn new() -> Self {
        Redir {
            redir_state: RedirState::Nil,
            stdout_file: String::new(),
            stderr_file: String::new(),
        }
    }

    fn parse(&mut self, toks: Vec<String>) -> Vec<String> {
        let mut cmd_toks: Vec<String> = Vec::new();
        let mut i = 0;
        while i < toks.len() {
            match toks[i].as_str() {
                ">" | "1>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdOut;
                        self.stdout_file = toks[i + 1].to_string();
                        i += 2; // skip > and filename
                    } else {
                        i += 1; // skip the redir if no filename
                    }
                }
                ">>" | "1>>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdOutAppend;
                        self.stdout_file = toks[i + 1].to_string();
                        i += 2; // skip > and filename
                    } else {
                        i += 1; // skip the redir if no filename
                    }
                }
                "2>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdErr;
                        self.stderr_file = toks[i + 1].to_string();
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "2>>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdErrAppend;
                        self.stderr_file = toks[i + 1].to_string();
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
        return cmd_toks;
    }
}

pub struct Pipe {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Pipe {
    pub fn new() -> Self {
        Pipe {
            cmd: String::new(),
            args: Vec::new(),
        }
    }

    pub fn parse_pipes(&mut self, toks: Vec<String>) -> Vec<String> {
        let mut cmd_toks: Vec<String> = Vec::new();
        let mut i = 0;
        while i < toks.len() {
            match toks[i].as_str() {
                "|" => {
                    if i + 1 < toks.len() {
                        self.cmd = toks[i + 1].to_string();
                        i += 2; // skip | and cmd
                        while i < toks.len() {
                            self.args.push(toks[i].to_string());
                            i += 1;
                        }
                        break;
                    } else {
                        i += 1; // skip the pipe if no cmd
                    }
                }
                _ => {
                    cmd_toks.push(toks[i].clone());
                    i += 1;
                }
            }
        }
        return cmd_toks;
    }
}

pub fn evaluate(command: String) {
    let toks: Vec<String> = tokenize(command.trim());
    if toks.is_empty() {
        return;
    }

    let mut redir: Redir = Redir::new();
    let mut cmd: Cmd = Cmd::new();
    let mut pipe: Pipe = Pipe::new();
    let mut cmd_toks: Vec<String> = redir.parse(toks);
    cmd_toks = pipe.parse_pipes(cmd_toks);

    match cmd_toks[0].as_str() {
        "exit" => {
            exit(0);
        }
        "echo" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.echo();
            cmd.handler(redir);
        }
        "type" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.types();
            cmd.handler(redir);
        }
        "pwd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.pwd();
            cmd.handler(redir);
        }
        "cd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.cd();
            cmd.handler(redir);
        }
        _ => {
            cmd.name(cmd_toks[0].clone());
            cmd.args(cmd_toks[1..].to_vec());
            cmd.external(pipe);
            cmd.handler(redir);
        }
    }
}
