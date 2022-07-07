extern crate handlebars;
extern crate serde;
extern crate yaserde;

mod code_generator;
mod templates;

use crate::code_generator::get_fix_spec;
use code_generator::*;
use std::env;
use std::path::PathBuf;
use templates::*;

pub fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("resources");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:warning={:?}", &out);
    let fix = get_fix_spec(&source, "FIX43.xml");
    generate_fields(&out, "fields.rs", &fix);
}
