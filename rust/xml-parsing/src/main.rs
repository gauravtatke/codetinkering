#![allow(dead_code, unused_imports, unused_variables)]

// extern crate yaserde;
extern crate xml;

mod fix_xml;
mod fix;
mod macros;
// mod strongxmltest;

use fix_xml::*;
// use strongxmltest::*;

pub (crate) const FILE_PATH: &str = "config/fix43/FIX43.xml";

fn main() {
    println!("{:#?}", get_fix(FILE_PATH)); 
}

