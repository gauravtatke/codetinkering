use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;
use std::str::FromStr;


#[derive(Clone, Debug)]
struct Int (u32);

impl Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Int {
    fn new(val: u32) -> Self {
        Int(val)
    }    
}


#[derive(Clone, Debug)]
struct Float (f64);

impl Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Float {
    fn new(val: f64) -> Self {
        Float(val)
    }
}

#[derive(Debug)]
struct Str (String);

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


pub trait Field {
    fn get_tag(&self) -> u32;
    fn get_val(&self) -> String;
    fn set_tag(&mut self, tag: u32);
    fn set_val_string(&mut self, val: String);
    fn set_val_str(&mut self, val: &str);
    
    // fn get_val_mut(&mut self) -> String;
}


pub struct IntField {
    pub tag: u32,
    pub val: u32,
}

impl Field for IntField {
    fn get_tag(&self) -> u32 {
        self.tag
    }
    
    fn get_val(&self) -> String {
        self.val.to_string()
    }
    
    fn set_tag(&mut self, tag: u32) {
        self.tag = tag;
    }
    
    fn set_val_string(&mut self, val: String) {
        self.val = u32::from_str(&val).unwrap();
    }
    
    fn set_val_str(&mut self, val: &str) {
        self.val = u32::from_str(val).unwrap();
    }
    // fn get_val_mut(&mut self) -> &mut String {
    //     &mut self.val.to_string()
    // }
}

// pub struct FloatField {
//     tag: u32,
//     val: f64
// }

// pub struct StringField {
//     tag: u32,
//     val: String
// }



#[allow(unused_variables)]
fn main() {
    let mut new_intfield = IntField {
        tag: 35,
        val: 0
    };
    let mut tag = new_intfield.get_tag();
    let mut val = new_intfield.get_val();
    val.push_str("2342");
    println!("tag = {}, val = {}", new_intfield.get_tag(), new_intfield.get_val());
    println!("new tag = {}, new val = {}", tag, val);
    
    new_intfield.set_tag(76);
    new_intfield.set_val_string(String::from("43"));
    
    println!("tag = {}, val = {}", new_intfield.get_tag(), new_intfield.get_val());
    
    new_intfield.set_val_str(&"56");
    
    println!("tag = {}, val = {}", new_intfield.get_tag(), new_intfield.get_val());
    // println!("x = {}", x);
}