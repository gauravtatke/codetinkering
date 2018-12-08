use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FieldParseErrorKind {
    EqualSignNotFound,
    // InvalidTagNumber,
    // InvalidValueForTag,
    // ValueOutOfRangeForTag,
}

#[derive(Debug)]
pub struct FieldParseError {
    pub kind: FieldParseErrorKind
}

impl fmt::Display for FieldParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FieldParseErrorKind::EqualSignNotFound => {
                write!(f, "Equal(=) sign not found ")
            },
            _ => {
                write!(f, "Something went wrong while parsing")
            }
        }
    }
}

impl Error for FieldParseError {
}

#[derive(Debug)]
pub enum MessageParseErrorKind {
    NoRepeatingGroup
}

#[derive(Debug)]
pub struct MessageParseError {
    pub kind: MessageParseErrorKind
}

impl Error for MessageParseError{
   //TODO 
}

impl fmt::Display for MessageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            MessageParseErrorKind::NoRepeatingGroup => {
                write!(f, "No repeating group for this tag")
            },
            _ => {
                write!(f, "Something went wrong while parsing")
            }
        }
    }
}