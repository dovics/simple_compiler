pub mod priority;

use std::collections::{HashSet, HashMap};

pub struct Grammer {
    gram:String,
    regulate: Vec<String>,

    vt:HashSet<String>,
    vn:HashSet<String>,

    first_vt:HashMap<String, HashSet<String>>,
    last_vt:HashMap<String, HashSet<String>>,

    first_vn_flag:HashMap<String, bool>,
    last_vn_flag:HashMap<String, bool>,
}

impl Grammer {
    pub fn new(grammer:&str) -> Self { 
        Self{ 
            gram: String::from(grammer),
            regulate: regulate_generations(String::from(grammer)),

            vt: HashSet::new(),
            vn: HashSet::new(),
            
            first_vt: HashMap::new(),
            last_vt: HashMap::new(),

            first_vn_flag: HashMap::new(),
            last_vn_flag: HashMap::new(),
        }
    } 

    pub fn init(&mut self) {
        self.init_vt_vn();
        self.init_firstvt_and_lastvt();
    }

    fn init_vt_vn(&mut self){
        let g1 = self.gram.replace("->", " ");
        let g2 = g1.replace("|", " ");
        for word in g2.split_whitespace() {
            if word.len() == 1 && is_upper_letter(word){
                self.vn.insert(String::from(word));
            } else {
                self.vt.insert(String::from(word));
            }
        }
    }

    fn init_firstvt_and_lastvt(&mut self) {
        for word in self.vn.iter() {
            self.first_vt.insert(word.clone(), HashSet::new());
            self.last_vt.insert(word.clone(), HashSet::new());
    
            self.first_vn_flag.insert(word.clone(), false);
            self.last_vn_flag.insert(word.clone(), false);
        }
        
        for word in self.vn.clone() {
            for word in self.vn.iter() {
                self.first_vn_flag.insert(word.clone(), false);
            }
            
            self.find_first_vt(word.clone(), word.clone());
            
            for word in self.vn.iter() {
                self.last_vn_flag.insert(word.clone(), false);
            }
            
            self.find_last_vt(word.clone(), word.clone());
        }
    }

    fn find_first_vt(&mut self, start:String, vn:String) {
        if self.first_vn_flag[&start] {
            return
        }

        self.first_vn_flag.insert(start.clone(), true);
        let regulate = self.regulate.clone();
        let lines = lines_start_with(regulate, start.clone());

        for line in lines {
            let mut words:Vec<&str> = line.split_whitespace().collect();
            words.remove(0);

            let first_one = String::from(words[0]);

            if self.vn.contains(&first_one) {
                if words.len() > 1 {
                    let first_two = String::from(words[1]);
                    if  self.vt.contains(&first_two) {
                        if let Some(set) = self.first_vt.get_mut(&vn) {
                            set.insert(first_two);
                        }
                    }
                }

                self.find_first_vt(first_one.clone(), vn.clone());
            }

            if self.vt.contains(&String::from(words[0])) {
                if let Some(set) = self.first_vt.get_mut(&vn) {
                    set.insert(first_one.clone());
                }
            }
        }
    }
    
    fn find_last_vt(&mut self, start:String, vn:String) {
        if self.last_vn_flag[&start] {
            return
        }

        self.last_vn_flag.insert(start.clone(), true);
        let regulate = self.regulate.clone();
        let lines = lines_start_with(regulate, start.clone());

        for line in lines {
            let words:Vec<&str> = line.split_whitespace().collect();
            let last_one = String::from(words[words.len() - 1]);

            if self.vt.contains(&last_one) {
                if let Some(set) = self.last_vt.get_mut(&vn) {
                    set.insert(last_one.clone());
                }
            }

            if self.vn.contains(&last_one) {
                if words.len() > 1 {
                    let last_second = String::from(words[words.len() - 2]);
                    if self.vt.contains(&last_second) {
                        if let Some(set) = self.last_vt.get_mut(&vn) {
                            set.insert(last_second);
                        }
                    }
                }

                self.find_last_vt(last_one, vn.clone());
            }
        }
    }

    pub fn gen_priority_table(&mut self) -> priority::PriorityTable{
        let mut pt = priority::PriorityTable::new();
        pt.init(self.vt.clone());
        for line in self.regulate.clone() {
            let mut words:Vec<&str> = line.split_whitespace().collect();
            words.remove(0);
            let l = words.len()-1;
            for i in 0..l {
                if self.vt.contains(words[i]) && self.vn.contains(words[i+1]) {
                    pt.add_lower_priority(String::from(words[i]), self.first_vt[words[i+1]].clone());
                }

                if self.vn.contains(words[i]) && self.vt.contains(words[i+1]) {
                    pt.add_greatter_priority(String::from(words[i+1]), self.last_vt[words[i]].clone())
                }

                if self.vt.contains(words[i]) && self.vt.contains(words[i+1]) {
                    pt.add_priority(String::from(words[i]), String::from(words[i+1]), priority::Priority::Equal);
                }
            }
            if l <= 0 {
                continue
            }
            for i in 0..l-1 {
                if self.vt.contains(words[i]) &&self.vn.contains(words[i+1]) && self.vt.contains(words[i+2]) {
                    pt.add_priority(String::from(words[i]), String::from(words[i+2]), priority::Priority::Equal)
                }
            }
        }

        return pt;
    }
}

fn regulate_generations(expr: String) -> Vec<String>{
    let e = expr.replace("->", " ");
    let mut result = Vec::new();
    for line in e.split("\n"){
        let l = String::from(line.trim());
        if l.len() == 0 {
            continue;
        }
        if let Some(start) = l.get(0..1) {
            for new_line in l[1..].trim().split("|") {
                if l.len() == 0 {
                    continue
                }  

                let mut nl = String::new();
                nl.push_str(start);
                nl.push_str(" ");
                nl.push_str(new_line);
                result.push(nl);
            }
        }
    }

    return result;
}

fn lines_start_with(lines:Vec<String>, c:String) -> Vec<String>{
    let mut result = Vec::new();
    for line in lines {
        if line.starts_with(c.as_str()) {
            result.push(line);
        }
    }

    return result;
}

fn is_upper_letter(s: &str) -> bool{
    for c in s.chars() {
        if c as u32 >= 0x41 && c as u32 <= 0x5a {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    const GRAM:&str= "X -> # P #
                P -> program L
                L -> S | id , L | id : K | var L ; G
                K -> integer | bool | real
                G -> begin S end
                S -> id := E | if B then S else S | while B do S
                B -> id < I | id > I
                E -> id + I | id - I
                I -> i | id | ( E ) | E";
    #[test]
    fn test_regulate_generations() {
        let result = regulate_generations(String::from(GRAM));
        println!("{:?}", result);
    } 
    #[test]
    fn test_init_vt_vn() {
        let mut gram = Grammer::new(GRAM);
        gram.init_vt_vn();
        println!("vt: {:?}", gram.vt);
        println!("vn: {:?}", gram.vn);
    }

    #[test]
    fn test_init_firstvt_and_lastvt() {
        let mut gram = Grammer::new(GRAM);
        gram.init_vt_vn();
        gram.init_firstvt_and_lastvt();
        println!("firstVT: {:?}", gram.first_vt);
        println!("lastVT: {:?}", gram.last_vt);
    }

    #[test]
    fn test_gen_priority_table() {
        let mut gram = Grammer::new(GRAM);
        gram.init();
        let pt = gram.gen_priority_table();
        pt.print();
    }
}

