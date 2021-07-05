use crate::lexical::token::Token;
pub struct Stack {
    pub stack: Vec<Token>,
    index:usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            index:0,
        }
    }

    pub fn push(&mut self, t:Token) {
        self.index = self.index + 1;
        self.stack.push(t)
    }

    // pub fn pop(&mut self) -> Token {
    //     let res = self.stack[self.stack.len() - 1].clone();
    //     self.stack.remove(self.stack.len() - 1);
    //     self.index = self.index - 1;
    //     return res;
    // }

    pub fn top(&self) -> Token {
        self.stack[self.stack.len() - 1].clone()
    }

    pub fn replace(&mut self,start:usize, end:usize, token: Token) {
        if start == end {
            self.stack[start] = token.clone();
        }
        self.stack[start] = token.clone();
        let mut i = 1;
        while i < end - start {
            self.stack.remove(start + 1);
            i = i + 1;
        }
    }

    pub fn heading_vt(&self, n:usize) -> (Token, usize){
        let mut code = self.top();
        let mut index:usize = 0;
        let mut count = 0;
        let mut i = self.stack.len() - 1;
        while  count <= n {
            if self.stack[i].tp != 100 {
                code = self.stack[i].clone();
                index = i;
                count += 1;
            }

            if i == 0 {
                break;
            }

            i = i - 1;
        }

        return (code, index);
    }

    // pub fn button_vt(&self, n:usize) -> (Token, usize) {
    //     let mut code = self.top();
    //     let mut count = 0;
    //     let mut index:usize = 0;
    //     let mut i = 0;
    //     while count <= n {
    //         if self.stack[i].tp != 100 {
    //             code = self.stack[i].clone();
    //             index = i;
    //             count += 1;
    //         }
    //         i = i + 1;
    //     }

    //     return (code, index);
    // }

    pub fn sprint(&self) -> String{
        let mut result = String::new();
        for t in self.stack.clone() {
            result.push_str(&t.symbol);
            result.push_str(" ");
        }

        return result;
    }
}