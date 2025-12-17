use crate::utils::check_ext_cmd;
use std::{env, path::Path, process::Command};

#[must_use]
pub fn cmd_echo(args: Vec<String>) -> Vec<u8> {
    if args.is_empty() {
        return "\n".as_bytes().to_vec();
    } else {
        let output = args.join(" ") + "\n";
        return output.as_bytes().to_vec();
    }
}

#[must_use]
pub fn cmd_type(cmd: &str) -> Vec<u8> {
    let builtins: [&str; 4] = ["echo", "exit", "type", "pwd"];
    if builtins.contains(&cmd) {
        return format!("{} is a shell builtin", cmd).as_bytes().to_vec();
    } else {
        let (found, full_path) = check_ext_cmd(&cmd);
        if found {
            return format!("{} is {}", cmd, full_path.unwrap().display())
                .as_bytes()
                .to_vec();
        } else {
            return format!("{} not found", cmd).as_bytes().to_vec();
        }
    }
}

#[must_use]
pub fn cmd_pwd() -> Vec<u8> {
    match env::current_dir() {
        Ok(path) => {
            let output = path.display().to_string() + "\n";
            return output.as_bytes().to_vec();
        }
        Err(e) => {
            return format!("Failed to cwd: {}\n", e).as_bytes().to_vec();
        }
    }
}

#[must_use]
pub fn exec_ext_cmd(cmd: &str, args: Vec<String>) -> Vec<u8> {
    // check_ext_cmd will be called later
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .expect("Failed to execute");
    return format!("{}", String::from_utf8_lossy(&output.stdout).into_owned())
        .as_bytes()
        .to_vec();
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
