#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]

extern crate serde;
extern crate serde_derive;

mod serial;
mod fields;
mod types;
// mod field;
mod newfield;


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
    // let logon: fields::Logon = Default::default();
    // let mut msg = serial::to_string(&logon).unwrap();
    // println!("Default logon = {}", &msg);
    // let logon2 = fields::Logon::new();
    // msg = serial::to_string(&logon2).unwrap();
    // println!("With some fields set = {}", msg);


}
