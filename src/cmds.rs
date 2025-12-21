use crate::parser::{Pipe, Redir, RedirState};
use crate::utils::{append_to_file, check_ext_cmd, write_to_file};
use libc::{STDIN_FILENO, STDOUT_FILENO, close, dup2, fork, pipe, waitpid};
use std::os::raw::c_int;
use std::os::unix::process::CommandExt;
use std::process::exit;
use std::ptr::null_mut;
use std::{env, path::Path, process::Command};

#[derive(Debug)]
pub struct Cmd {
    name: String,
    args: Vec<String>,
    stdout: String,
    stderr: String,
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

    pub fn print_out(&self) {
        if self.stdout.is_empty() {
            print!("{}", "".to_string());
        } else {
            print!("{}", self.stdout);
        }
    }

    pub fn print_err(&self) {
        if self.stderr.is_empty() {
            print!("{}", "".to_string());
        } else {
            print!("{}", self.stderr);
        }
    }

    pub fn print(&self) {
        self.print_out();
        self.print_err();
    }

    pub fn write_out(&self, filename: String) {
        if self.stdout.is_empty() {
            write_to_file("".as_bytes().to_vec(), filename);
        } else {
            write_to_file(self.stdout.as_bytes().to_vec(), filename);
        }
    }

    pub fn write_err(&self, filename: String) {
        if self.stderr.is_empty() {
            write_to_file("".as_bytes().to_vec(), filename);
        } else {
            write_to_file(self.stderr.as_bytes().to_vec(), filename);
        }
    }

    pub fn append_out(&self, filename: String) {
        if self.stdout.is_empty() {
            append_to_file("".as_bytes().to_vec(), filename);
        } else {
            append_to_file(self.stdout.as_bytes().to_vec(), filename);
        }
    }

    pub fn append_err(&self, filename: String) {
        if self.stderr.is_empty() {
            append_to_file("".as_bytes().to_vec(), filename);
        } else {
            append_to_file(self.stderr.as_bytes().to_vec(), filename);
        }
    }

    pub fn echo(&mut self) {
        self.name = "echo".to_string();
        if self.args.is_empty() {
            self.stdout = "\n".to_string();
        }
        self.stdout = self.args.join(" ") + "\n";
    }

    pub fn is_builtin(&self, cmd: String) -> bool {
        let builtins: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];
        if builtins.contains(&cmd.as_str()) {
            true
        } else {
            false
        }
    }

    pub fn types(&mut self) {
        self.name = "type".to_string();
        if let Some(cmd) = self.args.first().clone() {
            if self.is_builtin(cmd.to_string()) {
                self.stdout = format!("{} is a shell builtin\n", cmd);
            } else {
                let (found, full_path) = check_ext_cmd(&cmd);
                if found {
                    self.stdout = format!("{} is {}\n", cmd, full_path.unwrap().display());
                } else {
                    self.stderr = format!("{} not found\n", cmd);
                }
            }
        }
    }

    pub fn pwd(&mut self) {
        self.name = "pwd".to_string();
        match env::current_dir() {
            Ok(path) => {
                self.stdout = path.display().to_string() + "\n";
            }
            Err(e) => {
                self.stderr = format!("Failed to cwd: {}\n", e);
            }
        }
    }

    pub fn cd(&mut self) {
        self.name = "cd".to_string();
        if let Some(dir) = self.args.first() {
            if dir == "~" {
                match env::var("HOME") {
                    Ok(val) => {
                        env::set_current_dir(val).expect("Failed to change dir");
                    }
                    Err(e) => {
                        self.stderr = format!("{}", e);
                    }
                }
            } else {
                let path: &Path = Path::new(dir);
                if path.exists() {
                    env::set_current_dir(dir).expect("Failed to change dir");
                } else {
                    self.stderr = format!("cd: {}: No such file or directory\n", dir);
                }
            }
        }
    }

    pub fn builtins(&mut self) {
        match self.name.as_str() {
            "exit" => {
                exit(0);
            }
            "echo" => {
                self.echo();
            }
            "type" => {
                self.types();
            }
            "pwd" => {
                self.pwd();
            }
            "cd" => {
                self.cd();
            }
            _ => eprintln!("Unknown"),
        }
    }

    pub fn external(&mut self) {
        let output = Command::new(self.name.clone())
            .args(&self.args)
            .output()
            .expect("Failed to execute");

        self.stdout = format!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
        if !output.stderr.is_empty() {
            self.stderr = format!("{}", String::from_utf8_lossy(&output.stderr).into_owned());
        }
    }

    pub fn handler(&mut self, cmd_toks: Vec<String>, redir: Redir, pipeline: Pipe) {
        self.name = cmd_toks[0].clone();
        self.args = cmd_toks[1..].to_vec();
        if pipeline.cmd.is_empty() {
            if self.is_builtin(self.name.clone()) {
                self.builtins();
                self.handle_redir(redir);
            } else {
                let (found, _) = check_ext_cmd(&self.name);
                if found {
                    self.external();
                    self.handle_redir(redir);
                } else {
                    eprintln!("{}: command not found", self.name);
                }
            }
        } else {
            if self.is_builtin(pipeline.cmd.clone()) {
                self.name = pipeline.cmd;
                self.args = pipeline.args;
                self.builtins();
                self.handle_redir(redir);
            } else {
                let (found, _) = check_ext_cmd(&pipeline.cmd);
                if found {
                    self.handle_ext_cmd_pipe(pipeline);
                } else {
                    eprintln!("{}: command not found", pipeline.cmd);
                }
            }
        }
    }

    fn handle_ext_cmd_pipe(&mut self, pipeline: Pipe) {
        unsafe {
            let mut fds: [c_int; 2] = [0; 2];
            if pipe(fds.as_mut_ptr()) == -1 {
                eprintln!("Pipe failed");
            }
            let reader = fds[0];
            let writer = fds[1];

            let p1 = fork();
            if p1 < 0 {
                eprintln!("Fork failed");
                close(reader);
                close(writer);
                return;
            }

            if p1 == 0 {
                dup2(writer, STDOUT_FILENO);
                close(reader);
                close(writer);

                let _ = Command::new(self.name.clone())
                    .args(self.args.clone())
                    .exec();

                eprintln!("Failed to execute {}", self.name);
                exit(1);
            }

            let p2 = fork();
            if p2 < 0 {
                eprintln!("Fork failed");
                waitpid(p1, null_mut(), 0);
                return;
            }

            if p2 == 0 {
                dup2(reader, STDIN_FILENO);
                close(writer);
                close(reader);

                let _ = Command::new(pipeline.cmd.clone())
                    .args(pipeline.args)
                    .exec();

                eprintln!("Failed to execute {}", pipeline.cmd.clone());
                exit(1);
            }

            close(reader);
            close(writer);
            waitpid(p1, null_mut(), 0);
            waitpid(p2, null_mut(), 0);
        }
    }

    fn handle_redir(&self, redir: Redir) {
        match redir.redir_state {
            RedirState::StdOut => {
                self.print_err();
                self.write_out(redir.stdout_file.clone());
            }
            RedirState::StdErr => {
                self.print_out();
                self.write_err(redir.stderr_file.clone());
            }
            RedirState::StdOutAppend => {
                self.print_err();
                self.append_out(redir.stdout_file.clone());
            }
            RedirState::StdErrAppend => {
                self.print_out();
                self.append_err(redir.stderr_file.clone());
            }
            _ => {
                self.print();
            }
        }
    }
}
