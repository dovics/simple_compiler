use std::collections::HashMap;
use crate::lexical::{token, util};
use token::Token;

mod quotation;
const EMPTY:&str = "_";

type ActionFunc = dyn Fn(&mut Programmer, &mut Token, Vec<Token>);

pub struct ActionMap where{
    actions:HashMap<String, Box<ActionFunc>>,
    pub programmer: Programmer,
}

pub struct Programmer {
    pub quolist:quotation::QuotationList,
    variations:HashMap<String, String>
}

impl ActionMap {
    pub fn new() ->Self {
        let mut am = Self {
            actions: HashMap::new(),
            programmer: Programmer::new(),
        };
        am.actions.insert(String::from(":="), Box::new(assign));
        am.actions.insert(String::from("+"), Box::new(plus));
        am.actions.insert(String::from("-"), Box::new(dec));
        am.actions.insert(String::from("*"), Box::new(multiply));
        am.actions.insert(String::from("/"), Box::new(div));
        am.actions.insert(String::from("if then"), Box::new(if_then));
        am.actions.insert(String::from("if then else"), Box::new(if_then_else));
        am.actions.insert(String::from("while do"), Box::new(while_do));
        am.actions.insert(String::from("if then end"), Box::new(if_then));

        am.actions.insert(String::from(">"), Box::new(great_op));
        am.actions.insert(String::from(">="), Box::new(great_equal_op));
        am.actions.insert(String::from("<"), Box::new(low_op));
        am.actions.insert(String::from("<="), Box::new(low_equal_op));
        am.actions.insert(String::from("="), Box::new(equal_op));

        am.actions.insert(String::from("id"), Box::new(id_func));
        am.actions.insert(String::from("i"), Box::new(i_func));
        am.actions.insert(String::from(": integer"), Box::new(declare));
        am.actions.insert(String::from(": bool"), Box::new(declare));
        am.actions.insert(String::from(": real"), Box::new(declare));
        am.actions.insert(String::from(","), Box::new(declare));

        return am;
    }

    pub fn emit_func(&mut self, code:&str,t: &mut Token, code_slice: Vec<Token>){
        if let Some(emit_func) = self.actions.get(filter_token(code).trim()) {
            emit_func(&mut self.programmer, t, code_slice);
            return
        }

        if let Some(emit_func) = self.actions.get(&"id".to_string()) {
            emit_func(&mut self.programmer, t, code_slice);
            return
        }

        if let Some(emit_func) = self.actions.get(&"i".to_string()) {
            emit_func(&mut self.programmer, t, code_slice);
            return
        }
        
        println!("{} don't have emit func", code);
    }
}

fn filter_token(code:&str) -> String {
    let slice:Vec<&str> = code.split_whitespace().collect();
    let mut result = String::new();
    for s in slice {
        let (ok1, _) = util::is_token(&s.to_string());
        let (ok2, _) = util::is_key(&s.to_string());
        if ok1 || ok2 {
            result.push_str(s);
            result.push_str(" ");
        }
    }

    return result;
}

impl Programmer {
    fn new() -> Self{
        Self {
            quolist: quotation::QuotationList::new(),
            variations: HashMap::new(),
        }
    }
    pub fn contain_variation(&self, s:String) -> bool {
        self.variations.contains_key(&s)
    }

    pub fn add_variation(&mut self, s:String) { 
        self.variations.insert(s,String::new());
    }

    fn back_patch(&mut self, t:String, v:String) {
        let str_slice:Vec<&str> = t.split_whitespace().collect();

        for s in str_slice {
            let tar:usize;
            match s.parse::<usize>() {
                Ok(tar_v) => tar = tar_v,
                Err(_err) => return,
            }

            let val:usize;
            match v.parse::<usize>() {
                Ok(val_v) => val = val_v,
                Err(_err) => return,
            }

            if tar > self.quolist.current_id {
                return
            }

            self.quolist.set_addr(tar,val);
        }
    }
}

fn declare(p:&mut Programmer, t:&mut Token, code_slice: Vec<Token>) { 
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();
    
    let src_value = src.get_attr("value");
    t.add_attr("value", src_value.clone());
    t.tp = src.tp;

    if p.contain_variation(src.value.clone()) {
        panic!("error");
    }

    p.add_variation(tar.value.clone());
}

fn assign(p:&mut Programmer, t: &mut Token, code_slice: Vec<Token>) { 
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();
    let src_value = src.get_attr("value");
    let src_quad = src.get_attr("quad");

    t.add_attr("quad", src_quad.clone());
    p.quolist.add(":=", src_value.clone(), String::from(EMPTY), tar.value.clone());
    t.add_attr("value", src_value.clone());
    t.tp = src.tp;
}

fn plus(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();

    let addr = src.value.clone();
    t.add_attr("quad", p.quolist.string_id(0));

    p.quolist.add("+", src.value.clone(), tar.value.clone(), addr.clone());
    t.add_attr("value", addr);
}

fn dec(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();

    let addr = src.value.clone();
    t.add_attr("quad", p.quolist.string_id(0));

    p.quolist.add("-", src.value.clone(), tar.value.clone(), addr.clone());
    t.add_attr("value", addr)
}

fn multiply(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();

    let addr = src.value.clone();
    p.quolist.add("+", src.value.clone(), tar.value.clone(), addr.clone());
    t.add_attr("value", addr)
}

fn div(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    let tar = code_slice[0].clone();
    let src = code_slice[2].clone();

    let addr = src.value.clone();
    p.quolist.add("/", src.value.clone(), tar.value.clone(), addr.clone());
    t.add_attr("value", addr)
}

fn if_then(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    let cond = code_slice[1].clone();
    let do_token = code_slice[3].clone();
    
    let cond_true = cond.get_attr("true");
    let cond_false = cond.get_attr("false");

    let quad = do_token.get_attr("quad");
    let next = do_token.get_attr("next");

    let mut t_next = cond_false.clone();
    t_next.push_str(" ");
    t_next.push_str(&next);

    p.back_patch(cond_true.clone(), quad.clone());

    t.add_attr("quad", p.quolist.string_id(0));
    t.add_attr("next", t_next);

}

fn if_then_else(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    let cond = code_slice[1].clone();
    let do1 = code_slice[3].clone();
    let do2 = code_slice[5].clone();

    let cond_true = cond.get_attr("true");
    let cond_false = cond.get_attr("false");

    let quad1 = do1.get_attr("quad");
    let quad2 = do2.get_attr("quad");
    let next1 = do1.get_attr("next");
    let next2 = do2.get_attr("next");
    let next_n = do1.get_attr("nextN");

    let mut t_next = next1.clone();
    t_next.push_str(" ");
    t_next.push_str(&next2);
    t_next.push_str("_");
    t_next.push_str(&next_n);
    
    p.back_patch(cond_true.clone(), quad1.clone());
    p.back_patch(cond_false.clone(), quad2.clone());

    t.add_attr("quad", p.quolist.string_id(0));
    t.add_attr("next", t_next);
}

fn while_do(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {
    let cond = code_slice[1].clone();
    let do_token = code_slice[3].clone();

    let cond_true = cond.get_attr("true");
    let cond_false = cond.get_attr("false");

    let do_next = do_token.get_attr("next");
    let quad1 = cond.get_attr("quad");
    
    t.add_attr("next", cond_false.clone());
    p.back_patch(do_next.clone(), quad1.clone());
    p.back_patch(cond_true, "102".to_string());
    t.add_attr("quad", p.quolist.string_id(0));
    p.quolist.add("j", String::from(EMPTY), String::from(EMPTY), quad1.clone());    
}

fn great_op(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    compare_emiter(p, ">", t, code_slice);
}

fn great_equal_op(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    compare_emiter(p, ">=", t, code_slice);
}

fn low_op(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    compare_emiter(p, "<", t, code_slice);
}

fn low_equal_op(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    compare_emiter(p, "<=", t, code_slice);
}

fn equal_op(p:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    compare_emiter(p, "=", t, code_slice);
}

fn compare_emiter(p:&mut Programmer,op:&str, t:&mut Token, code_slice:Vec<Token>) {  
    let mut id1 = code_slice[0].clone();
    let id2 = code_slice[2].clone();

    t.add_attr("quad", p.quolist.string_id(0));
    id1.add_attr("value", id1.value.clone());
    t.add_attr("true", p.quolist.string_id(0));
    t.add_attr("false", p.quolist.string_id(1));

    let value1 = id1.get_attr("value");
    let value2 = id2.get_attr("value");

    let mut op_str = String::from("j");
    op_str.push_str(op);
    p.quolist.add(op_str.as_str(), value1.clone(), value2.clone(), String::from("0"));
    p.quolist.add("j", EMPTY.to_string(), EMPTY.to_string(), "0".to_string());
}

fn id_func(_:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) { 
    t.value = code_slice[0].value.clone();
    t.add_attr("value", t.value.clone());
}

fn i_func(_:&mut Programmer, t:&mut Token, code_slice:Vec<Token>) {  
    t.value = code_slice[0].value.clone();
    t.add_attr("value", t.value.clone());
}

#[cfg(test)]
mod tests { 
    use super::*;
    #[test] 
    fn test_emit_func() {
        let am = ActionMap::new();
        if let Some(_) = am.actions.get(&"<".to_string()) {
            println!("find");
            return
        }

        panic!("not find");
    }
}