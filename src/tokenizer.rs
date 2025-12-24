#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Pipe,         // |
    DoublePipe,   // ||
    And,          // &
    DoubleAnd,    // &&
    RedirectOut,  // >
    AppendOut,    // >>
    RedirectErr,  // 2>
    AppendErr,    // 2>>
    RedirectBoth, // &>
    AppendBoth,   // &>>
    Semicolon,    // ;
    LeftParen,    // (
    RightParen,   // )
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Word(s) => s.clone(),
            Token::Pipe => "|".to_string(),
            Token::DoublePipe => "||".to_string(),
            Token::And => "&".to_string(),
            Token::DoubleAnd => "&&".to_string(),
            Token::RedirectOut => ">".to_string(),
            Token::AppendOut => ">>".to_string(),
            Token::RedirectErr => "2>".to_string(),
            Token::AppendErr => "2>>".to_string(),
            Token::RedirectBoth => "&>".to_string(),
            Token::AppendBoth => "&>>".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let string_tokens = tokenize_to_strings(input);
    convert_to_tokens(&string_tokens)
}

fn tokenize_to_strings(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;

    #[derive(Debug)]
    enum State {
        Unquoted,
        SingleQuoted,
        DoubleQuoted,
    }

    let mut state = State::Unquoted;

    while i < chars.len() {
        let ch = chars[i];

        match state {
            State::Unquoted => {
                if ch == '\'' {
                    state = State::SingleQuoted;
                    i += 1;
                } else if ch == '"' {
                    state = State::DoubleQuoted;
                    i += 1;
                } else if ch == '\\' {
                    i += 1;
                    if i < chars.len() {
                        current_token.push(chars[i]);
                        i += 1;
                    }
                } else if ch.is_whitespace() {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    i += 1;
                } else if is_operator_start(ch) {
                    // handle operators
                    let operator = extract_operator(&chars[i..]);
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(operator.clone());
                    i += operator.len();
                } else if ch == '1' || ch == '2' {
                    if i + 1 < chars.len() && chars[i + 1] == '>' {
                        let mut operator = format!("{}>", ch);
                        if i + 2 < chars.len() && chars[i + 2] == '>' {
                            operator.push('>');
                        }
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                        tokens.push(operator.clone());
                        i += operator.len();
                    } else {
                        current_token.push(ch);
                        i += 1;
                    }
                } else {
                    current_token.push(ch);
                    i += 1;
                }
            }
            State::SingleQuoted => {
                if ch == '\'' {
                    state = State::Unquoted;
                    i += 1;
                } else {
                    current_token.push(ch);
                    i += 1;
                }
            }
            State::DoubleQuoted => {
                if ch == '"' {
                    state = State::Unquoted;
                    i += 1;
                } else if ch == '\\' {
                    i += 1;
                    if i < chars.len() {
                        let next_ch = chars[i];
                        if matches!(next_ch, '\\' | '"' | '$' | '`' | '\n') {
                            if next_ch != '\n' {
                                current_token.push(next_ch);
                            }
                            i += 1;
                        } else {
                            current_token.push('\\');
                        }
                    }
                } else {
                    current_token.push(ch);
                    i += 1;
                }
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn is_operator_start(ch: char) -> bool {
    matches!(ch, '>' | '&' | '|' | ';' | '(' | ')')
}

fn extract_operator(chars: &[char]) -> String {
    if chars.is_empty() {
        return String::new();
    }

    let first = chars[0];

    match first {
        // check for file descriptor redirects (1>, 2>, etc.)
        '1' | '2' => {
            if chars.len() > 1 && chars[1] == '>' {
                let mut result = format!("{}>", first);
                if chars.len() > 2 && chars[2] == '>' {
                    result.push('>');
                }
                return result;
            }
            // not a redirect operator, just a digit
            first.to_string()
        }
        // simple redirects
        '>' => {
            let mut result = ">".to_string();
            if chars.len() > 1 && chars[1] == '>' {
                result.push('>');
            }
            result
        }
        // & can be: &, &&, &>, or &>>
        '&' => {
            if chars.len() > 1 {
                match chars[1] {
                    '&' => "&&".to_string(),
                    '>' => {
                        let mut result = "&>".to_string();
                        if chars.len() > 2 && chars[2] == '>' {
                            result.push('>');
                        }
                        result
                    }
                    _ => "&".to_string(),
                }
            } else {
                "&".to_string()
            }
        }
        // pipe operators
        '|' => {
            if chars.len() > 1 && chars[1] == '|' {
                "||".to_string()
            } else {
                "|".to_string()
            }
        }
        // single character operators
        ';' | '(' | ')' => first.to_string(),
        // default case (shouldn't happen with is_operator_start)
        _ => first.to_string(),
    }
}

fn convert_to_tokens(string_tokens: &[String]) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in string_tokens {
        match token.as_str() {
            "|" => tokens.push(Token::Pipe),
            "||" => tokens.push(Token::DoublePipe),
            "&" => tokens.push(Token::And),
            "&&" => tokens.push(Token::DoubleAnd),
            ">" => tokens.push(Token::RedirectOut),
            "1>" => tokens.push(Token::RedirectOut),
            ">>" => tokens.push(Token::AppendOut),
            "1>>" => tokens.push(Token::AppendOut),
            "2>" => tokens.push(Token::RedirectErr),
            "2>>" => tokens.push(Token::AppendErr),
            "&>" => tokens.push(Token::RedirectBoth),
            "&>>" => tokens.push(Token::AppendBoth),
            ";" => tokens.push(Token::Semicolon),
            "(" => tokens.push(Token::LeftParen),
            ")" => tokens.push(Token::RightParen),
            _ => {
                tokens.push(Token::Word(token.clone()));
            }
        }
    }

    tokens
}
