use std::fmt::Display;
use std::fmt;



#[derive(Clone, Debug)]
struct Int (u32);

impl Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Int {
    fn new(val: u32) -> Self {
        Int(val)
    }    
}


#[derive(Clone, Debug)]
struct Float (f64);

impl Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Float {
    fn new(val: f64) -> Self {
        Float(val)
    }
}

#[derive(Debug)]
struct Str (String);

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct IntField {
    pub tag: u32,
    pub val: u32,
}


struct Field<T> {
    tag: u32,
    value: T
}

impl<T> Field<T> {
    fn new(tagnum: u32, val: T) -> Self {
        Self {
            tag: tagnum,
            value: val
        }
    }

    fn get_tag(&self) -> u32 {
        self.tag
    }

    fn get_val(&self) -> &T {
        &self.value
    }

    fn get_val_mut(&mut self) -> &mut T {
        &mut self.value
    }
}



#[allow(unused_variables)]
fn main() {
    let mut charfield = Field::new(35, 'A');
    let mut intfield = Field::new(40, 2);
    let mut strfield = Field::new(58, "random");

    println!("###### just printing #####");
    println!("char = {}, int = {}, str = {}", charfield.get_val(), intfield.get_val(), strfield.get_val());

    // let char2 = charfield.get_val();
    // let str2 = strfield.get_val();
    // let int2 = intfield.get_val();

    println!("###### printing old values #####");
    println!("char = {}, int = {}, str = {}", charfield.get_val(), intfield.get_val(), strfield.get_val());

    // println!("###### printing new variables #####");
    // println!("char = {}, int = {}, str = {}", char2, int2, str2);
    {
        let mut char3 = charfield.get_val_mut();
        let mut str3 = strfield.get_val_mut();
        let mut int3 = intfield.get_val_mut();

        let mut char3 = &mut 'D';
        let mut int3 = &mut 87;
        let mut str3 = &mut "gaurav tatke";

        println!("###### printing new mut variables #####");
        println!("char = {}, int = {}, str = {}", char3, int3, str3);
    }
    

    println!("###### printing old values #####");
    println!("char = {}, int = {}, str = {}", charfield.get_val(), intfield.get_val(), strfield.get_val());

}