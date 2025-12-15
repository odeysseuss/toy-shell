use crate::utils::check_ext_cmd;
use std::{env, path::Path, process::Command};

pub fn cmd_type(cmd: &str) {
    let builtins: [&str; 4] = ["echo", "exit", "type", "pwd"];
    if builtins.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else {
        let (found, full_path) = check_ext_cmd(&cmd);
        if found {
            println!("{} is {}", cmd, full_path.unwrap().display());
        } else {
            println!("{} not found", cmd);
        }
    }
}

pub fn exec_ext_cmd(cmd: &str, args: Vec<String>) {
    // check_ext_cmd will be called later
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .expect("Failed to execute");
    print!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
}

pub fn cd_cmd(dir: &str) {
    if dir == "~" {
        match env::var("HOME") {
            Ok(val) => {
                env::set_current_dir(val).expect("Failed to change dir");
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    } else {
        let path: &Path = Path::new(dir);
        if path.exists() {
            env::set_current_dir(dir).expect("Failed to change dir");
        } else {
            println!("cd: {}: No such file or directory", dir);
        }
    }
}
