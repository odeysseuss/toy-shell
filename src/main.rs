use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
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

fn cmd_type(cmd: &str) {
    let builtins: [&str; 3] = ["echo", "exit", "type"];
    if builtins.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else {
        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                let full_path: PathBuf = dir.join(cmd);
                if full_path.exists() && full_path.is_file() && is_exec(&full_path) {
                    println!("{} is {}", cmd, full_path.display());
                    return;
                }
            }
        }
        println!("{} not found", cmd);
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let toks: Vec<&str> = command.trim().split_whitespace().collect();
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
            _ => println!("{}: command not found", toks[0]),
        }
    }
}
