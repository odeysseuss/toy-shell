use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct Redir {
    pub stdout_file: Option<(String, bool)>, // (filename, is_append)
    pub stderr_file: Option<(String, bool)>,
    pub combined_file: Option<(String, bool)>,
}

impl Redir {
    pub fn new() -> Self {
        Redir {
            stdout_file: None,
            stderr_file: None,
            combined_file: None,
        }
    }

    pub fn parse(&mut self, toks: Vec<Token>) -> Vec<Token> {
        let mut cmd_toks: Vec<Token> = Vec::new();
        let mut i = 0;

        while i < toks.len() {
            match &toks[i] {
                Token::RedirectOut => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.stdout_file = Some((filename, false));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                Token::AppendOut => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.stdout_file = Some((filename, true));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                Token::RedirectErr => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.stderr_file = Some((filename, false));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                Token::AppendErr => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.stderr_file = Some((filename, true));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                Token::RedirectBoth => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.combined_file = Some((filename, false));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                Token::AppendBoth => {
                    if let Some(filename) = self.get_filename(&toks, i + 1) {
                        self.combined_file = Some((filename, true));
                        i += 2;
                    } else {
                        cmd_toks.push(toks[i].clone());
                        i += 1;
                    }
                }
                _ => {
                    cmd_toks.push(toks[i].clone());
                    i += 1;
                }
            }
        }

        cmd_toks
    }

    fn get_filename(&self, toks: &[Token], index: usize) -> Option<String> {
        if index < toks.len() {
            if let Token::Word(filename) = &toks[index] {
                Some(filename.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}
