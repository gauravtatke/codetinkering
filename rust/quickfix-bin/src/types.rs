use std::num::ParseIntError;
use std::convert::TryFrom;
use std::fmt::{self, Formatter};

#[derive(Debug)]
pub enum FixType {
    INT,
    FLOAT,
    STRING,
    BOOL,
    CHAR
}

#[derive(Debug)]
pub struct MessageField {
    field_type: FixType,
    data: String
}

#[derive(Debug)]
pub struct Int (i64);

impl Int {
    pub fn new<T: Into<i64>>(value: T) -> Int {
        Int (value.into())
    }
}
impl fmt::Display for Int {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Int> for MessageField {
    fn from(value: Int) -> MessageField {
        MessageField {
            field_type: FixType::INT,
            data: value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Float (f64);

impl Float {
    pub fn new<T: Into<f64>>(value: T) -> Float {
        Float (value.into())
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Float> for MessageField {
    fn from(value: Float) -> MessageField {
        MessageField {
            field_type: FixType::FLOAT,
            data: value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Str (String);

impl Str {
    pub fn new<T: Into<String>>(value: T) -> Str {
        Str (value.into())
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Str> for MessageField {
    fn from(value: Str) -> MessageField {
        MessageField {
            field_type: FixType::STRING,
            data: value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Char (char);

impl Char {
    pub fn new<T: Into<char>>(value: T) -> Char {
        Char (value.into())
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Char> for MessageField {
    fn from(value: Char) -> MessageField {
        MessageField {
            field_type: FixType::CHAR,
            data: value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Bool (bool);

impl Bool {
    pub fn new<T: Into<bool>>(value: T) -> Bool {
        Bool (value.into())
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0 {
            write!(f, "{}", 'Y')
        } else {
            write!(f, "{}", 'N')
        }
    }
}

impl From<Bool> for MessageField {
    fn from(value: Bool) -> MessageField {
        MessageField {
            field_type: FixType::BOOL,
            data: value.to_string()
        }
    }
}