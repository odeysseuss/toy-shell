use crate::utils::{append_to_file, check_ext_cmd, write_to_file};
use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};

pub struct Cmd {
    name: String,
    args: Vec<String>,
    pipe: String,
    pipe_args: Vec<String>,
    stdout: String,
    stderr: String,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            name: String::new(),
            args: Vec::new(),
            pipe: String::new(),
            pipe_args: Vec::new(),
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn args(&mut self, args: Vec<String>) {
        self.args = args;
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

    pub fn write_out(self, filename: String) {
        if self.stdout.is_empty() {
            write_to_file("".as_bytes().to_vec(), filename);
        } else {
            write_to_file(self.stdout.as_bytes().to_vec(), filename);
        }
    }

    pub fn write_err(self, filename: String) {
        if self.stderr.is_empty() {
            write_to_file("".as_bytes().to_vec(), filename);
        } else {
            write_to_file(self.stderr.as_bytes().to_vec(), filename);
        }
    }

    pub fn append_out(self, filename: String) {
        if self.stdout.is_empty() {
            append_to_file("".as_bytes().to_vec(), filename);
        } else {
            append_to_file(self.stdout.as_bytes().to_vec(), filename);
        }
    }

    pub fn append_err(self, filename: String) {
        if self.stderr.is_empty() {
            append_to_file("".as_bytes().to_vec(), filename);
        } else {
            append_to_file(self.stderr.as_bytes().to_vec(), filename);
        }
    }

    pub fn parse_pipes(&mut self, toks: Vec<String>) -> Vec<String> {
        let mut cmd_toks: Vec<String> = Vec::new();
        let mut i = 0;
        while i < toks.len() {
            match toks[i].as_str() {
                "|" => {
                    if i + 1 < toks.len() {
                        self.pipe = toks[i + 1].to_string();
                        i += 2; // skip | and cmd
                        while i < toks.len() {
                            self.pipe_args.push(toks[i].to_string());
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

    pub fn echo(&mut self) {
        self.name = "echo".to_string();
        if self.args.is_empty() {
            self.stdout = "\n".to_string();
        }
        self.stdout = self.args.join(" ") + "\n";
    }

    pub fn types(&mut self) {
        self.name = "type".to_string();
        let builtins: [&str; 4] = ["echo", "exit", "type", "pwd"];
        if let Some(cmd) = self.args.first().clone() {
            if builtins.contains(&cmd.as_str()) {
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

    pub fn external(&mut self) {
        let (found, _) = check_ext_cmd(&self.name);
        if found {
            if self.pipe.is_empty() {
                let cmd = Command::new(self.name.clone())
                    .args(&self.args)
                    .output()
                    .expect("Failed to execute");

                self.stdout = format!("{}", String::from_utf8_lossy(&cmd.stdout).into_owned());
                if !cmd.stderr.is_empty() {
                    self.stderr = format!("{}", String::from_utf8_lossy(&cmd.stderr).into_owned());
                }
            } else {
                let mut cmd = Command::new(self.name.clone())
                    .args(&self.args)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute");

                let cmd_out = cmd.stdout.take().expect("Failed to capture output");

                let pipe_cmd = Command::new(self.pipe.clone())
                    .args(&self.pipe_args)
                    .stdin(cmd_out)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute");

                let output = pipe_cmd
                    .wait_with_output()
                    .expect("Failed to excute cmd with pipe");

                self.stdout = format!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
                if !output.stderr.is_empty() {
                    self.stderr =
                        format!("{}", String::from_utf8_lossy(&output.stderr).into_owned());
                }
            }
        } else {
            eprintln!("{}: command not found", self.name);
        }
    }
}
