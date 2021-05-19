#![allow(dead_code)]
#![allow(unused_imports)]
use std::cmp::Ordering;
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};

use crate::quickfix_errors::*;
use crate::types::*;

type Tag = u32;
// pub const SOH: char = '\u{01}';
pub const SOH: char = '|';

#[derive(Debug, Clone)]
pub struct Field {
    tag: Tag,
    str_value: String,
}

impl Field {
    fn new<T: ToString>(tagnum: Tag, field: T) -> Self {
        Self {
            tag: tagnum,
            str_value: field.to_string(),
        }
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_int(&self) -> Result<Int, SessionLevelRejectErr> {
        self.str_value.parse::<Int>()
    }

    pub fn get_float(&self) -> Result<Float, SessionLevelRejectErr> {
        self.str_value.parse::<Float>()
    }

    pub fn get_char(&self) -> Result<Char, SessionLevelRejectErr> {
        self.str_value.parse::<Char>()
    }

    pub fn get_bool(&self) -> Result<Bool, SessionLevelRejectErr> {
        self.str_value.parse::<Bool>()
    }

    pub fn get_str(&self) -> Result<String, SessionLevelRejectErr> {
        Ok(self.str_value.to_owned())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}={}", self.tag, self.str_value)
    }
}

pub trait MessageBuilder {
    fn add_field<T: ToString>(&mut self, tag: Tag, value: T);
    fn get_field(&self, tag: Tag) -> Option<&Field>;
    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field>;
    fn add_group(&mut self, tag: Tag, group: Group);
    fn get_group(&self, tag: Tag) -> Option<&Group>;
    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group>;

    fn set_int<T: Into<Int>>(&mut self, tag: Tag, value: T) {
        self.add_field(tag, value.into())
    }

    fn get_int(&self, tag: Tag) -> Result<Int, SessionLevelRejectErr> {
        match self
            .get_field(tag)
            .ok_or_else(|| SessionLevelRejectErr::required_tag_missing_err())
        {
            Ok(field) => field.get_int(),
            Err(e) => Err(e),
        }
    }

    fn set_float<T: Into<Float>>(&mut self, tag: Tag, value: T) {
        self.add_field(tag, value.into());
    }

    fn get_float(&self, tag: Tag) -> Result<Float, SessionLevelRejectErr> {
        match self
            .get_field(tag)
            .ok_or_else(|| SessionLevelRejectErr::required_tag_missing_err())
        {
            Ok(field) => field.get_float(),
            Err(e) => Err(e),
        }
    }

    fn set_char<T: Into<Char>>(&mut self, tag: Tag, value: T) {
        self.add_field(tag, value.into());
    }

    fn get_char(&self, tag: Tag) -> Result<Char, SessionLevelRejectErr> {
        match self
            .get_field(tag)
            .ok_or_else(|| SessionLevelRejectErr::required_tag_missing_err())
        {
            Ok(field) => field.get_char(),
            Err(e) => Err(e),
        }
    }

    fn set_bool<T: Into<Bool>>(&mut self, tag: Tag, value: T) {
        self.add_field(tag, value.into());
    }

    fn get_bool(&self, tag: Tag) -> Result<Bool, SessionLevelRejectErr> {
        match self
            .get_field(tag)
            .ok_or_else(|| SessionLevelRejectErr::required_tag_missing_err())
        {
            Ok(field) => field.get_bool(),
            Err(e) => Err(e),
        }
    }

    fn set_string(&mut self, tag: Tag, value: String) {
        self.add_field(tag, value);
    }

    fn get_string(&self, tag: Tag) -> Result<String, SessionLevelRejectErr> {
        match self
            .get_field(tag)
            .ok_or_else(|| SessionLevelRejectErr::required_tag_missing_err())
        {
            Ok(field) => field.get_str(),
            Err(e) => Err(e),
        }
    }

    fn set_group(&mut self, tag: Tag, delim: Tag, value: u32) -> &mut Group {
        self.add_field(tag, value);
        let new_group = Group::new(delim, value as usize);
        self.add_group(tag, new_group);
        self.get_mut_group(tag).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct FieldMap {
    field_map: HashMap<Tag, Field>,
    group_map: HashMap<Tag, Group>,
    field_order: Option<Vec<Tag>>,
    calc_vec_str: Vec<String>,
}

impl Default for FieldMap {
    fn default() -> Self {
        Self {
            field_map: HashMap::new(),
            group_map: HashMap::new(),
            field_order: None,
            calc_vec_str: Vec::new(),
        }
    }
}

impl FieldMap {
    fn new() -> Self {
        Default::default()
    }

    fn with_field_order(f_order: &[u32]) -> Self {
        Self {
            field_order: Some(f_order.to_vec()),
            ..Default::default()
        }
    }

    fn iter(&self) -> FieldMapIter {
        let mut field_map_iter = FieldMapIter::new();
        field_map_iter.flatten_field_map(&self);
        field_map_iter
    }

    fn field_order(&self) -> &Option<Vec<Tag>> {
        &self.field_order
    }

    fn set_field_order(&mut self, f_ord: &[u32]) {
        self.field_order = Some(f_ord.to_vec());
    }

    fn is_ordered_field(&self, tag: Tag) -> bool {
        match self.field_order() {
            Some(forder) => forder.contains(&tag),
            None => false,
        }
    }

    fn field_order_index(&self, tag: Tag) -> u32 {
        self.field_order()
            .as_ref()
            .map_or_else(u32::max_value, |v| {
                v.iter()
                    .position(|&x| x == tag)
                    .map_or_else(u32::max_value, |y| y as u32)
            })
    }

    fn field_comparator(&self, tag1: Tag, tag2: Tag) -> Ordering {
        self.field_order_index(tag1)
            .cmp(&self.field_order_index(tag2))
    }
}

impl MessageBuilder for FieldMap {
    fn add_field<T: ToString>(&mut self, tag: Tag, value: T) {
        self.field_map
            .insert(tag, Field::new(tag, value.to_string()));
    }

    fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.field_map.get(&tag)
    }

    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field> {
        self.field_map.get_mut(&tag)
    }

    fn add_group(&mut self, tag: Tag, group: Group) {
        self.group_map.insert(tag, group);
    }

    fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.group_map.get(&tag)
    }

    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.group_map.get_mut(&tag)
    }
}

#[derive(Debug)]
pub struct FieldMapIter<'a> {
    field_vec: Vec<(Tag, &'a Field)>,
}

impl<'a> FieldMapIter<'a> {
    fn new() -> Self {
        FieldMapIter {
            field_vec: Vec::with_capacity(4),
        }
    }

    fn push_back(&mut self, tag: Tag, field: &'a Field) {
        self.field_vec.push((tag, field));
    }

    // fn push_front(&mut self, tag: Tag, field: &'a Field) {
    //     self.field_vec.push_front((tag, field));
    // }

    fn flatten_field_map(&mut self, fmap: &'a FieldMap) {
        let mut temp_vec = fmap.field_map.iter().collect::<Vec<_>>();
        if fmap.field_order().is_some() {
            temp_vec.sort_unstable_by(|f1, f2| fmap.field_comparator(*f1.0, *f2.0));
        }

        for (tag, field) in temp_vec {
            self.push_back(*tag, field);
            if let Some(sub_group) = fmap.get_group(*tag) {
                for subgrp_inst in sub_group {
                    self.flatten_field_map(subgrp_inst);
                }
            }
        }
    }
}

impl<'a> IntoIterator for FieldMapIter<'a> {
    type Item = (Tag, &'a Field);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.field_vec.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    subgroup_delimiter: Tag,
    group_list: Vec<FieldMap>,
}

impl Group {
    fn new(delim: Tag, capacity: usize) -> Self {
        Self {
            subgroup_delimiter: delim,
            group_list: vec![FieldMap::new(); capacity],
        }
    }

    fn set_delimiter(&mut self, delim: Tag) {
        self.subgroup_delimiter = delim;
    }

    fn add(&mut self, instance: FieldMap) -> usize {
        self.group_list.push(instance);
        self.group_list.len()
    }

    fn length(&self) -> u32 {
        self.group_list.len() as u32
    }
}

impl Index<usize> for Group {
    type Output = FieldMap;

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
    type Item = &'a FieldMap;
    type IntoIter = std::slice::Iter<'a, FieldMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.group_list.iter()
    }
}

impl<'a> IntoIterator for &'a mut Group {
    type Item = &'a mut FieldMap;
    type IntoIter = std::slice::IterMut<'a, FieldMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.group_list.iter_mut()
    }
}

#[derive(Debug)]
pub struct Header(FieldMap);

impl Header {
    pub fn new() -> Self {
        let mut header_map = FieldMap::new();
        header_map.set_field_order(&[8, 9, 35]);
        Header(header_map)
    }

    // pub fn with_header(other: Header) -> Self {
    //     // Reverses sender and target ids, subids, compids, onbehalfof ids
    //     // message type needs to be set explicitly
    // }

    pub fn set_begin_str(&mut self, s: &str) {
        self.0.set_string(8, s.to_owned());
    }

    pub fn begin_str(&self) -> Result<String, FieldNotPresentError> {
        self.0.get_string(8).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn set_body_length<T: Into<Int>>(&mut self, length: T) {
        self.set_int(9, length);
    }

    pub fn body_length(&self) -> Result<Int, FieldNotPresentError> {
        self.0.get_int(9).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn set_sender_compid(&mut self, id: &str) {
        self.set_string(49, id.to_owned());
    }

    pub fn sender_compid(&self) -> Result<String, FieldNotPresentError> {
        self.0.get_string(49).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn set_target_compid(&mut self, id: &str) {
        self.set_string(56, id.to_owned());
    }

    pub fn target_compid(&self) -> Result<String, FieldNotPresentError> {
        self.0.get_string(56).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn set_onbehalf_of_compid(&mut self, id: &str) {
        self.set_string(115, id.to_owned());
    }

    pub fn set_deliver_to_compid(&mut self, id: &str) {
        self.set_string(128, id.to_owned());
    }

    pub fn set_sender_subid(&mut self, id: &str) {
        self.set_string(50, id.to_owned());
    }

    pub fn set_sender_locationid(&mut self, id: &str) {
        self.set_string(142, id.to_owned());
    }

    pub fn set_target_subid(&mut self, id: &str) {
        self.set_string(57, id.to_owned());
    }

    pub fn set_target_locationid(&mut self, id: &str) {
        self.set_string(143, id.to_owned());
    }

    pub fn set_onbehalf_of_subid(&mut self, id: &str) {
        self.set_string(116, id.to_owned());
    }

    pub fn set_onbehalf_of_locationid(&mut self, id: &str) {
        self.set_string(144, id.to_owned());
    }

    pub fn set_deliver_to_subid(&mut self, id: &str) {
        self.set_string(129, id.to_owned());
    }

    pub fn set_deliver_to_locationid(&mut self, id: &str) {
        self.set_string(145, id.to_owned());
    }

    pub fn set_poss_dup_flag(&mut self, dup_flag: bool) {
        self.set_bool(43, dup_flag);
    }

    pub fn set_poss_resend(&mut self, resend_flag: bool) {
        self.set_bool(97, resend_flag);
    }

    pub fn set_msg_seqnum<T: Into<Int>>(&mut self, seq_num: T) {
        self.set_int(34, seq_num);
    }

    pub fn set_msg_type(&mut self, msg_type: &str) {
        self.set_string(35, msg_type.to_owned());
    }

    pub fn msg_type(&self) -> Result<String, FieldNotPresentError> {
        self.0.get_string(35).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn set_sending_time(&mut self, send_time: &str) {
        // TODO: Chnage to UTC Time
        self.set_string(52, send_time.to_owned());
    }

    pub fn sending_time(&self) -> Result<String, FieldNotPresentError> {
        self.0.get_string(52).or_else(|_| Err(FieldNotPresentError))
    }

    pub fn iter(&self) -> FieldMapIter {
        self.0.iter()
    }
}

impl MessageBuilder for Header {
    fn add_field<T: ToString>(&mut self, tag: Tag, value: T) {
        self.0.add_field(tag, value);
    }

    fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.0.get_field(tag)
    }

    fn get_mut_field(&mut self, tag: Tag) -> Option<&mut Field> {
        self.0.get_mut_field(tag)
    }

    fn add_group(&mut self, tag: Tag, group: Group) {
        self.0.add_group(tag, group);
    }

    fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.0.get_group(tag)
    }

    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.0.get_mut_group(tag)
    }
}
// struct Trailer(FieldMap);

#[derive(Debug)]
pub struct Message {
    message_header: Header,
    message_body: FieldMap,
    message_trailer: FieldMap,
}

impl Message {
    pub fn new() -> Self {
        Self {
            message_header: Header::new(),
            message_body: FieldMap::new(),
            message_trailer: FieldMap::new(),
        }
    }

    pub fn header(&self) -> &Header {
        &self.message_header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.message_header
    }

    pub fn body(&self) -> &FieldMap {
        &self.message_body
    }

    pub fn body_mut(&mut self) -> &mut FieldMap {
        &mut self.message_body
    }

    pub fn trailer(&self) -> &FieldMap {
        &self.message_trailer
    }

    pub fn trailer_mut(&mut self) -> &mut FieldMap {
        &mut self.message_trailer
    }

    pub fn iter(&self) -> FieldMapIter {
        let mut message_iter = self.header().iter();
        // message_iter.flatten_field_map(self.header());
        message_iter.flatten_field_map(self.body());
        message_iter.flatten_field_map(self.trailer());
        message_iter
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printable = String::with_capacity(100);
        for (_, field) in self.iter() {
            printable.push_str(field.to_string().as_str());
            printable.push(SOH);
        }
        write!(f, "{}", printable)
    }
}

pub mod store {
    pub trait MessageStore {
        // This is for defining next/previous sequence number retrieval for sender and target
        // currently keeping it empty.
    }

    pub struct DefaultMessageStore;
    impl DefaultMessageStore {
        pub fn new() -> Self {
            Self
        }
    }

    impl MessageStore for DefaultMessageStore {}

    pub trait LogStore {
        // this is just for logging the message somewhere. currently empty
    }

    pub struct DefaultLogStore;
    impl DefaultLogStore {
        pub fn new() -> Self {
            Self
        }
    }
    impl LogStore for DefaultLogStore {}
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn test_message_without_group() {
        let mut msg = Message::new();
        msg.header_mut().set_string(8, "FIX4.3".to_string());
        msg.header_mut().set_string(49, "Gaurav".to_string());
        msg.header_mut().set_string(56, "Tatke".to_string());

        msg.body_mut().set_int(34, 8765);
        msg.body_mut().set_float(44, 1.87856);
        msg.body_mut().set_bool(654, true);
        msg.body_mut().set_char(54, 'b');
        msg.body_mut().set_string(1, "BOX_AccId".to_string());

        msg.trailer_mut().set_int(10, 101);
        assert!(!msg.to_string().is_empty());
    }

    #[test]
    fn test_message_with_group() {
        let mut msg = Message::new();
        msg.header_mut().set_string(8, "FIX4.3".to_string());
        msg.header_mut().set_string(49, "Gaurav".to_string());
        msg.header_mut().set_string(56, "Tatke".to_string());

        let partyid_grp = msg.body_mut().set_group(453, 448, 3);
        assert_eq!(partyid_grp.length(), 3);
        partyid_grp[0].set_int(448, 3);
        partyid_grp[0].set_int(447, 78);
        partyid_grp[0].set_char(452, 'D');

        partyid_grp[1].set_int(448, 13);
        partyid_grp[1].set_int(447, 79);
        partyid_grp[1].set_char(452, 'E');

        msg.trailer_mut().set_int(10, 101);
        let printable = msg.to_string();
        println!("{}", printable);
        assert!(!printable.is_empty());
    }

    #[test]
    fn test_fieldmap_ordering_fields() {
        let mut test_fmap1 = FieldMap::new();
        test_fmap1.set_field_order(&[8, 9, 35]);
        test_fmap1.set_string(35, "A".to_string());
        test_fmap1.set_string(8, "FIX.4.3".to_string());
        test_fmap1.set_int(9, 101);
        test_fmap1.set_string(49, "sender_id".to_string());
        test_fmap1.set_string(56, "target_id".to_string());

        // verify that output string has ordering of 8, 9, 35
        let mut fmap_iter = test_fmap1
            .iter()
            .into_iter()
            .map(|f| f.0)
            .filter(|&t| t == 8 || t == 9 || t == 35);
        assert_eq!(fmap_iter.next(), Some(8));
        assert_eq!(fmap_iter.next(), Some(9));
        assert_eq!(fmap_iter.next(), Some(35));

        // test same as above but set field order after setting fields

        let mut test_fmap2 = FieldMap::new();
        test_fmap2.set_string(35, "A".to_string());
        test_fmap2.set_string(8, "FIX.4.3".to_string());
        test_fmap2.set_int(9, 101);
        test_fmap2.set_string(49, "sender_id".to_string());
        test_fmap2.set_string(56, "target_id".to_string());
        test_fmap2.set_field_order(&[35, 8, 9]);
        // verify that output string has ordering of 8, 9, 35
        let mut fmap_iter2 = test_fmap2
            .iter()
            .into_iter()
            .map(|f| f.0)
            .filter(|&t| t == 8 || t == 9 || t == 35);
        assert_eq!(fmap_iter2.next(), Some(35));
        assert_eq!(fmap_iter2.next(), Some(8));
        assert_eq!(fmap_iter2.next(), Some(9));
    }
}
