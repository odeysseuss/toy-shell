pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = input.chars().peekable();

    #[derive(PartialEq)]
    enum State {
        Unquoted,
        SingleQuoted,
        DoubleQuoted,
    }

    let mut state = State::Unquoted;

    while let Some(ch) = chars.next() {
        match state {
            State::Unquoted => match ch {
                '\'' => {
                    state = State::SingleQuoted;
                }
                '"' => {
                    state = State::DoubleQuoted;
                }
                '\\' => {
                    if let Some(next_ch) = chars.next() {
                        current_token.push(next_ch);
                    }
                }
                ' ' | '\t' | '\n' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                '>' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(">".to_string());
                }
                '1' => {
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == '>' {
                            if !current_token.is_empty() {
                                tokens.push(current_token.clone());
                                current_token.clear();
                            }
                            chars.next();
                            tokens.push("1>".to_string());
                        } else {
                            current_token.push(ch);
                        }
                    } else {
                        current_token.push(ch);
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            },
            State::SingleQuoted => match ch {
                '\'' => {
                    state = State::Unquoted;
                }
                _ => {
                    current_token.push(ch);
                }
            },
            State::DoubleQuoted => match ch {
                '"' => {
                    state = State::Unquoted;
                }
                '\\' => {
                    if let Some(&next_ch) = chars.peek() {
                        if matches!(next_ch, '\\' | '"' | '$' | '`' | '\n') {
                            chars.next();
                            if next_ch != '\n' {
                                current_token.push(next_ch);
                            }
                        } else {
                            current_token.push(ch);
                        }
                    } else {
                        current_token.push(ch);
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            },
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    return tokens;
}
