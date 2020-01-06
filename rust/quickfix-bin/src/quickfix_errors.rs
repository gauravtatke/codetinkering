use std::error::Error;
use std::fmt::{self, Formatter};

#[derive(Debug)]
pub struct FixTypeFieldParseError {
    pub kind: FixTypeFieldParseErrorKind,
}

#[derive(Debug)]
pub enum FixTypeFieldParseErrorKind {
    NotInt,
    NotFloat,
    NotBool,
    NotString,
    NotChar,
}

impl fmt::Display for FixTypeFieldParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Cannot convert to FixField")
    }
}

impl Error for FixTypeFieldParseError {
    fn description(&self) -> &str {
        match self.kind {
            FixTypeFieldParseErrorKind::NotInt => "Cannot parse data into Int",
            FixTypeFieldParseErrorKind::NotFloat => "Cannot parse data into Float",
            FixTypeFieldParseErrorKind::NotBool => "Cannot parse data into Bool",
            FixTypeFieldParseErrorKind::NotChar => "Cannot parse data into Char",
            FixTypeFieldParseErrorKind::NotString => "Cannot parse data into valid UTF8 String",
        }
    }
}

#[derive(Debug)]
pub struct FieldNotPresentError;

impl fmt::Display for FieldNotPresentError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Mandatory field not present")
    }
}

impl Error for FieldNotPresentError {
    fn description(&self) -> &str {
        "Mandatory field not present"
    }
}
