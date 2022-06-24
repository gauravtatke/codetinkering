use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

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
    UtcTimestamp(String),
}

type Tag = u32;
// pub const SOH: char = '\u{01}';
pub const SOH: char = '|';

#[derive(Debug)]
pub struct Field {
    tag: u32,
    value: String,
}

#[derive(Debug, Default)]
pub struct FieldMap {
    fields: HashMap<Tag, Field>,
    group: HashMap<Tag, Group>,
    field_order: Option<Vec<Tag>>,
    calc_vec_str: Vec<String>,
}

impl FieldMap {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    pub fn set_field<T: ToString>(&mut self, tag: u32, value: T) {
        self.fields.insert(
            tag,
            Field {
                tag,
                value: value.to_string(),
            },
        );
    }

    pub fn get_field<T: FromStr>(&self, tag: u32) -> Result<T, String> {
        if let Some(field) = self.fields.get(&tag) {
            return field
                .value
                .parse::<T>()
                .or(Err("could not parse".to_string()));
        }
        Err("not found".to_string())
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        self.set_field(tag, value);
        let group = self
            .group
            .entry(tag)
            .or_insert(Group::new(rep_grp_delimiter));
        // create group instances and insert into group
        for i in 0..value {
            group.add_group(FieldMap::new());
        }
        group
    }
}
// type FieldMap = HashMap<u32, Field>;
// type GroupInstance = HashMap<u32, Field>;
// type GroupMap = HashMap<u32, Group>;

#[derive(Debug, Default)]
pub struct Group {
    delim: u32,
    fields: Vec<FieldMap>,
    // groups: GroupMap,
}

impl Group {
    pub fn new(delimiter: Tag) -> Self {
        Self {
            delim: delimiter,
            fields: Vec::<FieldMap>::new(),
        }
    }

    pub fn add_group(&mut self, grp: FieldMap) {
        self.fields.push(grp);
    }
}

type Header = FieldMap;

#[derive(Debug, Default)]
pub struct Message {
    // fields: FieldMap,
    // groups: GroupMap,
    pub header: Header,
    pub body: FieldMap,
    trailer: FieldMap,
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn set_field<T: ToString>(&mut self, tag: Tag, value: T) {
        self.body.set_field(tag, value);
    }

    pub fn get_field<T: FromStr>(&self, tag: Tag) -> Result<T, String> {
        self.body.get_field(tag)
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        self.body.set_group(tag, value, rep_grp_delimiter)
    }
}

pub struct MessageBuilder {}
