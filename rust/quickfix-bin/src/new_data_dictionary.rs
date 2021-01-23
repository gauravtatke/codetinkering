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
use roxmltree::{Document, Node, NodeType};
use std::ops::Add;

type FxStr = &'static str;
type DictResult<T> = std::result::Result<T, NewFixError>;

#[derive(Debug)]
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
pub struct DataDict {
    fields_by_tag: HashMap<u32, FieldEntry>,
    fields_by_name: HashMap<String, u32>,
    groups: HashMap<String, Group>,
    messages: HashMap<String, Message>,
}

impl DataDict {
    pub fn new() -> Self {
        Self {
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new(),
            groups: HashMap::new(),
            messages: HashMap::new(),
        }
    }

    fn check_valid_tag(&self, field: &Field) -> DictResult<()> {
        todo!()
    }

    pub fn check_tag_for_message(&self, tag: u32, msg_type: &str) -> Result<(), NewFixError> {
        let message = match self.messages.get(msg_type) {
            Some(m) => m,
            None => {
                return Err(NewFixError {
                    kind: NewFixErrorKind::InvalidMessageType,
                })
            }
        };
        for (field_tag, _) in message.fields.iter() {
            if tag == *field_tag {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn check_tag_valid_value(&self, tag: u32, val: &str) -> Result<(), NewFixError> {
        // returns true or false based on valid/invalid value
        if val.is_empty() {
            return Err(NewFixError {
                kind: NewFixErrorKind::TagSpecifiedWithoutValue,
            });
        }

        let field_entry = match self.fields_by_tag.get(&tag) {
            // if there is not field entry then its an error
            Some(f) => f,
            None => {
                return Err(NewFixError {
                    kind: NewFixErrorKind::UndefinedTag,
                })
            }
        };

        if field_entry.field_values.is_empty() {
            // empty field values means its free value tag
            // TODO: verify that value can be parsed into required type
            return Ok(());
        } else if field_entry.field_values.get(val).is_none() {
            return Err(NewFixError {
                kind: NewFixErrorKind::ValueOutOfRange,
            });
        }
        Ok(())
    }

    fn check_has_required(&self, msg: &crate::message::Message) -> DictResult<()> {
        todo!()
    }

    fn check_tag_without_value(&self, field: &Field) -> DictResult<()> {
        todo!()
    }

    fn check_tag_format(&self, field: &Field) -> DictResult<()> {
        todo!()
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
        // let ftype: FxStr = *FIXTYPES.get(field_type).unwrap();
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

trait AddFieldAndGroup {
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

impl AddFieldAndGroup for Group {
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

impl AddFieldAndGroup for Message {
    fn add_required_field(&mut self, field: u32, required: bool) {
        self.fields.insert(field, required);
    }

    fn add_required_group(&mut self, group_name: &str, required: bool) {
        self.groups.insert(group_name.to_string(), required);
    }

    fn add_delim(&mut self, delim: u32) {}
}

fn update_fields(field_node: Node, dict: &mut DataDict) {
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

fn add_component<T: AddFieldAndGroup>(
    comp_node: &Node,
    comp_node_req: bool,
    field_map: &mut T,
    dict: &mut DataDict,
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

fn add_group(group_node: Node, dict: &mut DataDict) -> Group {
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

fn get_field_tag(node: Node, dict: &DataDict) -> u32 {
    let field_tag = node
        .attribute("name")
        .and_then(|n| dict.fields_by_name.get(n))
        .unwrap();
    *field_tag
}

fn message_handler(node: Node, dict: &mut DataDict, components: &HashMap<&str, Node>) {
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

pub fn create_data_dict(fix_xml: &str) -> DataDict {
    let mut file_data = String::with_capacity(1024 * 64);
    let mut file = File::open(fix_xml).unwrap();
    file.read_to_string(&mut file_data).unwrap();
    let doc = Document::parse(&file_data).unwrap();
    let mut data_dict = DataDict::new();

    let field_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "fields")
        .unwrap();
    update_fields(field_node, &mut data_dict);
    // println!("dictionary fields: {:?}", data_dict.fields);

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

    // component_handler(component_node, &mut data_dict);

    let message_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "messages")
        .unwrap();
    message_handler(message_node, &mut data_dict, &component_map);
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
        println!("\ndictionary messages {:?}", data_dict.messages);
    }
}
