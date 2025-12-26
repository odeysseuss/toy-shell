use crate::{handler::cmds::get_builtins, utils::is_exec};
use rustyline::completion::Completer;
use std::env;

pub struct EditHelper;

impl rustyline::Helper for EditHelper {}

impl Completer for EditHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let builtins = get_builtins();

        let (start, word) = extract_word(line, pos);

        let mut candidates: Vec<String> = builtins
            .into_iter()
            .filter(|cmd| cmd.starts_with(&word))
            .map(|s| s.to_string())
            .collect();

        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if file_name.starts_with(&word) {
                                let full_path = entry.path();
                                if full_path.is_file() && is_exec(&full_path) {
                                    if !candidates.iter().any(|c| c == &file_name) {
                                        candidates.push(file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        candidates.sort();

        let candidates: Vec<String> = if candidates.len() == 1 {
            vec![format!("{} ", candidates[0])]
        } else {
            candidates
        };

        Ok((start, candidates))
    }
}

fn extract_word(line: &str, pos: usize) -> (usize, String) {
    if pos == 0 {
        return (0, String::new());
    }

    let mut start = pos;
    while start > 0 && !line[start - 1..start].chars().any(|c| c.is_whitespace()) {
        start -= 1;
    }

    let word = line[start..pos].to_string();
    (start, word)
}

impl rustyline::highlight::Highlighter for EditHelper {}
impl rustyline::validate::Validator for EditHelper {}
impl rustyline::hint::Hinter for EditHelper {
    type Hint = &'static str;
}
