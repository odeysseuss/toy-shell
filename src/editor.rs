use crate::commands::cmds::get_builtins;
use rustyline::completion::Completer;

pub struct EditHelper;

impl rustyline::Helper for EditHelper {}

impl Completer for EditHelper {
    type Candidate = String; // Change from &'static str to String

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let builtins = get_builtins();
        let prefix = &line[..pos];

        let candidates: Vec<String> = builtins
            .into_iter()
            .filter(|&cmd| cmd.starts_with(prefix))
            .map(|cmd| format!("{} ", cmd))
            .collect();

        if !candidates.is_empty() {
            return Ok((0, candidates));
        }

        Ok((0, vec![]))
    }
}

impl rustyline::highlight::Highlighter for EditHelper {}
impl rustyline::validate::Validator for EditHelper {}
impl rustyline::hint::Hinter for EditHelper {
    type Hint = &'static str;
}
