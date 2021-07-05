use std::collections::HashMap; 

#[derive(Clone)]
pub struct Token {
    pub tp: TokenType,
    pub value: String,
    pub symbol: String,
    attr: HashMap<String, String>,
}

pub type TokenType = i32;

const SYMBOL_ID:&str = "id";
const SYMBOL_I:&str = "i";

impl Token {
    pub fn new(tp:TokenType, v:&str) -> Self {
        let symbol:String;
        if tp == 18 {
            symbol = String::from(SYMBOL_ID);
        } else if tp == 19 || tp == 20 {
            symbol = String::from(SYMBOL_I);
        } else {
            symbol = String::from(v);
        }

        Self{
            tp: tp,
            value: String::from(v), 
            symbol: symbol,
            attr: HashMap::new(),
        }
    }
    pub fn add_attr(&mut self, key:&str , value:String) {
        self.attr.insert(String::from(key), value);
    }
    
    pub fn get_attr(&self, key:&str) -> String {
        if let Some(result) = self.attr.get(&String::from(key)) {
            return result.clone();
        }
        return String::from("");
    }

    // pub fn has_attr(self, key:&String) -> bool {
    //     let v = self.attr.get(key);
    //     match v {
    //         Some(_) => return true,
    //         None => return false,
    //     }
    // }
}