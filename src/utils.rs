use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[cfg(unix)]
pub fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
pub fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    true
}

pub fn check_ext_cmd(cmd: &str) -> (bool, Option<PathBuf>) {
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

pub fn tokenize(input: &str) -> Vec<String> {
    let mut toks = Vec::new();
    let mut cur_tok = String::new();
    let mut escape = false;
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;

    for ch in input.chars() {
        if escape {
            cur_tok.push(ch);
            escape = false;
            continue;
        }

        match ch {
            '\\' => escape = true,
            '\'' => {
                if !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else {
                    cur_tok.push(ch);
                }
            }
            '"' => {
                if !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                } else {
                    cur_tok.push(ch);
                }
            }
            ' ' | '\t' | '\n' => {
                if !in_single_quotes && !in_double_quotes {
                    if !cur_tok.is_empty() {
                        toks.push(cur_tok.clone());
                        cur_tok.clear();
                    }
                } else {
                    cur_tok.push(ch);
                }
            }
            _ => cur_tok.push(ch),
        }
    }

    if !cur_tok.is_empty() {
        toks.push(cur_tok);
    }

    return toks;
}
