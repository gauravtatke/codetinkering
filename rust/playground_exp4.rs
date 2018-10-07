use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Value {
    Int(u32),
    Float(f64),
    Str(String),
    Bool(bool)
}

#[derive(Debug)]
struct Field {
    tag: u32,
    value: Value
}

#[derive(Debug)]
enum FixError {
    EmptyValue,
    InvalidTag
}

impl FromStr for Field {
    type Err=FixError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('=') {
            Some(idx) => {
                let tag: u32 = match s[..idx].parse(){
                    Ok(t) => t,
                    Err(e) => 0
                };
                let raw_value = &s[idx+1..];
                let value = raw_value.parse()?;
                Ok(Field {tag, value})
            },
            None => Err(FixError::EmptyValue)
        }
    }
}

impl FromStr for Value {
    type Err=FixError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<u32>() {
            Ok(Value::Int(i))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Value::Float(f))
        } else if let Ok(b) = s.parse::<bool>() {
            Ok(Value::Bool(b))
        } else {
            Ok(Value::Str(s.to_string()))
        }
    }
}


struct Message {
    fields: HashMap<u32, Field>,
    // groups: Vec<HashMap<u32, Field>>
}


impl Message {
    fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }
    
    fn set_field(&mut self, tagnum: u32, fld: Field) {
        self.fields.insert(tagnum, fld);
    }
    
    fn get_field(&self, tagnum: u32) -> Option<&Field> {
        self.fields.get(&tagnum)
    }
}

#[allow(unused_variables)]
fn main() {
    let price = Field {
        tag: 44,
        value: Value::Float(1.11895)
    };
    
    let ccypair = Field {
        tag: 64,
        value: Value::Str(String::from("EUR/USD"))
    };
    
    let amt = Field {
        tag: 32,
        value: Value::Int(1000000)
    };
    
    let mut message = Message::new();
    message.set_field(44, price);
    message.set_field(64, ccypair);
    message.set_field(32, amt);
    
    println!("price is {:?}", message.get_field(44));
    println!("price is {:?}", message.get_field(44));
    let f: Field = "40=D".parse().unwrap();
    println!("New field = {:?}", f);
    println!("New field = {:?}", f);
    
}