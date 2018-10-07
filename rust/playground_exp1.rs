use std::collections::HashMap;

pub trait Nothing {
    type Output;
    
    fn get_nothing(&self) -> Self::Output;
}

struct integer (u32);
struct float (f64);
struct stringy (String);

impl Nothing for integer {
    type Output = u32;
    
    fn get_nothing(&self) -> u32 {
        self.0
    }
}

impl Nothing for float {
    type Output = f64;
    
    fn get_nothing(&self) -> f64 {
        self.0
    }
}

impl Nothing for stringy {
    type Output = String;
    
    fn get_nothing(&self) -> String {
        self.0
    }
}

pub struct Field {
    pub tag: u32,
    pub val: Box<Nothing>,
}


pub struct FieldMap {
    pub fields: HashMap<u32, Field>
}

impl FieldMap {
    pub fn set(&mut self, tag: u32, field: Field) {
        self.fields.insert(tag, field);
    }
    
    pub fn new() -> Self {
        FieldMap {
            fields: HashMap::new()  
        }
    }
}

#[allow(unused)]
fn main() {
    let field1 = Field {
        tag: 35,
        val: Box::new(integer(0))
    };
    
    let field2 = Field {
        tag: 40,
        val: Box::new(float(32.6))
    };
    
    let field3 = Field {
        tag: 12,
        val: Box::new(stringy("nothing is great".to_string()))
    };
    
    let mut fieldmap = FieldMap::new();
    fieldmap.set(field1.tag, field1);
    fieldmap.set(field2.tag, field2);
    fieldmap.set(field3.tag, field3);
    
    for (key, value) in fieldmap.fields.iter() {
        println!("key = {}, field tag = {}, field value = {}", key, value.tag, value.val);
    }
    println!("################");
    for (key, value) in fieldmap.fields.iter() {
        println!("key = {}, field tag = {}, field value = {}", key, value.tag, value.val);
    }
}