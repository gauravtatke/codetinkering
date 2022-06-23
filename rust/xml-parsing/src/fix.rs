use std::collections::{HashMap, HashSet};

/*
derive a macro which will create impl fns for each of the items in this enum
 and then delete this comment
 */
#[derive(Debug)]
pub enum Type {
    Int(i64),
    Length(u32),
    TagNum(u32),
    DayOfMonth(u32),
    SeqNum(u64),
    NumInGroup(u32),
    Float(f64),
    Price(f64),
    PriceOffset(f64),
    Amt(f64),
    Percent(f64),
    Qty(f64),
    Char(char),
    Bool(bool),
    Str(String),
    Currency(String),
    Country(String),
    Exchange(String),
    LocalMktDate(String),
    MonthYear(String),
    MultiValueStr(String),
    UtcDate(String),
    UtcTimeOnly(String),
    UtcTimestamp(String)
}

pub struct Field {
    tag: u32,
    value: Type
}

type FieldMap = HashMap<u32, Field>;
type GroupInstance = HashMap<u32, Type>;

pub struct Group {
    delim: u32,
    fields: Vec<GroupInstance>,
}

pub struct Message {
    fields: FieldMap,
    groups: Group,
}