#[allow(unused_imports)]

mod types;
mod quickfix_errors;
mod message;

use crate::types::*;


fn main() {
    let int_fix = Int::new(-67);
    let char_fix = Char::new('g');
    let bool_fix = Bool::new(true);
    let float_fix = Float::new(-65.8754);
    // let str_fix = Str::new("my life");
    // println!("{:?}", str_fix);
    // println!("{}", str_fix);
    // println!("Message field {:?}", FixTypeField::from(str_fix));
}
