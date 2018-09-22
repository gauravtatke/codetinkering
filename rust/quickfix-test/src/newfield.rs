use std::collections::HashMap;

use types::integer_type::*;
use types::float_type::*;
use types::char_type::*;
// use types::string_type::*;
use types::data_type::*;
use types::FixType;
use types::{Getter, Setter};

pub struct Field {
    tag: TagNum,
    value: Box<FixType>,
}

impl<T: FixType> Field<T> {
    pub fn get_tag(&self) -> &TagNum {
        &self.tag
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set_tag(&mut self, tag: TagNum) {
        self.tag = tag;
    }

    pub fn set_value(&mut self, val: T) {
        self.value = val;
    }
}

pub struct FieldMap<T: FixType> {
    fields: HashMap<TagNum, Field<T>>,
    groups: HashMap<TagNum, Vec<Group<T>>>
}

pub trait SetValue<Other=Self> {
    fn set_value(&mut self, val: Other);
}


pub struct Group<T: FixType> {
    delim: TagNum,
    fields: HashMap<TagNum, T>,
    groups: HashMap<TagNum, Vec<Group<T>>>
}

impl<T: FixType> Group<T> {
    pub fn new() -> Self {
        Group {
            delim: TagNum::new(),
            fields: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn delim(&self) -> TagNum {
        self.delim
    }

    pub fn set_delim(&mut self, de: TagNum) {
        self.delim = de;
    }

    // fn get_group(&self) -> &FieldMap<T> {
    //     &self.group
    // }

    // fn get_group_mut(&mut self) -> &mut FieldMap<T> {
    //     &mut self.group
    // }
    // some duplication

    pub fn get_field(&self, fl: TagNum) -> Option<&T> {
        self.fields.get(&fl)
    }

    fn get_mut_field(&mut self, fl: TagNum) -> Option<&mut T> {
        self.fields.get_mut(&fl)
    }

    pub fn get_group(&self, gnum: TagNum) -> Option<&Vec<Group<T>>> {
        self.groups.get(&gnum)
    }

    pub fn get_group_mut(&mut self, gnum: TagNum) -> Option<&mut Vec<Group<T>>> {
        self.groups.get_mut(&gnum)
    }

    pub fn set_field(&mut self, key: TagNum, val: T) -> Option<T> {
        self.fields.insert(key, val)
    }

    pub fn set_group(&mut self, key: TagNum, val: Vec<Group<T>>) -> Option<Vec<Group<T>>> {
        self.groups.insert(key, val)
    }
}

pub struct Message<T: FixType> {
    header: HashMap<TagNum, T>,
    trailer: HashMap<TagNum, T>,
    fields: HashMap<TagNum, T>,
    groups: HashMap<TagNum, Vec<Group<T>>>,
}

impl<T: FixType> Message<T> {
    pub fn new() -> Self {
        Message {
            header: HashMap::new(),
            trailer: HashMap::new(),
            fields: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn get_field(&self, fl: TagNum) -> Option<&T> {
        self.fields.get(&fl)
    }

    fn get_mut_field(&mut self, fl: TagNum) -> Option<&mut T> {
        self.fields.get_mut(&fl)
    }

    pub fn get_group(&self, gnum: TagNum) -> Option<&Vec<Group<T>>> {
        self.groups.get(&gnum)
    }

    pub fn get_group_mut(&mut self, gnum: TagNum) -> Option<&mut Vec<Group<T>>> {
        self.groups.get_mut(&gnum)
    }

    pub fn set_field(&mut self, key: TagNum, val: T) -> Option<T> {
        self.fields.insert(key, val)
    }

    pub fn set_group(&mut self, key: TagNum, val: Vec<Group<T>>) -> Option<Vec<Group<T>>> {
        self.groups.insert(key, val)
    }

}