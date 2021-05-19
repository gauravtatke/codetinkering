use std::cmp::{Eq, PartialEq};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::Into;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::iter::{IntoIterator, Iterator};
use std::path;

use crate::message::Field;
use crate::quickfix_errors::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use roxmltree::{Document, Node, NodeType};

type FxStr = &'static str;
type DictResult<T> = std::result::Result<T, SessionLevelRejectErr>;

#[derive(Debug, Copy, Clone)]
enum FixType {
    CHAR,
    BOOLEAN,
    DATA,
    FLOAT,
    AMT,
    PERCENTAGE,
    PRICE,
    PRICEOFFSET,
    QTY,
    INT,
    LENGTH,
    NUMINGROUP,
    SEQNUM,
    TAGNUM,
    STRING,
    COUNTRY,
    CURRENCY,
    EXCHANGE,
    LOCALMKTDATE,
    MONTHYEAR,
    MULTIPLEVALUESTRING,
    UTCDATE,
    UTCTIMEONLY,
    UTCTIMESTAMP,
}

impl FixType {
    fn value_of(value: &str) -> Self {
        match value {
            "CHAR" => FixType::CHAR,
            "BOOLEAN" => FixType::BOOLEAN,
            "DATA" => FixType::DATA,
            "FLOAT" => FixType::FLOAT,
            "AMT" => FixType::AMT,
            "PERCENTAGE" => FixType::PERCENTAGE,
            "PRICE" => FixType::PRICE,
            "PRICEOFFSET" => FixType::PRICEOFFSET,
            "QTY" => FixType::QTY,
            "INT" => FixType::INT,
            "LENGTH" => FixType::LENGTH,
            "NUMINGROUP" => FixType::NUMINGROUP,
            "SEQNUM" => FixType::SEQNUM,
            "TAGNUM" => FixType::TAGNUM,
            "STRING" => FixType::STRING,
            "COUNTRY" => FixType::COUNTRY,
            "CURRENCY" => FixType::CURRENCY,
            "EXCHANGE" => FixType::EXCHANGE,
            "LOCALMKTDATE" => FixType::LOCALMKTDATE,
            "MONTHYEAR" => FixType::MONTHYEAR,
            "MULTIPLEVALUESTRING" => FixType::MULTIPLEVALUESTRING,
            "UTCDATE" => FixType::UTCDATE,
            "UTCTIMEONLY" => FixType::UTCTIMEONLY,
            "UTCTIMESTAMP" => FixType::UTCTIMESTAMP,
            _ => panic!("Unknown Fix Type..aborting"),
        }
    }
}

#[derive(Debug)]
pub struct DataDictionary {
    begin_string: String,
    header: Header,
    trailer: Trailer,
    fields_by_tag: HashMap<u32, FieldEntry>,
    fields_by_name: HashMap<String, u32>,
    groups: HashMap<String, Group>,
    messages: HashMap<String, Message>,
}

impl DataDictionary {
    fn new() -> Self {
        Self {
            begin_string: String::new(),
            header: Header::new(),
            trailer: Trailer::new(),
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new(),
            groups: HashMap::new(),
            messages: HashMap::new(),
        }
    }

    pub fn with_xml(xml_file: &str) -> Self {
        create_data_dict(xml_file)
    }

    fn get_field_type(&self, field: &Field) -> FixType {
        let field_entry = self.fields_by_tag.get(&field.get_tag()).unwrap();
        return field_entry.field_type;
    }

    fn check_valid_tag(&self, field: &Field) -> DictResult<&FieldEntry> {
        // checks if the tag is valid according to data dictionary
        let tag = field.get_tag();
        self.fields_by_tag
            .get(&tag)
            .ok_or_else(|| SessionLevelRejectErr::invalid_tag_err())
    }

    fn check_msg_type(&self, msg_type: &str) -> DictResult<&Message> {
        // checks that message type is valid according to data dictionary
        self.messages
            .get(msg_type)
            .ok_or_else(|| SessionLevelRejectErr::invalid_msg_type_err())
    }

    pub fn check_tag_for_message(
        &self,
        tag: u32,
        msg_type: &str,
    ) -> Result<(), SessionLevelRejectErr> {
        let message = match self.check_msg_type(msg_type) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        match message.fields.get(&tag) {
            Some(_) => return Ok(()),
            None => {
                for (grp_name, _) in message.groups.iter() {
                    if check_tag_in_group(tag, grp_name, self) {
                        return Ok(());
                    }
                }
            }
        }
        Err(SessionLevelRejectErr::undefined_tag_err())
    }

    pub fn check_tag_valid_value(&self, field: &Field) -> Result<(), SessionLevelRejectErr> {
        // returns true or false based on valid/invalid value
        let value = field.get_str().unwrap();
        if value.is_empty() {
            return Err(SessionLevelRejectErr::tag_without_value_err());
        }

        let field_entry = match self.check_valid_tag(field) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        if field_entry.field_values.is_empty() {
            // empty field values means its free value tag
            return self.check_tag_format(field);
        } else if field_entry.field_values.get(&value).is_none() {
            // no need to check format here. it will result in error regardless
            return Err(SessionLevelRejectErr::value_out_of_range_err());
        }
        Ok(())
    }

    fn check_has_required(&self, msg: &crate::message::Message) -> DictResult<()> {
        todo!()
    }

    fn check_tag_format(&self, field: &Field) -> DictResult<()> {
        let expected_format = self.get_field_type(field);
        let result = match expected_format {
            FixType::FLOAT
            | FixType::AMT
            | FixType::PERCENTAGE
            | FixType::PRICEOFFSET
            | FixType::PRICE
            | FixType::QTY => field.get_float().map(|_| ()),
            FixType::INT
            | FixType::LENGTH
            | FixType::NUMINGROUP
            | FixType::SEQNUM
            | FixType::TAGNUM => field.get_int().map(|_| ()),
            FixType::CHAR => field.get_char().map(|_| ()),
            FixType::BOOLEAN => field.get_bool().map(|_| ()),
            FixType::DATA
            | FixType::STRING
            | FixType::CURRENCY
            | FixType::COUNTRY
            | FixType::EXCHANGE
            | FixType::MULTIPLEVALUESTRING => field.get_str().map(|_| ()),
            FixType::LOCALMKTDATE | FixType::UTCDATE => {
                // todo!("implement time related parsing using other crates")
                match field.get_str() {
                    Ok(s) => {
                        if NaiveDate::parse_from_str(&s, "%Y%m%d").is_err() {
                            return Err(SessionLevelRejectErr::incorrect_data_format_err());
                        }
                    }
                    Err(_) => return Err(SessionLevelRejectErr::incorrect_data_format_err()),
                }
                Ok(())
            }
            FixType::MONTHYEAR => {
                match field.get_str() {
                    Ok(s) => {
                        if NaiveDate::parse_from_str(&s, "%Y%m").is_err() {
                            return Err(SessionLevelRejectErr::incorrect_data_format_err());
                        }
                    }
                    Err(_) => return Err(SessionLevelRejectErr::incorrect_data_format_err()),
                }
                Ok(())
            }
            FixType::UTCTIMEONLY => {
                match field.get_str() {
                    Ok(s) => {
                        if NaiveTime::parse_from_str(&s, "%T%.3f").is_err() {
                            return Err(SessionLevelRejectErr::incorrect_data_format_err());
                        }
                    }
                    Err(_) => return Err(SessionLevelRejectErr::incorrect_data_format_err()),
                }
                Ok(())
            }
            FixType::UTCTIMESTAMP => {
                match field.get_str() {
                    Ok(s) => {
                        if NaiveDateTime::parse_from_str(&s, "%Y%m%d-%T%.3f").is_err() {
                            return Err(SessionLevelRejectErr::incorrect_data_format_err());
                        }
                    }
                    Err(_) => return Err(SessionLevelRejectErr::incorrect_data_format_err()),
                }
                Ok(())
            }
        };
        return result;
    }
}

#[derive(Debug)]
struct FieldEntry {
    field_number: u32,
    field_name: String,
    field_type: FixType,
    field_values: BTreeMap<String, FieldValueEntry>,
}

impl FieldEntry {
    fn new(field_number: u32, field_name: &str, field_type: &str) -> Self {
        let ftype = FixType::value_of(field_type);
        Self {
            field_number,
            field_name: field_name.to_owned(),
            field_type: ftype,
            field_values: BTreeMap::new(),
        }
    }

    fn get_field_name(&self) -> &str {
        &self.field_name.as_str()
    }
}

#[derive(Debug)]
struct FieldValueEntry {
    value: String,
    description: String,
}

impl FieldValueEntry {
    fn new(value: &str, desc: &str) -> Self {
        Self {
            value: value.to_owned(),
            description: desc.to_owned(),
        }
    }

    fn get_field_value(&self) -> &str {
        self.value.as_str()
    }
}

trait FieldAndGroupSetter {
    fn add_required_field(&mut self, field: u32, required: bool);
    fn add_required_group(&mut self, group_name: &str, required: bool);
    fn add_delim(&mut self, delim: u32);
}

#[derive(Debug)]
struct Group {
    group_name: String,
    group_delim: u32,
    group_fields: HashMap<u32, bool>, // field number to required mapping
    sub_groups: HashMap<String, bool>, // (sub)group to required mapping
}

impl Group {
    fn new(name: &str) -> Self {
        Self {
            group_name: name.to_owned(),
            group_delim: 0,
            group_fields: HashMap::new(),
            sub_groups: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Header {
    header_field: HashMap<u32, bool>,
    header_groups: HashMap<String, bool>,
}

impl Header {
    fn new() -> Self {
        Self {
            header_field: HashMap::new(),
            header_groups: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Trailer {
    trailer_field: HashMap<u32, bool>,
}

impl Trailer {
    fn new() -> Self {
        Self {
            trailer_field: HashMap::new(),
        }
    }
}

fn check_tag_in_group(tag: u32, group_name: &str, dict: &DataDictionary) -> bool {
    // check if the field is supported by the group or not
    let group = match dict.groups.get(group_name) {
        Some(g) => g,
        None => {
            eprintln!("Group {} not found in dictionary", group_name);
            return false;
        }
    };
    if group.group_fields.contains_key(&tag) {
        return true;
    }
    for (group_name, _) in group.sub_groups.iter() {
        if check_tag_in_group(tag, group_name, dict) {
            return true;
        }
    }
    return false;
}

impl FieldAndGroupSetter for Group {
    fn add_required_field(&mut self, field: u32, required: bool) {
        self.group_fields.insert(field, required);
    }

    fn add_required_group(&mut self, group_name: &str, required: bool) {
        self.sub_groups.insert(group_name.to_string(), required);
    }

    fn add_delim(&mut self, delim: u32) {
        if self.group_delim == 0 {
            self.group_delim = delim;
        }
    }
}

#[derive(Debug)]
struct Message {
    name: String,
    m_type: String,
    category: String,
    fields: HashMap<u32, bool>,    // field tag to required
    groups: HashMap<String, bool>, // group name to required mapping
}

impl Message {
    fn new(msg_name: &str, msg_type: &str, msg_cat: &str) -> Self {
        Self {
            name: msg_name.to_owned(),
            m_type: msg_type.to_owned(),
            category: msg_cat.to_owned(),
            fields: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

impl FieldAndGroupSetter for Message {
    fn add_required_field(&mut self, field: u32, required: bool) {
        self.fields.insert(field, required);
    }

    fn add_required_group(&mut self, group_name: &str, required: bool) {
        self.groups.insert(group_name.to_string(), required);
    }

    fn add_delim(&mut self, delim: u32) {}
}

fn update_fields(field_node: Node, dict: &mut DataDictionary) {
    for node in field_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let fname = node.attribute("name").unwrap();
        let fnum = node.attribute("number").unwrap().parse::<u32>().unwrap();
        let ftype = node.attribute("type").unwrap();
        let mut f_entry = FieldEntry::new(fnum, fname, ftype);
        for child in node
            .children()
            .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("value"))
        {
            let valid_value = child.attribute("enum").unwrap();
            let fvalue_entry =
                FieldValueEntry::new(valid_value, child.attribute("description").unwrap());
            f_entry
                .field_values
                .insert(valid_value.to_string(), fvalue_entry);
        }
        dict.fields_by_tag.insert(fnum, f_entry);
        dict.fields_by_name.insert(fname.to_string(), fnum);
    }
}

fn add_component<T: FieldAndGroupSetter>(
    comp_node: &Node,
    comp_node_req: bool,
    field_map: &mut T,
    dict: &mut DataDictionary,
) {
    for child_node in comp_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        match child_node.tag_name().name() {
            "field" => {
                // field details are in struct.
                let field_tag = get_field_tag(child_node, dict);
                let required = get_required_attribute(child_node);
                // if comp is required and components field is required then add as required
                // otherwise optional
                field_map.add_required_field(field_tag, required && comp_node_req);
                field_map.add_delim(field_tag)
            }
            "group" => {
                let grp_name = get_name_attribute(child_node);
                let grp_req = get_required_attribute(child_node);
                let group_field_tag = dict.fields_by_name.get(grp_name).unwrap();
                field_map.add_required_field(*group_field_tag, grp_req && comp_node_req);
                field_map.add_required_group(grp_name, grp_req && comp_node_req);
                if dict.groups.get(grp_name).is_none() {
                    let sub_group = add_group(child_node, dict);
                    dict.groups.insert(grp_name.to_string(), sub_group);
                }
            }
            _ => panic!("Invalid xml tag in component"),
        }
    }
}

fn add_group(group_node: Node, dict: &mut DataDictionary) -> Group {
    // create group, subgroup and components entry and return
    let group_name = group_node.attribute("name").unwrap();
    let mut group = Group::new(group_name);
    for child_node in group_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        match child_node.tag_name().name() {
            "field" => {
                let field_tag = get_field_tag(child_node, dict);
                let required = get_required_attribute(child_node);
                group.group_fields.insert(field_tag, required);
                if group.group_delim == 0 {
                    group.group_delim = field_tag;
                }
            }
            "component" => {
                let required = get_required_attribute(child_node);
                add_component(&child_node, required, &mut group, dict);
            }
            "group" => {
                let sub_group_name = get_name_attribute(child_node);
                let required = get_required_attribute(child_node);
                group
                    .sub_groups
                    .insert(sub_group_name.to_string(), required);
                if dict.groups.get(sub_group_name).is_none() {
                    let sub_group = add_group(child_node, dict);
                    dict.groups.insert(sub_group_name.to_string(), sub_group);
                }
            }
            _ => panic!("Unknown xml tag in groups. Aborting."),
        }
    }
    group
}

fn get_name_attribute<'a>(node: Node<'a, '_>) -> &'a str {
    node.attribute("name").unwrap()
}

fn get_required_attribute(node: Node) -> bool {
    node.attribute("required")
        .map(|req| req.eq_ignore_ascii_case("y"))
        .unwrap()
}

fn get_field_tag(node: Node, dict: &DataDictionary) -> u32 {
    let field_tag = node
        .attribute("name")
        .and_then(|n| dict.fields_by_name.get(n))
        .unwrap();
    *field_tag
}

fn add_messages(node: Node, dict: &mut DataDictionary, components: &HashMap<&str, Node>) {
    for msg_node in node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let msg_name = msg_node.attribute("name").unwrap();
        let msg_type = msg_node.attribute("msgtype").unwrap();
        let msg_cat = msg_node.attribute("msgcat").unwrap();
        let mut message = Message::new(msg_name, msg_type, msg_cat);
        for msg_child_node in msg_node
            .children()
            .filter(|n| n.node_type() == NodeType::Element)
        {
            match msg_child_node.tag_name().name() {
                "field" => {
                    let field_tag = get_field_tag(msg_child_node, dict);
                    let required = get_required_attribute(msg_child_node);
                    message.fields.insert(field_tag, required);
                }
                "group" => {
                    let grp_name = get_name_attribute(msg_child_node);
                    let required = get_required_attribute(msg_child_node);
                    message.groups.insert(grp_name.to_string(), required);
                    if dict.groups.get(grp_name).is_none() {
                        let msg_group = add_group(msg_child_node, dict);
                        dict.groups.insert(grp_name.to_string(), msg_group);
                    }
                }
                "component" => {
                    let cname = get_name_attribute(msg_child_node);
                    let required = get_required_attribute(msg_child_node);
                    let component = components.get(cname).unwrap();
                    // message.components.insert(cname.to_string(), required);
                    add_component(component, required, &mut message, dict);
                }
                _ => panic!("Unknown xml tag in message section. Aborting."),
            }
        }
        dict.messages.insert(msg_type.to_string(), message);
    }
}

fn add_header(header_node: Node, dict: &mut DataDictionary) {
    for node in header_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        match node.tag_name().name() {
            "field" => {
                let field_tag = get_field_tag(node, dict);
                let required = get_required_attribute(node);
                dict.header.header_field.insert(field_tag, required);
            }
            "group" => {
                let grp_name = get_name_attribute(node);
                let required = get_required_attribute(node);
                dict.header
                    .header_groups
                    .insert(grp_name.to_string(), required);
                if dict.groups.get(grp_name).is_none() {
                    let hdr_group = add_group(node, dict);
                    dict.groups.insert(grp_name.to_string(), hdr_group);
                }
            }
            _ => panic!("Unknown xml tag in header section. Aborting."),
        }
    }
}

fn add_trailer(trailer_node: Node, dict: &mut DataDictionary) {
    for node in trailer_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        match node.tag_name().name() {
            "field" => {
                let field_tag = get_field_tag(node, dict);
                let required = get_required_attribute(node);
                dict.trailer.trailer_field.insert(field_tag, required);
            }
            _ => panic!("Unknown xml tag in header section. Aborting."),
        }
    }
}

pub fn create_data_dict(fix_xml: &str) -> DataDictionary {
    let mut file_data = String::with_capacity(1024 * 64);
    let mut file = File::open(fix_xml).unwrap();
    file.read_to_string(&mut file_data).unwrap();
    let doc = Document::parse(&file_data).unwrap();
    let mut data_dict = DataDictionary::new();
    let dict_type = doc.root_element().attribute("type").unwrap();
    let major_version = doc.root_element().attribute("major").unwrap();
    let minor_verion = doc.root_element().attribute("minor").unwrap();
    let begin_string = format!("{}.{}.{}", dict_type, major_version, minor_verion);
    data_dict.begin_string = begin_string;

    let field_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "fields")
        .unwrap();
    update_fields(field_node, &mut data_dict);

    let component_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "components")
        .unwrap();
    let mut component_map: HashMap<&str, Node> = HashMap::new();
    for cnode in component_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let cmp_name = get_name_attribute(cnode);
        component_map.insert(cmp_name, cnode);
    }

    let header_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "header")
        .unwrap();
    add_header(header_node, &mut data_dict);

    let trailer_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "trailer")
        .unwrap();
    add_trailer(trailer_node, &mut data_dict);

    let message_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "messages")
        .unwrap();
    add_messages(message_node, &mut data_dict, &component_map);
    data_dict
}

#[cfg(test)]
mod dict_test {
    use super::*;

    #[test]
    fn test_it_works() {
        let data_dict = create_data_dict("src/fix43/FIX43.xml");
        // println!("\ndictionary fields by name: {:?}", data_dict.fields_by_name);
        // println!("\ndictionary fields by tag: {:?}", data_dict.fields_by_tag);
        // println!("\ndictionary components: {:?}", data_dict.components);
        // println!("\ndictionary groups: {:?}", data_dict.groups);
        // println!("\ndictionary messages {:?}", data_dict.messages);
        println!("\ndictionary header {:?}", data_dict.header);
        println!("\ndictionary begin string {}", data_dict.begin_string);
        println!("\ndictionary trailer {:?}", data_dict.trailer);
    }

    fn test_date_time_fields() {}
}
