use crate::grammer::priority;
use crate::lexical;
use crate::action;
mod stack;

pub struct Parser {
    pt: priority::PriorityTable,
    codes: stack::Stack,
    progress: Vec<String>,

    pub actions:action::ActionMap
}

impl Parser {
    pub fn new(pt: priority::PriorityTable) -> Self{ 
        Self{
            pt: pt,
            codes: stack::Stack::new(),
            progress: Vec::new(),
            actions:action::ActionMap::new(),
        }
    }

    pub fn analyse(&mut self, mut l:lexical::Lexer) {
        let mut next:lexical::token::Token;
        let mut action:Action;

        self.codes.push(l.next_code());
        while l.has_next() {
            next = l.next_code();
            let prev = self.codes.top();
            self.codes.push(next.clone());
            
            // println!("{:?} compare: {:?} {:?}", self.codes.sprint(), &prev.symbol, &next.symbol);
            match self.pt.compare(&prev.symbol, &next.symbol) {
                priority::Priority::Greatter => {
                    action = Action::Reduction;
                    self.reduction()
                }
                priority::Priority::Lower => {
                    action = Action::Lower;
                }
                priority::Priority::Equal => {
                    continue;
                }
                priority::Priority::Undifined => {
                    action = Action::Finish;
                }
            }
            self.progress.push(format!("{:?} {:?} {:?}", self.codes.sprint(), l.sprint(), action));
            if action == Action::Finish {
                break;
            }
        }

        self.actions.programmer.quolist.print();
    }

    fn reduction(&mut self) {
        while self.should_reduction() {
            let (codes, start, end) = self.code_to_reduction();
            self.progress.push(format!("{} {} {}", self.codes.sprint(), "", "reduction"));

            let mut lines:Vec<String> = Vec::new();
            for c in codes.clone() {
                lines.push(c.value);
            }

            let code = lines.join(" ");
            let code = code.trim_start();
            
            let mut starter = lexical::token::Token::new(100, "P");
            // println!("replace: {} ,{}, {}", self.codes.sprint() ,start, end);
            self.actions.emit_func(code, &mut starter,codes);
            self.codes.replace(start, end, starter);
        }
    }

    fn should_reduction(&mut self) -> bool {
        let (prev, _) = self.codes.heading_vt(1);
        let (next, _ ) = self.codes.heading_vt(0);

        if self.pt.compare(&prev.symbol, &next.symbol) == priority::Priority::Greatter {
            return true;
        }

        return false;
    }

    fn code_to_reduction(&mut self) -> (Vec<lexical::token::Token>, usize, usize){
        let start = self.find_start_index();     
        let end = self.find_end_index();

        if start == end {
            return (self.codes.stack[start..end+1].to_vec(), start, end)
        }

        return (self.codes.stack[start..end].to_vec(), start, end)
    }

    pub fn find_start_index(&mut self) -> usize{
        let mut count = 0;
        let mut i:isize = self.codes.stack.len() as isize - 1;
        let mut index:usize = 0;
        while i >= 0 {
            let ( next, _ ) = self.codes.heading_vt(count);
            let ( prev, prev_index ) = self.codes.heading_vt(count + 1);

            if self.pt.compare(&prev.symbol, &next.symbol) == priority::Priority::Lower {
                index = prev_index + 1;
                break;
            }
            count = count + 1;
            i = i - 1;
        }

        return index;
    }

    pub fn find_end_index(&mut self) -> usize {
        let mut count = 0;
        let mut i:isize = self.codes.stack.len() as isize - 2;
        let mut index:usize = 0;

        while i >= 0 {
            let (next, next_index) = self.codes.heading_vt(count);
            let (prev, _) = self.codes.heading_vt(count + 1);

            if self.pt.compare(&prev.symbol, &next.symbol) == priority::Priority::Greatter{
                index = next_index;
                break;
            }
            count = count + 1;
            i = i - 1;
        }
        return index;
    }
    
    #[allow(dead_code)]
    pub fn print(&self) {
        for s in self.progress.clone() {
            println!("{}", s);
        }
    }
}

#[derive(PartialEq, Debug)]
enum Action {
    Reduction,
    // InputNext,
    Lower,
    Finish,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammer::Grammer;

    fn test(code:&str) {
        let mut l = lexical::Lexer::new();
        let code_str = String::from(code);
        l.analyse(code_str);
        let gram = "X -> # P #
                    P -> program L
                    L -> S | id , L | id : K | var L ; G
                    K -> integer | bool | real
                    G -> begin S end
                    S -> id := E | if B then S else S | while B do S
                    B -> id < I | id > I
                    E -> id + I | id - I
                    I -> i | id | ( E ) | E
                    ";
        let mut gram = Grammer::new(gram);
        gram.init();
        let pt = gram.gen_priority_table();
        let mut p = Parser::new(pt);
        p.analyse(l);
        p.print();
    }

    #[test]
    fn test_parse() {
        test("program
        var a,b : integer;
        begin
        while a < b do
        if b > 0 then b := b - a else b := a + 1
        end");
        println!();
        test("program
        while a < b do
        if c < d then x := y + z
        end")
    }
}