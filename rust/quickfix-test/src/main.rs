#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]

extern crate serde;
extern crate serde_derive;

mod serial;
mod fields;
mod types;
mod field;


// fn to_string<T>(value: &T) -> Result<String, FixError> 
//     where T: Serialize {
//         let mut serializer = FixSerializer { msg: String::new()};
//         value.serialize(&mut serializer)?;
//         Ok(serializer.msg)
//     }


fn main() {
    let logon: fields::Logon = Default::default();
    let mut msg = serial::to_string(&logon).unwrap();
    println!("Default logon = {}", &msg);
    let logon2 = fields::Logon::new();
    msg = serial::to_string(&logon2).unwrap();
    println!("With some fields set = {}", msg);
}
