pub enum Token {
    ObjOpen,
    ObjClose,
    ArrOpen,
    ArrClose,
    Str(String),
    Colon,
    Comma,
    True,
    False,
    Null,
    Num(f64),
}

impl Token {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Str(_) | Self::Num(_) | Self::Null | Self::True | Self::False
        )
    }
}

pub fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::with_capacity(input.len());

    let mut in_string = false;
    let mut current_string = String::new();
    let mut in_number = false;
    let mut current_number = String::new();
    let mut skip = 0;

    for (i, char) in input.char_indices() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match char {
            '"' => {
                in_string = !in_string;
                if !in_string {
                    tokens.push(Token::Str(current_string.clone()));
                    current_string.clear();
                }
            }
            char if in_string => current_string.push(char),

            '{' => tokens.push(Token::ObjOpen),
            '}' => {
                if in_number {
                    in_number = false;
                    let parsed: f64 = current_number
                        .parse()
                        .expect(format!("couldn't parse {current_number}").as_str());
                    tokens.push(Token::Num(parsed));
                    current_number.clear();
                }
                tokens.push(Token::ObjClose)
            }

            '[' => tokens.push(Token::ArrOpen),
            ']' => {
                if in_number {
                    in_number = false;
                    let parsed: f64 = current_number
                        .parse()
                        .expect(format!("couldn't parse {current_number}").as_str());
                    tokens.push(Token::Num(parsed));
                    current_number.clear();
                }
                tokens.push(Token::ArrClose)
            }

            ':' => tokens.push(Token::Colon),

            ',' => {
                if in_number {
                    in_number = false;
                    let parsed: f64 = current_number
                        .parse()
                        .expect(format!("couldn't parse {current_number}").as_str());
                    tokens.push(Token::Num(parsed));
                    current_number.clear();
                }
                tokens.push(Token::Comma);
            }

            't' => {
                if let "true" = &input[i..i + 4] {
                    tokens.push(Token::True);
                }
                skip = 3;
            }

            'f' => {
                if let "false" = &input[i..i + 5] {
                    tokens.push(Token::False);
                }
                skip = 4;
            }

            'n' => {
                if let "null" = &input[i..i + 4] {
                    tokens.push(Token::Null);
                }
                skip = 3;
            }

            char if char.is_numeric() => {
                if !in_number {
                    in_number = true;
                }
                current_number.push(char);
            }

            char if char.is_whitespace() => {
                if in_number {
                    in_number = false;
                    let parsed: f64 = current_number
                        .parse()
                        .expect(format!("couldn't parse {current_number}").as_str());
                    tokens.push(Token::Num(parsed));
                    current_number.clear();
                }
                continue;
            }

            _ => return Err(String::from("invalid char")),
        }
    }

    Ok(tokens)
}
