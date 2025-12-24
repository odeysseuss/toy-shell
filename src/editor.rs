use crate::{commands::cmds::get_builtins, utils::is_exec};
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
        let prefix = &line[..pos];

        let mut candidates: Vec<String> = builtins
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| format!("{} ", cmd))
            .collect();

        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if file_name.starts_with(prefix) {
                                let full_path = entry.path();
                                if full_path.is_file() && is_exec(&full_path) {
                                    if !candidates.iter().any(|c| c.trim_end() == file_name) {
                                        candidates.push(format!("{} ", file_name));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((0, candidates))
    }
}

impl rustyline::highlight::Highlighter for EditHelper {}
impl rustyline::validate::Validator for EditHelper {}
impl rustyline::hint::Hinter for EditHelper {
    type Hint = &'static str;
}
