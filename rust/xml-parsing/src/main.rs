#![allow(dead_code, unused_imports, unused_variables)]

// extern crate yaserde;
extern crate xml;

mod fix_xml;
mod fix;
mod macros;
// mod strongxmltest;

use fix_xml::*;
use fix::*;
// use strongxmltest::*;

pub (crate) const FILE_PATH: &str = "config/fix43/FIX43.xml";

fn main() {
    let mut message = Message::new();
    message.set_field(44, 45.678);
    message.set_field(35, 'A');

    let header = message.header_mut();
    header.set_field(76, "gaurav");
    println!("{:#?}", &message);
    let price: Result<u32, String> = message.get_field(44);
    let found_price: f32 = message.get_field(44).unwrap();
    println!("price = {:?}", found_price);
    
    println!("not parsing {:?}", message.get_field::<f32>(35));
    println!("not found {:?}", message.get_field::<u32>(56));
    message.set_field(34, 1);
}

