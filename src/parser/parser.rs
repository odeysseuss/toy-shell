use crate::{
    parser::{pipe::Pipe, redir::Redir},
    tokenizer::{Token, tokenize},
};

#[derive(Debug, Clone)]
pub struct Parser {
    pub redir: Redir,
    pub pipe: Pipe,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            redir: Redir::new(),
            pipe: Pipe::new(),
        }
    }

    pub fn parse(&mut self, cmd: String) -> Vec<Token> {
        let toks: Vec<Token> = tokenize(cmd.trim());
        if toks.is_empty() {
            return Vec::new();
        }

        let mut cmd_toks = self.pipe.parse(toks.clone());
        cmd_toks = self.redir.parse(cmd_toks);
        return cmd_toks;
    }
}
