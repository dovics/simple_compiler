pub struct Quotation {
    id:usize,
    op:String,
    a:String,
    b:String,
    addr:String,
}

impl Quotation {
    fn new(id:usize, op:String, a:String, b:String, addr:String) -> Self {
        Self{ 
            id, op, a, b, addr
        }
    }
}

pub struct QuotationList {
    list:Vec<Quotation>,
    pub current_id:usize,
}

impl QuotationList {
    pub fn new() -> Self{
        Self{
            list: Vec::new(),
            current_id:100,
        }
    }

    pub fn add(&mut self,op:&str, a:String, b:String, addr:String) {
        self.list.push(Quotation::new(self.current_id, String::from(op), a, b, addr));
        self.current_id = self.current_id + 1;
    }

    pub fn print(&self) {
        for q in self.list.iter() {
            println!("{} ({}, {}, {}, {})", q.id, q.op, q.a, q.b, q.addr)
        }
    }

    pub fn string_id(&mut self, off:isize) -> String {
        (self.current_id as isize + off).to_string()
    }

    pub fn set_addr(&mut self, tar:usize, val:usize) {
        self.list[tar-100].addr = val.to_string();
    }
}