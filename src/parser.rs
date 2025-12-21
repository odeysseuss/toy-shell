use crate::cmds::Cmd;
use crate::tokenizer::tokenize;

#[derive(Debug)]
pub enum RedirState {
    StdOut,
    StdErr,
    StdOutAppend,
    StdErrAppend,
    StdOutErr,
    StdOutErrAppend,
    Nil,
}

#[derive(Debug)]
pub struct Redir {
    pub redir_state: RedirState,
    pub stdout_file: String,
    pub stderr_file: String,
    pub redir_file: String, // for &> and &>>
}

impl Redir {
    fn new() -> Self {
        Redir {
            redir_state: RedirState::Nil,
            stdout_file: String::new(),
            stderr_file: String::new(),
            redir_file: String::new(),
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
                "&>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdOutErr;
                        self.redir_file = toks[i + 1].to_string();
                        i += 2; // skip > and filename
                    } else {
                        i += 1; // skip the redir if no filename
                    }
                }
                "&>>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdOutErrAppend;
                        self.redir_file = toks[i + 1].to_string();
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

#[derive(Debug)]
pub struct Pipe {
    pub commands: Vec<Vec<String>>,
}

impl Pipe {
    pub fn new() -> Self {
        Pipe {
            commands: Vec::new(),
        }
    }

    pub fn parse(&mut self, toks: Vec<String>) -> Vec<String> {
        let mut cur_cmd: Vec<String> = Vec::new();
        let mut cmd_toks: Vec<String> = Vec::new();

        for tok in toks {
            if tok == "|" {
                if !cur_cmd.is_empty() {
                    self.commands.push(cur_cmd);
                    cur_cmd = Vec::new();
                }
            } else {
                cur_cmd.push(tok.clone());
                cmd_toks.push(tok);
            }
        }

        if !cur_cmd.is_empty() {
            if !self.commands.is_empty() {
                self.commands.push(cur_cmd);
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
    cmd_toks = pipe.parse(cmd_toks);
    cmd.handler(cmd_toks, redir, pipe);
}
