mod lexical;
mod grammer;
mod parser;
mod action;

fn main() {
    let mut l = lexical::Lexer::new();
    let code = String::from("program
                            while a < b do
                            if c < d then x := y + z
                            end");
    l.analyse(code);
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
    let mut gram = grammer::Grammer::new(gram);
    gram.init();
    let pt = gram.gen_priority_table();
    let mut p = parser::Parser::new(pt);
    p.analyse(l);
}
