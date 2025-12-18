use crate::cmds::Cmd;
use std::process::exit;

enum RedirState {
    StdOut,
    StdErr,
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

    fn handler(&mut self, toks: Vec<String>) -> Vec<String> {
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
                "2>" => {
                    if i + 1 < toks.len() {
                        self.redir_state = RedirState::StdErr;
                        self.stderr_file = toks[i + 1].to_string();
                        i += 2;
                    } else {
                        i += 1;
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

pub fn parse_tokens(toks: Vec<String>) {
    let mut redir: Redir = Redir::new();
    let mut cmd: Cmd = Cmd::new();
    let cmd_toks: Vec<String> = redir.handler(toks);

    match cmd_toks[0].as_str() {
        "exit" => {
            exit(0);
        }
        "echo" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.echo();
            if matches!(redir.redir_state, RedirState::StdOut) {
                cmd.print_err();
                cmd.write_out(redir.stdout_file);
            } else if matches!(redir.redir_state, RedirState::StdErr) {
                cmd.print_out();
                cmd.write_err(redir.stderr_file);
            } else {
                cmd.print_out();
            }
        }
        "type" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.types();
            if matches!(redir.redir_state, RedirState::StdOut) {
                cmd.print_err();
                cmd.write_out(redir.stdout_file);
            } else if matches!(redir.redir_state, RedirState::StdErr) {
                cmd.print_out();
                cmd.write_err(redir.stderr_file);
            } else {
                cmd.print_out();
            }
        }
        "pwd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.pwd();
            if matches!(redir.redir_state, RedirState::StdOut) {
                cmd.print_err();
                cmd.write_out(redir.stdout_file);
            } else if matches!(redir.redir_state, RedirState::StdErr) {
                cmd.print_out();
                cmd.write_err(redir.stderr_file);
            } else {
                cmd.print_out();
            }
        }
        "cd" => {
            cmd.args(cmd_toks[1..].to_vec());
            cmd.cd();
            if matches!(redir.redir_state, RedirState::StdOut) {
                cmd.print_err();
                cmd.write_out(redir.stdout_file);
            } else if matches!(redir.redir_state, RedirState::StdErr) {
                cmd.print_out();
                cmd.write_err(redir.stderr_file);
            } else {
                cmd.print_out();
            }
        }
        _ => {
            cmd.name(cmd_toks[0]);
            cmd.args(cmd_toks[1..].to_vec());
            cmd.external();
            if matches!(redir.redir_state, RedirState::StdOut) {
                cmd.print_err();
                cmd.write_out(redir.stdout_file);
            } else if matches!(redir.redir_state, RedirState::StdErr) {
                cmd.print_out();
                cmd.write_err(redir.stderr_file);
            } else {
                cmd.print_out();
            }
        }
    }
}
