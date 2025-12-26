use crate::{
    handler::cmds::{Cmd, get_builtins},
    utils::check_ext_cmd,
};
use std::{env, path::Path, process::exit};

pub fn handle_builtins(cmd: &mut Cmd) {
    match cmd.name.as_str() {
        "exit" => exit(0),
        "echo" => echo(cmd),
        "type" => types(cmd),
        "pwd" => pwd(cmd),
        "cd" => cd(cmd),
        _ => eprintln!("Unknown builtins"),
    }
}

pub fn is_builtin(cmd: String) -> bool {
    let builtins: Vec<&'static str> = get_builtins();
    if builtins.contains(&cmd.as_str()) {
        true
    } else {
        false
    }
}

fn echo(cmd: &mut Cmd) {
    cmd.name = "echo".to_string();
    if cmd.args.is_empty() {
        cmd.stdout = "\n".to_string();
    }
    cmd.stdout = cmd.args.join(" ") + "\n";
}

fn types(cmd: &mut Cmd) {
    cmd.name = "type".to_string();
    if let Some(exec) = cmd.args.first().clone() {
        if is_builtin(exec.to_string()) {
            cmd.stdout = format!("{} is a shell builtin\n", exec);
        } else {
            let (found, full_path) = check_ext_cmd(&exec);
            if found {
                cmd.stdout = format!("{} is {}\n", exec, full_path.unwrap().display());
            } else {
                cmd.stderr = format!("{} not found\n", exec);
            }
        }
    }
}

fn pwd(cmd: &mut Cmd) {
    cmd.name = "pwd".to_string();
    match env::current_dir() {
        Ok(path) => {
            cmd.stdout = path.display().to_string() + "\n";
        }
        Err(e) => {
            cmd.stderr = format!("Failed to cwd: {}\n", e);
        }
    }
}

fn cd(cmd: &mut Cmd) {
    cmd.name = "cd".to_string();
    if let Some(dir) = cmd.args.first() {
        if dir == "~" {
            match env::var("HOME") {
                Ok(val) => {
                    env::set_current_dir(val).expect("Failed to change dir");
                }
                Err(e) => {
                    cmd.stderr = format!("{}", e);
                }
            }
        } else {
            let path: &Path = Path::new(dir);
            if path.exists() {
                env::set_current_dir(dir).expect("Failed to change dir");
            } else {
                cmd.stderr = format!("cd: {}: No such file or directory\n", dir);
            }
        }
    }
}
