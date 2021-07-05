use std::collections::{HashSet, HashMap};

pub struct PriorityTable {
    vt:HashSet<String>,
    pub table:HashMap<String, HashMap<String, Priority>>,
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Priority {
    Greatter,
    Lower,
    Equal,
    Undifined,
}

impl PriorityTable {
    pub fn new() -> Self {
        Self {
            vt: HashSet::new(),
            table: HashMap::new(),
        }
    }

    pub fn init(&mut self, vt:HashSet<String>) {
        self.vt = vt.clone();
        for k1 in vt.clone() {
            let mut map:HashMap<String, Priority> = HashMap::new();
            for k2 in vt.clone() {
                map.insert(k2, Priority::Undifined);
            }
            self.table.insert(k1, map);
        }
    }

    pub fn add_priority(&mut self,line:String, column:String, p:Priority) {
        if self.table[&line][&column] != Priority::Undifined && !(self.table[&line][&column] == p) {
            panic!("dup priority");
        }

        if let Some(map) = self.table.get_mut(&line) {
            map.insert(column, p);
        }
    }

    pub fn add_lower_priority(&mut self, a:String, set:HashSet<String>) {
        for vt in set {
            self.add_priority(a.clone(), vt, Priority::Lower);
        }
    }

    pub fn add_greatter_priority(&mut self, a:String, set:HashSet<String>) {
        for vt in set {
            self.add_priority(vt, a.clone(), Priority::Greatter);
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut list = Vec::new();
        print!("{:>width$} ", "", width = 15);
        for vt in self.vt.clone() {
            print!("{:>width$} ", vt, width = 15);
            list.push(vt)
        }
        println!();
        for k1 in list.clone() { 
            print!("{:>width$}: ", k1, width = 15);
            for k2 in list.clone() {
                let priority = &self.table[&k1][&k2];   
                print!("{:>width$}", format!("{:?}", priority), width = 15); 
            }
            println!()
        }
    }

    pub fn compare(&mut self, prev:&String, next:&String) -> Priority{
        self.table[prev][next].clone()
    }
}