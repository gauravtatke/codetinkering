use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::result::Result;
use std::convert::Infallible;

#[derive(Debug)]
pub enum FixType {
    INT,
    FLOAT,
    STRING,
    // CHAR,
}

#[derive(Debug)]
pub struct Field {
    fxtype: FixType,
    data: String
}

impl Field {
    pub fn new<T:ToString>(fxtype: FixType, val: T) -> Self {
        Self {
            fxtype,
            data: val.to_string()
        }
    }
    
    pub fn get_int(&self) -> Int {
        match self.fxtype {
            FixType::INT => Int::new(i64::from_str(&self.data).unwrap()),
            _ => Int(0)
        }
    }
}

#[derive(Debug)]
pub struct Int(i64);

impl Int {
    pub fn new<T: Into<i64>>(val: T) -> Self {
        Int(val.into())
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Int {
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Int, ParseIntError> {
    let result = i64::from_str(s);
    match result {
        Ok(i) => Ok(Int::new(i)),
        Err(e) => Err(e)
        }
    }
}

impl Into<Field> for Int {
    
}

#[derive(Debug)]
pub struct Float(f64);

impl Float {
    pub fn new<T: Into<f64>>(val: T) -> Self {
        Float(val.into())
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Float {
    type Err = ParseFloatError;
    
    fn from_str(s: &str) -> Result<Float, ParseFloatError> {
    let result = f64::from_str(s);
    match result {
        Ok(f) => Ok(Float::new(f)),
        Err(e) => Err(e)
        }
    }
}

#[derive(Debug)]
pub struct Str(String);

impl Str {
    pub fn new<T: Into<String>>(val: T) -> Self {
        Str(val.into())
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Str {
    type Err = Infallible;
    
    fn from_str(s: &str) -> Result<Str, Infallible> {
    let result = String::from_str(s);
    match result {
        Ok(st) => Ok(Str::new(st)),
        Err(e) => Err(e)
        }
    }
}

fn main() {
    let flt32 = 10.9;
    let new_float = Float::new(flt32);
    let new_int = Int::new(19);
    println!("new int: {}", new_int);
    let new_int_string = new_int.to_string();
    println!("string of int: {}", new_int_string);
    println!("new int after: {}", new_int);
    println!("float value: {}", new_float);
    println!("Hello, world!");
}