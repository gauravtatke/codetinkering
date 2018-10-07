use std::collections::HashMap;

type Tagnum = u32;
type Amount = u64;
type Currency = String;
type Price = f64;
type Char = char;
type Bool = bool;


trait FixField {
    fn get_tag(&self) -> u32;
    fn get_value(&self) -> &String;
    fn get_value_mut(&mut self) -> &mut String;
}

pub struct Field<T> {
    pub tag: Tagnum,
    pub value: T,
    pub string_val: String,
}

impl Field<u32> {
    fn new(tag: u32, val: u32) -> Self {
        Self {
            tag: tag,
            value: val,
            string_val: stringify!(tag=value).to_string()
        }
    }
}

impl FixField for Field<u32> {
    fn get_tag(&self) -> u32 {
        self.tag
    }

    fn get_value(&self) -> &String {
        let temp = self.value;
        &stringify!(temp).to_string()
    }

    fn get_value_mut(&mut self) -> &mut String {
        &mut self.string_val
    }
}

impl Field<f64> {
    fn new(tag: u32, val: f64) -> Self {
        Self {
            tag: tag,
            value: val,
            string_val: stringify!(tag=value).to_string()
        }
    }
}

impl FixField for Field<f64> {
    fn get_tag(&self) -> u32 {
        self.tag
    }

    fn get_value(&self) -> &String {
        &self.string_val
    }

    fn get_value_mut(&mut self) -> &mut String {
        &mut self.string_val
    }
}

impl Field<String> {
    fn new(tag: u32, val: String) -> Self {
        Self {
            tag: tag,
            value: val,
            string_val: stringify!(tag=value).to_string()
        }
    }
}

impl FixField for Field<String> {
    fn get_tag(&self) -> u32 {
        self.tag
    }

    fn get_value(&self) -> &String {
        &self.string_val
    }

    fn get_value_mut(&mut self) -> &mut String {
        &mut self.string_val
    }
}

struct FieldMap<T> {
    fields: HashMap<Tagnum, Field<T>>
}


#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");
}
