use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::result::Result;

// use types::integer_type::*;
// use types::float_type::*;
// use types::char_type::*;
// use types::string_type::*;
// use types::data_type::*;
// use types::FixType;
// use types::{Getter, Setter};
use crate::fxerror::*;


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

#[derive(Debug)]
pub struct FXField {
    tag: u32,
    value: FXType
}

impl FXField {
    fn new(tagnum: u32, val: FXType) -> FXField {
        FXField {
            tag: tagnum,
            value: val
        }
    }
}

impl FromStr for FXField {
    type Err = FieldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(FieldParseError {
            kind: FieldParseErrorKind::EqualSignNotFound
        })
    }

}

#[derive(Debug)]
pub struct FieldMap (HashMap<u32, FXField>);

impl FieldMap {
    fn new() -> Self {
        FieldMap (HashMap::new())
    }
    
    fn add(&mut self, tag: u32, val: FXType) {
        self.0.insert(tag, FXField {tag, value: val});
    }
    
    fn get(&self, tag: u32) -> Option<&FXField> {
        self.0.get(&tag)
    }
}

#[derive(Debug)]
pub struct Group {
    delim: u32,
    group_vec: Vec<GroupInstance>
}


impl Group {
    fn new(delim: u32, capacity: usize) -> Self {
        Self {
            delim,
            group_vec: Vec::with_capacity(capacity)
        }
    }
    
    fn push(&mut self, val: GroupInstance) {
        self.group_vec.push(val)
    }
    
}

impl Index<usize> for Group {
    type Output = GroupInstance;
    
    fn index(&self, idx: usize) -> &Self::Output {
        self.group_vec.index(idx)
    }
}

impl IndexMut<usize> for Group {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.group_vec.index_mut(idx)
    }
}

impl IntoIterator for Group {
    type Item = GroupInstance;
    type IntoIter = ::std::vec::IntoIter<GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.into_iter()
    }
}

impl<'a> IntoIterator for &'a Group {
    type Item = &'a GroupInstance;
    type IntoIter = ::std::slice::Iter<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.iter()
    }
}

impl<'a> IntoIterator for &'a mut Group {
    type Item = &'a mut GroupInstance;
    type IntoIter = ::std::slice::IterMut<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.iter_mut()
    }
}


trait GenericMessageBuilder {
    fn add_field(&mut self, tag: u32, val: FXType);
    fn get_field(&self, tag: u32) -> Option<&FXField>;
    fn set_group(&mut self, tag: u32, val: u32) -> &mut Group;
    fn get_group(&self, tag: u32) -> Result<&Group, MessageParseError>; 
}


#[derive(Debug)]
pub struct GroupInstance {
    grp_fields: FieldMap,
    sub_grp: HashMap<u32, Group>
}

impl GroupInstance {
    fn new() -> Self {
        Self {
            grp_fields: FieldMap::new(),
            sub_grp: HashMap::new()
        }
    }
}

impl GenericMessageBuilder for GroupInstance {

    fn add_field(&mut self, tag: u32, val: FXType) {
        self.grp_fields.add(tag, val);
    }
    
    fn get_field(&self, tag: u32) -> Option<&FXField> {
        self.grp_fields.get(tag)
    }
    
    fn set_group(&mut self, tag: u32, val: u32) -> &mut Group {
        self.add_field(tag, FXType::Length(val));
        let mut group = Group::new(tag, val as usize);
        for _i in 0..val {
            let sub_group = GroupInstance::new();
            group.push(sub_group);
        }
        self.sub_grp.entry(tag).or_insert(group)
    }
    
    fn get_group(&self, tag: u32) -> Result<&Group, MessageParseError> {
        self.sub_grp.get(&tag).ok_or(MessageParseError {
            kind: MessageParseErrorKind::NoRepeatingGroup
        })
    }
}

pub struct Message {
    fields: FieldMap,
    group: HashMap<u32, Group>
}

impl Message {
    fn new() -> Self {
        Self {
            fields: FieldMap::new(),
            group: HashMap::new()
        }
    }
}

impl GenericMessageBuilder for Message {

    fn add_field(&mut self, tag: u32, val: FXType) {
        self.fields.add(tag, val);
    }
    
    fn get_field(&self, tag: u32) -> Option<&FXField> {
        self.fields.get(tag)
    }
    
    fn set_group(&mut self, tag: u32, val: u32) -> &mut Group {
        self.add_field(tag, FXType::Length(val));
        let mut grp = Group::new(tag, val as usize);
        for _i in 0..val {
            let sub_group = GroupInstance::new();
            grp.push(sub_group);
        }
        self.group.entry(tag).or_insert(grp)
    }
    
    fn get_group(&self, tag: u32) -> Result<&Group, MessageParseError> {
        self.group.get(&tag).ok_or(MessageParseError {
            kind: MessageParseErrorKind::NoRepeatingGroup
        })
    }
}
