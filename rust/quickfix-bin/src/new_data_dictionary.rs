use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path;
use std::cmp::{PartialEq, Eq};
use std::convert::Into;
use std::hash::{Hash, Hasher};

use roxmltree::{Document, Node, NodeType};
use crate::message::Field;
use crate::quickfix_errors::*;

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
    components: HashMap<String, ComponentEntry>,
}

impl DataDict {
    pub fn new() -> Self {
        Self {
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new()
        }
    }

    pub fn is_tag_valid_for_message(&self, tag: u32, msg_type: &str) -> Result<(), NewFixError> {
        Ok(())
    }

    pub fn is_tag_value_valid(&self, tag: u32, val: &str) -> Result<(), NewFixError> {
        // returns true or false based on valid/invalid value
        if val.is_empty() {
            return Err(NewFixError {
                kind: NewFixErrorKind::TagSpecifiedWithoutValue
            })
        }

        let field_entry = match self.fields_by_tag.get(&tag) {
            // if there is not field entry then its an error
            Some(f) => f,
            None => return Err(NewFixError {
                kind: NewFixErrorKind::UndefinedTag,
            })
        };

        if field_entry.field_values.is_empty() {
            // empty field values means its free value tag
            // TODO: verify that value can be parsed into required type
            return Ok(());
        } else if field_entry.field_values.get(val).is_none() {
            return Err(NewFixError {
                kind: NewFixErrorKind::ValueOutOfRange,
            })
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

    // fn set_valid_values(&mut self, value: FieldValueEntry) {
    //     self.field_values.insert(value);
    // }
}

// impl Hash for FieldEntry {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.field_name.hash(state);
//     }
// }
//
// impl PartialEq<u32> for FieldEntry {
//     fn eq(&self, other: &u32) -> bool {
//         self.field_number == *other
//     }
// }



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
struct ComponentEntry {
    component_name: String,
    component_fields: HashMap<u32, bool>, // field number to required mapping
    component_group: HashMap<String, bool> // component group to required mapping

}

impl ComponentEntry {
    fn new(name: &str) -> Self {
        Self {
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

// impl PartialEq<FxStr> for FieldValueEntry {
//     // impl PartialEq so that any &str can be compared to FieldValueEntry
//     fn eq(&self, other: &FxStr) -> bool {
//         self.value.as_str() == *other
//     }
// }
//
// impl Hash for FieldValueEntry {
//     // impl hash so that just &str can be used to get the fieldvalue entry from dict
//     // it is done because from fix message, we will only have &str
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.value.hash(state);
//     }
// }


fn update_fields(field_node: Node, dict: & mut DataDict) {
    for node in field_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let fname = node.attribute("name").unwrap();
        let fnum = node.attribute("number").unwrap().parse::<u32>().unwrap();
        let ftype= node.attribute("type").unwrap();
        let mut f_entry = FieldEntry::new(fnum, fname, ftype);
        for child in node
            .children()
            .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("value"))
        {
            let valid_value = child.attribute("enum").unwrap();
            let fvalue_entry = FieldValueEntry::new(
                valid_value,
                child.attribute("description").unwrap(),
            );
            f_entry.field_values.insert(valid_value.to_string(), fvalue_entry);
            // f_entry.set_valid_value(fvalue_entry);
        }
        dict.fields_by_tag.insert(fnum, f_entry);
        dict.fields_by_name.insert(fname.to_string(), fnum);
    }
}

fn component_handler(comp_node: Node, dict: &mut DataDict) {
    for node in comp_node
        .children()
        .filter(|n| n.node_type() == NodeType::Element)
    {
        let comp_name = node.attribute("name").unwrap();
        let mut cmp_entry = ComponentEntry::new(comp_name);
        for field_group in node
            .children()
            .filter(|n| n.node_type() == NodeType::Element)
        {
            match field_group.tag_name().name() {
                "field" => {
                    let fname = field_group.attribute("name").unwrap();
                    cmp_entry.add_field(fname, field_group.attribute("required").unwrap());
                }
                "group" => {
                    let grp_name = field_group.attribute("name").unwrap();
                    let grp_req = field_group.attribute("required").unwrap();
                    let mut group = Group::new(grp_name);
                    for group_field in field_group
                        .children()
                        .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("field"))
                    {
                        group.add_field(
                            group_field.attribute("name").unwrap(),
                            group_field.attribute("required").unwrap(),
                        );
                    }
                    dict.add_group(grp_name, group);
                    cmp_entry.add_group(grp_name, grp_req);
                }
                _ => {} // Do nothing TODO: Error handling
            }
        }
        dict.add_component(comp_name, cmp_entry);
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
    component_handler(component_node, &mut dictionary);


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