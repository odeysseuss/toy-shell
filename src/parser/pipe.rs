use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct Pipe {
    pub commands: Vec<Vec<Token>>,
}

impl Pipe {
    pub fn new() -> Self {
        Pipe {
            commands: Vec::new(),
        }
    }

    pub fn parse(&mut self, toks: Vec<Token>) -> Vec<Token> {
        let mut cur_cmd: Vec<Token> = Vec::new();
        let mut cmd_toks: Vec<Token> = Vec::new();

        for tok in toks {
            if matches!(tok, Token::Pipe) {
                if !cur_cmd.is_empty() {
                    self.commands.push(cur_cmd);
                    cur_cmd = Vec::new();
                }
            } else {
                cur_cmd.push(tok.clone());
                cmd_toks.push(tok);
            }
        }

        if !cur_cmd.is_empty() {
            self.commands.push(cur_cmd);
        }

        return cmd_toks;
    }
}
