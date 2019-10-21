use std::fmt::{self, Formatter};
use std::ops::{Index, IndexMut};
use std::iter::{IntoIterator, Iterator};
use std::collections::HashMap;

use crate::quickfix_errors::*;
use crate::types::*;

type Tag = u32;

#[derive(Debug)]
pub struct Field {
    tag: Tag,
    str_value: String
}

impl Field {
    fn new<T: ToString>(tagnum: Tag, field: T) -> Self {
        Self {
            tag: tagnum,
            str_value: field.to_string()
        }
    }

    fn get_int(&self) -> Result<Int, FixTypeFieldParseError> {
        self.str_value.parse::<Int>()
    }

    fn get_float(&self) -> Result<Float, FixTypeFieldParseError> {
        self.str_value.parse::<Float>()
    }

    fn get_char(&self) -> Result<Char, FixTypeFieldParseError> {
        self.str_value.parse::<Char>()
    }

    fn get_bool(&self) -> Result<Bool, FixTypeFieldParseError> {
        self.str_value.parse::<Bool>()
    }

    fn get_str(&self) -> Result<String, FixTypeFieldParseError> {
        Ok(self.str_value.to_owned())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}={}", self.tag, self.str_value)
    }
}


#[derive(Debug)]
struct FieldMap (HashMap<Tag, Field>);

impl FieldMap {
    fn new() -> FieldMap {
        FieldMap (HashMap::new())
    }

    fn insert(&mut self, tag: Tag, value: Field) {
        self.0.insert(tag, value);
    }

    fn get(&self, tag: Tag) -> Option<&Field> {
        self.0.get(&tag)
    }

    fn get_mut(&mut self, tag: Tag) -> Option<&mut Field> {
        self.0.get_mut(&tag)
    }
}

pub trait MessageBuilder {
    fn add_field(&mut self, tag: Tag, field: Field);
    fn get_field(&self, tag: Tag) -> Option<&Field>;
    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field>;
    fn add_group(&mut self, tag: Tag, group: Group);
    fn get_group(&self, tag: Tag) -> Option<&Group>;
    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group>;
}

#[derive(Debug)]
pub struct GroupInstance {
    group_fields: FieldMap,
    sub_groups: HashMap<Tag, Group>
}

impl GroupInstance {
    fn new() -> GroupInstance {
        GroupInstance {
            group_fields: FieldMap::new(),
            sub_groups: HashMap::new()
        }
    }

    fn set_int<T: Into<Int>>(&mut self, tag: Tag, value: T) {
            self.add_field(tag, Field::new(tag, value.into()));
    }

    fn get_int(&self, tag: Tag) -> Option<Int> {
        self.get_field(tag).map_or(None, |fld| fld.get_int().ok())
    }

    fn set_float<T: Into<Float>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn get_float(&self, tag: Tag) -> Option<Float> {
        self.get_field(tag).map_or(None, |fld| fld.get_float().ok())
    }

    fn set_string<T: ToString>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.to_string()));
    }

    fn get_string(&self, tag: Tag) -> Option<String> {
        self.get_field(tag).map_or(None, |fld| fld.get_str().ok())
    }

    fn set_char<T: Into<Char>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn get_char(&self, tag: Tag) -> Option<Char> {
        self.get_field(tag).map_or(None, |fld| fld.get_char().ok())
    }

    fn set_bool<T: Into<Bool>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn get_bool(&self, tag: Tag) -> Option<Bool> {
        self.get_field(tag).map_or(None, |fld| fld.get_bool().ok())
    }

    fn set_subgroup(&mut self, tag: Tag, delim_tag: Tag, num_subgrp: u32) -> &mut Group {
        self.add_field(tag, Field::new(tag, Int::new(num_subgrp)));
        let sub_group = Group::new(delim_tag, num_subgrp as usize);
        self.add_group(tag, sub_group);
        self.get_mut_group(tag).unwrap()
    }
}

impl MessageBuilder for GroupInstance {
    fn add_field(&mut self, tag: Tag, field: Field) {
        self.group_fields.insert(tag, field);
    }

    fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.group_fields.get(tag)
    }

    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field> {
        self.group_fields.get_mut(tag)
    }

    fn add_group(&mut self, tag: Tag, group: Group) {
        self.sub_groups.insert(tag, group);
    }

    fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.sub_groups.get(&tag)
    }

    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.sub_groups.get_mut(&tag)
    }
}

#[derive(Debug)]
pub struct Group {
    subgroup_delimiter: Tag,
    group_list: Vec<GroupInstance>
}

impl Group {
    fn new(delim: Tag, capacity: usize) -> Self {
        Self {
            subgroup_delimiter: delim,
            group_list: Vec::with_capacity(capacity)
        }
    }

    fn set_delimiter(&mut self, delim: Tag) {
        self.subgroup_delimiter = delim;
    }

    fn add(&mut self, instance: GroupInstance) -> usize {
        self.group_list.push(instance);
        self.group_list.len()
    }

    fn length(&self) -> u32 {
        self.group_list.len() as u32
    }
}

impl Index<usize> for Group {
    type Output = GroupInstance;

    fn index(&self, idx: usize) -> &Self::Output {
        self.group_list.index(idx)
    }
}

impl IndexMut<usize> for Group {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.group_list.index_mut(idx)
    }
}

impl<'a> IntoIterator for &'a Group {
    type Item = &'a GroupInstance;
    type IntoIter = std::slice::Iter<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_list.iter()
    }
}

impl<'a> IntoIterator for &'a mut Group {
    type Item = &'a mut GroupInstance;
    type IntoIter = std::slice::IterMut<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_list.iter_mut()
    }
}

#[derive(Debug)]
pub struct Message {
    message_fields: FieldMap,
    message_groups: HashMap<Tag, Group>
}

impl MessageBuilder for Message {
    fn add_field(&mut self, tag: Tag, field: Field) {
        self.message_fields.insert(tag, field);
    }

    fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.message_fields.get(tag)
    }

    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field> {
        self.message_fields.get_mut(tag)
    }

    fn add_group(&mut self, tag: Tag, group: Group) {
        self.message_groups.insert(tag, group);
    }

    fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.message_groups.get(&tag)
    }

    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.message_groups.get_mut(&tag)
    }
}

impl Message {
    fn new() -> Self {
        Self {
            message_fields: FieldMap::new(),
            message_groups: HashMap::new()
        }
    }

    fn set_int<T: Into<Int>>(&mut self, tag: Tag, value: T) {
            self.add_field(tag, Field::new(tag, value.into()));
    }

    fn set_float<T: Into<Float>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn set_string<T: ToString>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.to_string()));
    }

    fn set_char<T: Into<Char>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn set_bool<T: Into<Bool>>(&mut self, tag: u32, value: T) {
        self.add_field(tag, Field::new(tag, value.into()));
    }

    fn get_int(&self, tag: Tag) -> Option<Int> {
        self.get_field(tag).map_or(None, |fld| fld.get_int().ok())
    }

    fn get_float(&self, tag: Tag) -> Option<Float> {
        self.get_field(tag).map_or(None, |fld| fld.get_float().ok())
    }

    fn get_char(&self, tag: Tag) -> Option<Char> {
        self.get_field(tag).map_or(None, |fld| fld.get_char().ok())
    }

    fn get_string(&self, tag: Tag) -> Option<String> {
        self.get_field(tag).map_or(None, |fld| fld.get_str().ok())
    }

    fn get_bool(&self, tag: Tag) -> Option<Bool> {
        self.get_field(tag).map_or(None, |fld| fld.get_bool().ok())
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

}