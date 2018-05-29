// pub mod int_field {
//     use types::integer_type::*;
// }


// use types::integer_type::*;
// use types::float_type::*
// use types::char_type::*
// use types::string_type::*
use std::collections::HashMap;

type Chr = char;

type Bool = bool;

type Data = Vec<u8>;

type Float = f64;
type Amt = f64;
type Percent = f64;
type Price = f64;
type PriceOffset = f64;
type Qty = f64;

type Int = i64;
type DayOfMonth = u8;
type Length = u32;
type NumInGroup = u16;
type SeqNum = u64;
type TagNum = u32;

//type Str = String;
type Country = String;
type Exchange = String;
type LocalMktDate = String;
type MonthYear = String;
type MultiValueStr = String;
type UTCDateOnly = String;
type UTCTimeOnly = String;
type UTCTimeStamp = String;

pub struct Field<T> {
    tag: TagNum,
    value: T,
}

pub trait SetValue<Other=Self> {
    fn set_value(&mut self, v: Other);
}

pub struct FloatField<T> {
    tag: TagNum,
    value: T,
}

pub struct CharField<T> {
    tag: TagNum,
    value: T,
}

pub struct FieldMap<T> {
    fields: HashMap<TagNum, T>,
    groups: HashMap<TagNum, Vec<Group<T>>>,
}

impl<T> FieldMap<T> {
    pub fn new() -> Self {
        FieldMap {
            fields: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn get_field(&self, fl: TagNum) -> Option<&T> {
        self.fields.get(&fl)
    }

    pub fn get_mut_field(&mut self, fl: TagNum) -> Option<&mut T> {
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

pub struct Group<T> {
    delim: TagNum,
    fields: HashMap<TagNum, T>,
    groups: HashMap<TagNum, Vec<Group<T>>>
}

impl<T> Group<T> {
    pub fn new() -> Self {
        Group {
            delim: 0,
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

pub struct Message<T> {
    header: HashMap<TagNum, T>,
    trailer: HashMap<TagNum, T>,
    fields: HashMap<TagNum, T>,
    groups: HashMap<TagNum, Vec<Group<T>>>,
}

impl<T> Message<T> {
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





