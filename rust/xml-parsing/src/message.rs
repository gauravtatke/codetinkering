use std::collections::binary_heap::Iter;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::hash::Hash;
use std::iter::Peekable;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::data_dictionary::DataDictionary;
use crate::quickfix_errors::SessionRejectError;

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
    tag: Tag,
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

    pub fn set_field<T: ToString>(&mut self, tag: Tag, value: T) {
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
            return field.value.parse::<T>().or(Err("could not parse".to_string()));
        }
        Err("not found".to_string())
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        self.set_field(tag, value);
        let group = self.group.entry(tag).or_insert(Group::new(rep_grp_delimiter));
        // create group instances and insert into group
        for i in 0..value {
            group.add_group(FieldMap::new());
        }
        group
    }

    pub fn set_field_order(&mut self, f_order: &[Tag]) {
        self.field_order = Some(f_order.to_vec());
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

    pub fn size(&self) -> u32 {
        self.fields.len() as u32
    }
}

impl Index<usize> for Group {
    type Output = FieldMap;

    fn index(&self, idx: usize) -> &Self::Output {
        self.fields.index(idx)
    }
}

impl IndexMut<usize> for Group {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.fields.index_mut(idx)
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

    pub fn trailer(&self) -> &FieldMap {
        &self.trailer
    }

    pub fn trailer_mut(&mut self) -> &mut FieldMap {
        &mut self.trailer
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

    fn add_group(&mut self, tag: Tag, grp: Group) {
        self.body.group.insert(tag, grp);
    }

    pub fn set_checksum(&mut self) {
        todo!()
    }

    pub fn from_str<'a>(s: &'a str, dd: &DataDictionary) -> Result<Self, SessionRejectError> {
        let mut vdeq: VecDeque<(u32, &str)> = VecDeque::with_capacity(16);
        for field in s.split_terminator("|") {
            let (tag, value) = match field.split_once("=") {
                Some((t, v)) => {
                    let parse_result = t.parse::<u32>();
                    if parse_result.is_err() {
                        return Err(SessionRejectError::invalid_tag_err());
                    }
                    if v.is_empty() {
                        return Err(SessionRejectError::tag_without_value_err());
                    }
                    (parse_result.unwrap(), v)
                }
                None => return Err(SessionRejectError::invalid_tag_err()),
            };
            vdeq.push_back((tag, value));
        }

        Self::from_vec(vdeq, dd)
    }

    fn from_vec(
        mut v: VecDeque<(u32, &str)>, dd: &DataDictionary,
    ) -> Result<Self, SessionRejectError> {
        todo!()
    }
}

pub const SAMPLE_MSG: &str = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|34=2|52=2003061501:14:49|11=12345|1=111111|63=0|64=20030621|21=3|110=1000|111=50000|55=IBM|48=459200101|22=1|54=1|60=2003061501:14:49|38=5000|40=1|44=15.75|15=USD|59=0|10=127|";

pub struct MessageBuilder {}
