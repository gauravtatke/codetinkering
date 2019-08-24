#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod serial;
mod fields;
mod types;
// mod field;
mod newfield;
mod fxerror;
mod codegen;

use crate::newfield::{FXType, FXField, Message, Group, GroupInstance, GenericMessageBuilder};
// fn to_string<T>(value: &T) -> Result<String, FixError> 
//     where T: Serialize {
//         let mut serializer = FixSerializer { msg: String::new()};
//         value.serialize(&mut serializer)?;
//         Ok(serializer.msg)
//     }

// 8=FIX.4.4|9=126|35=A|49=theBroker.12345|56=CSERVER|34=1|52=20170117- 08:03:04|57=TRADE|50=any_string|98=0|108=30|141=Y|553=12345|554=passw0rd!|10=131| 

fn create_message() {
    let mut msg = Message::new();
    msg.add_field(44, FXType::Price(1.4));
    msg.add_field(38, FXType::Int(10000));
    msg.add_field(40, FXType::Currency("EUR".to_string()));
    msg.add_field(54, FXType::Bool(true));
    println!("Message = {:?}", msg);
}

fn main() {
    println!("Hello! World");
    let dict = codegen::create_data_dict("src/fix43/FIX43.xml").unwrap();
    // for f in dict.messages() {
    //     println!("{:?}", f);
    // }

    // for f in dict.fields_by_name() {
    //     println!("{:?}", f);
    // }

    // for f in dict.fields_by_tag() {
    //     println!("{:?}", f);
    // }

    // codegen::generate_code("src/fix43/fields.rs", dict);
    create_message();

}
