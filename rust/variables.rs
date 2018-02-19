fn main(){
    let x = 5; //immutable x
    // x = x+1; will throw error
    let mut y = 6; //mutable y
    println!("x = {}, y = {}", x, y);
    y = y+1;
    let x = "a string"; // this declares a new variable x and binds it to string
    println!("x = {}, y = {}", x, y);
    // y = "a second string"; error because type cannot be mutated, only value can
    const MAX_VALUE: u32 = 1000000; //constant declaration
    println!("constant max_val = {}", MAX_VALUE);
    datatype()
}

fn datatype(){
    println!("THE SCALAR TYPES---->");
    let uint_a = 34; //default unsigned int
    let sint_b: i32 = 87765; // with type annotation for signed 32 bit int
    let float_c = 2.5; //f64 -> 64 bit float
    let float_d: f32 = 6.43; // f32
    let with_sep = 43_23_100; // equivalent to 4323100 -> '_' can be used as visual separator
    let hexa_f = 0xff; //hexadecimal
    let oct_g = 0o77; //octal
    let bin_h = 0b1111_0000; //binary 11110000
    let byte_i = b'A'; // byte u8 only
    println!("integer = {}, {}, {}, {}, {}, {}, {}", uint_a, sint_b, with_sep, hexa_f, oct_g, bin_h, byte_i);
    println!("float = {}, {}", float_c, float_d);
    let bool_t = true;
    let bool_f: bool = false;
    println!("boolean = {}, {}", bool_t, bool_f);
    let ascii_c = 'z'; //can take unicode scalar value. a lot more than ascii
    let unicode_z: char = 'ℤ'; // unicode value
    let emoji: char = '😻'; //even emojis
    println!("char = {}, {}, {}", ascii_c, unicode_z, emoji);
}
