#[allow(unused_imports)]

mod types;

use crate::types::*;

fn main() {
    let int_fix = Int::new(-67);
    let char_fix = Char::new('g');
    let bool_fix = Bool::new(true);
    let float_fix = Float::new(-65.8754);
    let str_fix = Str::new("my life");

    println!("{:?}", int_fix);
    println!("{}", int_fix);
    println!("Message field {:?}", MessageField::from(int_fix));
    println!("{:?}", char_fix);
    println!("{}", char_fix);
    println!("Message field {:?}", MessageField::from(char_fix));
    println!("{:?}", bool_fix);
    println!("{}", bool_fix);
    println!("Message field {:?}", MessageField::from(bool_fix));
    println!("{:?}", float_fix);
    println!("{}", float_fix);
    println!("Message field {:?}", MessageField::from(float_fix));
    println!("{:?}", str_fix);
    println!("{}", str_fix);
    println!("Message field {:?}", MessageField::from(str_fix));
}
