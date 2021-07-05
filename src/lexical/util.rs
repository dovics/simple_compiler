use super::token;
const TOKENSTR:&str = "( ) + - * / . , := : ; <= <> < >= > =";
const KEYSTR:&str = "and begin bool do else end false if integer not or program real then true var while 标志符 整数 实数";

pub fn contains_token(s: &str) -> (usize, usize, bool) {
    let tokens:Vec<&str> = TOKENSTR.split_whitespace().collect();
    for token in tokens {
        if s.contains(token) {
            if let Some(index) = s.find(token) {
                return (index, token.len(), true);
            };
        }
    }
    (0, 0, false)
}

pub fn is_token(s:&String) -> (bool, token::TokenType) {
    let tokens:Vec<&str> = TOKENSTR.split_whitespace().collect();
    let mut count = 0;
    for token in tokens {
        if s.eq(token){
            return (true, count + 1);
        }
        count = count + 1;
    }
    (false,0)
}

pub fn is_key(s:&String) -> (bool, token::TokenType) {
    let tokens:Vec<&str> = KEYSTR.split_whitespace().collect();
    let mut count = 0;
    for token in tokens {
        if s.eq(token){
            return (true, count + 21);
        }
        count = count + 1;
    }
    (false,0)
}

pub fn is_valid_identify(s:&String) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    
    true
}

pub fn is_valid_number(s:&String) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}