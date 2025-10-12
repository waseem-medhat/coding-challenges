use crate::lexer::Token;

pub fn print(tokens: &[Token]) {
    fn to_text(token: &Token) -> String {
        match token {
            Token::ObjOpen => "{".to_string(),
            Token::ObjClose => "}".to_string(),
            Token::ArrOpen => "[".to_string(),
            Token::ArrClose => "]".to_string(),
            Token::Str(str) => format!("{:?}", str.clone()),
            Token::Colon => ": ".to_string(),
            Token::Comma => ", ".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::Null => "null".to_string(),
            Token::Num(num) => num.to_string(),
        }
    }

    let text = tokens
        .iter()
        .fold(String::new(), |acc, token| acc + &to_text(token));

    println!("{text}");
}
