use crate::lexer::Token;

pub fn is_valid_json(tokens: &Vec<Token>) -> bool {
    if tokens.len() < 2 {
        return false;
    }

    validate(tokens.as_slice())
}

fn validate(token_slice: &[Token]) -> bool {
    if token_slice.len() == 0 {
        return true;
    }

    let result = match token_slice[0] {
        Token::ObjOpen => validate_obj(token_slice),
        Token::ArrOpen => valid_arr(token_slice),
        _ => Err(()),
    };

    result.is_ok()
}

fn validate_obj(token_slice: &[Token]) -> Result<&[Token], ()> {
    match token_slice {
        [Token::ObjOpen, Token::ObjClose, rest @ ..] => Ok(rest),
        [Token::ObjOpen, rest @ ..] => {
            let mut current_kv_slice = rest;
            loop {
                match valid_kv_pair(current_kv_slice) {
                    Ok([Token::Comma, rest @ ..]) => {
                        current_kv_slice = rest;
                        continue;
                    }
                    Ok([Token::ObjClose, rest @ ..]) => return Ok(rest),
                    _ => return Err(()),
                }
            }
        }
        _ => Err(()),
    }
}

fn valid_kv_pair(token_slice: &[Token]) -> Result<&[Token], ()> {
    match token_slice {
        [Token::Str(_), Token::Colon, val_slice @ ..] => match val_slice {
            [val_token, rest @ ..] if val_token.is_primitive() => Ok(rest),
            [Token::ObjOpen, ..] => validate_obj(val_slice),
            [Token::ArrOpen, ..] => valid_arr(val_slice),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn valid_arr(token_slice: &[Token]) -> Result<&[Token], ()> {
    match token_slice {
        [Token::ArrOpen, Token::ArrClose, rest @ ..] => Ok(rest),
        [Token::ArrOpen, rest @ ..] => {
            let mut current_val_slice = rest;
            loop {
                match valid_val(current_val_slice) {
                    Ok([Token::Comma, rest @ ..]) => {
                        current_val_slice = rest;
                        continue;
                    }
                    Ok([Token::ArrClose, rest @ ..]) => return Ok(rest),
                    _ => return Err(()),
                }
            }
        }
        _ => Err(()),
    }
}

fn valid_val(token_slice: &[Token]) -> Result<&[Token], ()> {
    match token_slice {
        [Token::ObjOpen, ..] => validate_obj(token_slice),
        [Token::ArrOpen, ..] => valid_arr(token_slice),
        [token, rest @ ..] if token.is_primitive() => Ok(rest),
        _ => Err(()),
    }
}
