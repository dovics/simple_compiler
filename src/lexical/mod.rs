pub mod token;
pub mod util;
pub struct Lexer {
    list:Vec<token::Token>,
    current_line:usize,
    errors:Vec<LexErr>,
    code_index: usize,
}

pub struct LexErr {
    location:usize,
    which:String,
    err:String,
}

impl Lexer {
    pub fn new() -> Self {
        let mut l = Self {
            list: Vec::new(),
            current_line:0,
            errors: Vec::new(),
            code_index: 0,
        };
        l.add_line(token::Token::new(-1, "#"));
        return l;
    }
    
    pub fn analyse(&mut self, s:String) {
        let code:Vec<&str> = s.split_whitespace().collect();
        for word in code {
            let (index, length, ok) = util::contains_token(word);

            if ok {
                let before_sep = &word[0..index];
                let sep = &word[index..index+length];
                let after_seq = &word[index+length..word.len()];
                self.handle_word(String::from(before_sep));
                self.handle_word(String::from(sep));
                self.handle_word(String::from(after_seq));
            } else {
                self.handle_word(String::from(word));
            }
        }
        self.add_line(token::Token::new(-1, "#"));

        for err in &self.errors {
            println!("local: {:?}, word: {:?} -> err: {:?}", err.location, err.which, err.err)
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for token in self.list.clone()  {
            println!("type: {:?}    value: {:?}", token.tp, token.value)
        }
    }
    
    pub fn next_code(&mut self) -> token::Token {
        if !self.has_next() {
            return token::Token::new(0, "");
        }
        self.code_index = self.code_index + 1;
        self.list[self.code_index - 1].clone()
    }

    pub fn has_next(&self) -> bool {
        !(self.code_index == self.list.len())
    }
    
    fn handle_word(&mut self, s:String) {
        for c in s.chars(){
            if c.is_alphabetic() {
                let (is_key, token_type) = util::is_key(&s);
                if is_key {
                    self.add_line(token::Token::new(token_type, s.as_str()));
                    break;
                }

                let is_id = util::is_valid_identify(&s);
                if is_id {
                    self.add_line(token::Token::new(18, s.as_str()));
                    break;
                }
            }

            if c.is_numeric() {
                let is_integer = util::is_valid_number(&s);
                if is_integer {
                    self.add_line(token::Token::new(19, s.as_str()));
                    break;
                } else {
                    self.add_line(token::Token::new(20, s.as_str()));
                    break;
                }

            }

            let (is_token, token_type) = util::is_token(&s);
            if is_token {
                self.add_line(token::Token::new(token_type, s.as_str()));
                break;
            }

            self.record_err(s,"unknown type of word");
            break;
        }
        
    }

    fn add_line(&mut self, t:token::Token) {
        self.list.push(t)
    }

    fn record_err(&mut self, word:String, error:&str) {
        let err = LexErr {
            location: self.current_line,
            which: word,
            err: String::from(error),
        };
        self.errors.push(err);
    }
    
    pub fn sprint(&self) -> String{
        let mut result = String::new();
        let mut i = self.code_index; 
        while i < self.list.len() - 1 {
            result.push_str(&self.list[i].value);
            result.push_str(" ");
            i = i + 1;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(c:&str) {
        let code = String::from(c);
        let mut l = Lexer::new();
        l.analyse(code);
        l.print()
    }
    #[test]
    fn test_lex() {
        test("program
        var a: integer;
        var b: integer;
		begin
		while a < b do
		if b > 0 then b := b - a else b := a + 1
        end");
        
        println!();
        test("program
        while a < b do
        if c < d then x := y + z
        end");
    }
}