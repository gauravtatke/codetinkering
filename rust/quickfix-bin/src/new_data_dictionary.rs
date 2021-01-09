use std::cmp::{Eq, PartialEq};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::Into;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path;

use crate::message::Field;
use crate::quickfix_errors::*;
use roxmltree::{Document, Node, NodeType};

type FxStr = &'static str;

lazy_static! (
    // statically load all fix types
    static ref FIXTYPES: HashSet<FxStr> = [
            "CHAR",
            "BOOLEAN",
            "DATA",
            "FLOAT",
            "AMT",
            "PERCENTAGE",
            "PRICE",
            "PRICEOFFSET",
            "QTY",
            "INT",
            "LENGTH",
            "NUMINGROUP",
            "SEQNUM",
            "TAGNUM",
            "STRING",
            "COUNTRY",
            "CURRENCY",
            "EXCHANGE",
            "LOCALMKTDATE",
            "MONTHYEAR",
            "MULTIPLEVALUESTRING",
            "UTCDATE",
            "UTCTIMEONLY",
            "UTCTIMESTAMP",
        ]
        .iter()
        .cloned()
        .collect();
);

#[derive(Debug)]
pub struct DataDict {
    fields_by_tag: HashMap<u32, FieldEntry>,
    fields_by_name: HashMap<String, u32>,
    components: HashMap<String, Component>,
    groups: HashMap<String, Group>,
    messages: HashMap<String, Message>,
}

impl DataDict {
    pub fn new() -> Self {
        Self {
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new(),
            components: HashMap::new(),
            groups: HashMap::new(),
            messages: HashMap::new(),
        }
    }

    pub fn is_tag_valid_for_message(&self, tag: u32, msg_type: &str) -> Result<(), NewFixError> {
        Ok(())
    }

    pub fn is_tag_value_valid(&self, tag: u32, val: &str) -> Result<(), NewFixError> {
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
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct FieldEntry {
    field_number: u32,
    field_name: String,
    field_type: FxStr,
    field_values: BTreeMap<String, FieldValueEntry>,
}

impl FieldEntry {
    fn new(field_number: u32, field_name: &str, field_type: &str) -> Self {
        let ftype: FxStr = *FIXTYPES.get(field_type).unwrap();
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

#[derive(Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug)]
struct Component {
    component_name: String,
    component_fields: HashMap<u32, bool>, // field number to required mapping
    component_group: HashMap<String, bool>, // component group to required mapping
}

impl Component {
    fn new(name: &str) -> Self {
        Component {
            component_name: name.to_owned(),
            component_fields: HashMap::new(),
            component_group: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Group {
    group_name: String,
    group_fields: HashMap<u32, bool>, // field number to required mapping
    group_components: HashMap<String, bool>, // component name to required mapping
    sub_groups: HashMap<String, bool>, // (sub)group to required mapping
}

impl Group {
    fn new(name: &str) -> Self {
        Self {
            group_name: name.to_owned(),
            group_fields: HashMap::new(),
            group_components: HashMap::new(),
            sub_groups: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Message {
    name: String,
    m_type: String,
    category: String,
    fields: HashMap<u32, bool>,        // field tag to required
    groups: HashMap<String, bool>,     // group name to required mapping
    components: HashMap<String, bool>, // component to required mapping
}

impl Message {
    fn new(msg_name: &str, msg_type: &str, msg_cat: &str) -> Self {
        Self {
            name: msg_name.to_owned(),
            m_type: msg_type.to_owned(),
            category: msg_cat.to_owned(),
            fields: HashMap::new(),
            groups: HashMap::new(),
            components: HashMap::new(),
        }
    }
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
            // f_entry.set_valid_value(fvalue_entry);
        }
        dict.fields_by_tag.insert(fnum, f_entry);
        dict.fields_by_name.insert(fname.to_string(), fnum);
    }
}

fn create_group(group_node: Node, dict: &mut DataDict) -> Group {
    // create group, subgroup and components entry and return
    let group_name = group_node.attribute("name").unwrap();
    let mut group = Group::new(group_name);
    for child_node in group_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        match child_node.tag_name().name() {
            "field" => {
                let field_tag = child_node
                    .attribute("name")
                    .and_then(|n| dict.fields_by_name.get(n))
                    .unwrap();
                let required = child_node
                    .attribute("required")
                    .map(|req| req.eq_ignore_ascii_case("y"))
                    .unwrap();
                group.group_fields.insert(*field_tag, required);
            }
            "component" => {
                let component_name = child_node.attribute("name").unwrap();
                let required = child_node
                    .attribute("required")
                    .map(|req| req.eq_ignore_ascii_case("y"))
                    .unwrap();
                group
                    .group_components
                    .insert(component_name.to_string(), required);
            }
            "group" => {
                let sub_group_name = child_node.attribute("name").unwrap();
                let required = child_node
                    .attribute("required")
                    .map(|req| req.eq_ignore_ascii_case("y"))
                    .unwrap();
                group
                    .sub_groups
                    .insert(sub_group_name.to_string(), required);
                if dict.groups.get(sub_group_name).is_none() {
                    let sub_group = create_group(child_node, dict);
                    dict.groups.insert(sub_group_name.to_string(), sub_group);
                }
            }
            _ => panic!("Unknown xml tag in groups. Aborting."),
        }
    }
    group
}

fn component_handler(component_node: Node, dict: &mut DataDict) {
    for node in component_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let component_name = node.attribute("name").unwrap();
        let mut component_entry = Component::new(component_name);
        for field_group in node
            .children()
            .filter(|n| n.node_type() == NodeType::Element)
        {
            match field_group.tag_name().name() {
                "field" => {
                    let field_tag = field_group
                        .attribute("name")
                        .and_then(|n| dict.fields_by_name.get(n))
                        .unwrap();
                    let required = field_group
                        .attribute("required")
                        .map(|req| req.eq_ignore_ascii_case("y"))
                        .unwrap();
                    component_entry
                        .component_fields
                        .insert(*field_tag, required);
                }
                "group" => {
                    let grp_name = field_group.attribute("name").unwrap();
                    let grp_req = field_group
                        .attribute("required")
                        .map(|req| req.eq_ignore_ascii_case("y"))
                        .unwrap();
                    component_entry
                        .component_group
                        .insert(grp_name.to_string(), grp_req);
                    if dict.groups.get(grp_name).is_none() {
                        let group = create_group(field_group, dict);
                        dict.groups.insert(grp_name.to_string(), group);
                    }
                }
                _ => panic!("Unknown tag in xml. Could not process. Aborting."),
            }
        }
        dict.components
            .insert(component_name.to_string(), component_entry);
    }
}

fn message_handler(node: Node, dict: &mut DataDict) {
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
                    let field_tag = msg_child_node
                        .attribute("name")
                        .and_then(|n| dict.fields_by_name.get(n))
                        .unwrap();
                    let required = msg_child_node
                        .attribute("required")
                        .map(|req| req.eq_ignore_ascii_case("y"))
                        .unwrap();
                    message.fields.insert(*field_tag, required);
                }
                "group" => {
                    let grp_name = msg_child_node.attribute("name").unwrap();
                    let required = msg_child_node
                        .attribute("required")
                        .map(|req| req.eq_ignore_ascii_case("y"))
                        .unwrap();
                    message.groups.insert(grp_name.to_string(), required);
                    if dict.groups.get(grp_name).is_none() {
                        let msg_group = create_group(msg_child_node, dict);
                        dict.groups.insert(grp_name.to_string(), msg_group);
                    }
                }
                "component" => {
                    let cname = msg_child_node.attribute("name").unwrap();
                    let required = msg_child_node
                        .attribute("name")
                        .map(|req| req.eq_ignore_ascii_case("y"))
                        .unwrap();
                    message.components.insert(cname.to_string(), required);
                }
                _ => panic!("Unknown xml tag in message section. Aborting."),
            }
        }
        dict.messages.insert(msg_name.to_string(), message);
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
    component_handler(component_node, &mut data_dict);

    let message_node = doc
        .root_element()
        .children()
        .find(|node| node.tag_name().name() == "messages")
        .unwrap();
    message_handler(message_node, &mut data_dict);
    data_dict
}

#[cfg(test)]
mod dict_test {
    use super::*;

    #[test]
    fn test_it_works() {
        let data_dict = create_data_dict("src/fix43/FIX43.xml");
        println!("dictionary fields: {:?}", data_dict);
    }
}
