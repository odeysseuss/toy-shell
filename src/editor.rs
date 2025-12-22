use rustyline::completion::Completer;

pub struct EditHelper;

impl rustyline::Helper for EditHelper {}

impl Completer for EditHelper {
    type Candidate = &'static str;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if "echo ".starts_with(&line[..pos]) {
            return Ok((0, vec!["echo "]));
        }
        if "type ".starts_with(&line[..pos]) {
            return Ok((0, vec!["type "]));
        }
        if "pwd ".starts_with(&line[..pos]) {
            return Ok((0, vec!["pwd "]));
        }
        if "cd ".starts_with(&line[..pos]) {
            return Ok((0, vec!["cd "]));
        }
        if "exit ".starts_with(&line[..pos]) {
            return Ok((0, vec!["exit "]));
        }
        Ok((0, vec![]))
    }
}

impl rustyline::highlight::Highlighter for EditHelper {}
impl rustyline::validate::Validator for EditHelper {}
impl rustyline::hint::Hinter for EditHelper {
    type Hint = &'static str;
}
