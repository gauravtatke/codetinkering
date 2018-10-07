use std::collections::HashMap;
use std::fmt::Debug;

use types::integer_type::*;
use types::float_type::*;
use types::char_type::*;
// use types::string_type::*;
use types::data_type::*;
use types::FixType;
use types::{Getter, Setter};

#[derive(Debug)]
pub enum FXType {
    Int(i64),
    Length(u32),
    TagNum(u32),
    DayOfMonth(u8),
    SeqNum(u64),
    Float(f64),
    Price(f64),
    PriceOffset(f64),
    Amt(f64),
    Percent(f64),
    Qty(f64),
    Char(char),
    Bool(bool),
    Str(String),
    Currency(String)
}

pub struct FXField {
    tag: u32,
    value: FXType
}


pub trait SetValue<Other=Self> {
    fn set_value(&mut self, val: Other);
}
