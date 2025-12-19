use crate::cmds::Cmd;
use std::process::exit;

enum RedirState {
    StdOut,
    StdErr,
    StdOutAppend,
    StdErrAppend,
    Nil,
}

struct Redir {
    redir_state: RedirState,
    stdout_file: String,
    stderr_file: String,
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
                        self.stdout_file = toks[i + 1].to_string();
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

    fn handler(&self, cmd: Cmd) {
        if matches!(self.redir_state, RedirState::StdOut) {
            cmd.print_err();
            cmd.write_out(self.stdout_file.clone());
        } else if matches!(self.redir_state, RedirState::StdErr) {
            cmd.print_out();
            cmd.write_err(self.stderr_file.clone());
        } else if matches!(self.redir_state, RedirState::StdOutAppend) {
            cmd.print_err();
            cmd.append_out(self.stdout_file.clone());
        } else if matches!(self.redir_state, RedirState::StdErrAppend) {
            cmd.print_out();
            cmd.append_err(self.stdout_file.clone());
        } else {
            cmd.print();
        }
    }
}

pub fn parse_tokens(toks: Vec<String>) {
    let mut redir: Redir = Redir::new();
    let mut cmd: Cmd = Cmd::new();
    let cmd_toks: Vec<String> = redir.parse(toks);

    match cmd_toks[0].as_str() {
        "exit" => {
            exit(0);
        }
        "echo" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.echo();
            redir.handler(cmd);
        }
        "type" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.types();
            redir.handler(cmd);
        }
        "pwd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.pwd();
            redir.handler(cmd);
        }
        "cd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.cd();
            redir.handler(cmd);
        }
        _ => {
            cmd.name(cmd_toks[0].clone());
            cmd.args(cmd_toks[1..].to_vec());
            cmd.external();
            redir.handler(cmd);
        }
    }
}
