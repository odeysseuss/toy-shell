use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(unix)]
fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    true
}

fn check_ext_cmd(cmd: &str) -> (bool, Option<PathBuf>) {
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let full_path: PathBuf = dir.join(cmd);
            if full_path.exists() && full_path.is_file() && is_exec(&full_path) {
                return (true, Some(full_path));
            }
        }
    }
    return (false, None);
}

fn cmd_type(cmd: &str) {
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

fn exec_ext_cmd(cmd: &str, args: Vec<&str>) {
    // check_ext_cmd will be called later
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .expect("Failed to execute");
    print!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
}

fn cd_cmd(dir: &str) {
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

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let toks: Vec<&str> = command.trim().split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }

        match toks[0] {
            "exit" => break,
            "echo" => {
                if toks.len() > 1 {
                    println!("{}", toks[1..].join(" "));
                } else {
                    println!();
                }
            }
            "type" => {
                if toks.len() > 1 {
                    cmd_type(toks[1]);
                }
            }
            "pwd" => {
                println!(
                    "{}",
                    env::current_dir().expect("Failed to get cwd").display()
                );
            }
            "cd" => {
                if toks.len() > 1 {
                    cd_cmd(toks[1]);
                }
            }
            _ => {
                let (found, _) = check_ext_cmd(&toks[0]);
                if found {
                    exec_ext_cmd(&toks[0], toks[1..].to_vec());
                } else {
                    println!("{}: command not found", toks[0]);
                }
            }
        }
    }
}
