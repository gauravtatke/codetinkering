#![allow(dead_code)]
#![allow(unused_imports)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate roxmltree;

mod serial;
mod fields;
mod types;
// mod field;
mod newfield;
mod fxerror;
mod codegen;


// fn to_string<T>(value: &T) -> Result<String, FixError> 
//     where T: Serialize {
//         let mut serializer = FixSerializer { msg: String::new()};
//         value.serialize(&mut serializer)?;
//         Ok(serializer.msg)
//     }

// 8=FIX.4.4|9=126|35=A|49=theBroker.12345|56=CSERVER|34=1|52=20170117- 08:03:04|57=TRADE|50=any_string|98=0|108=30|141=Y|553=12345|554=passw0rd!|10=131| 

fn create_message() {
    // let mut msg = newfield::Message::new();

}

fn main() {
    println!("Hello! World");
    codegen::create_data_dict("/Users/gtatke/myrepos/codetinkering/rust/quickfix-test/src/fix43/FIX43.xml");
}
