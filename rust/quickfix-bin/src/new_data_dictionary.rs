use std::collections::{HashMap, HashSet};
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

lazy_static!(
    // static values for all boolean yes/no
    static ref BOOLEAN_VALUES: HashSet<FxStr> = ["Y", "YES", "N", "NO"].iter().cloned().collect();
);

// lazy_static!(
//     // static values for all field names
//     static ref FIELD_NAMES: HashSet<String> = HashSet::with_capacity(64);
// );

#[derive(Debug)]
pub struct DataDict {
    fields_by_tag: HashMap<u32, FieldEntry>,
    fields_by_name: HashMap<String, u32>,
}

impl DataDict {
    pub fn new() -> Self {
        Self {
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new()
        }
    }

    pub fn is_tag_value_valid(&self, msg_type: &str, tag: &str, val: &str) -> Result<(), Err> {
        // returns true or false based on valid/invalid value

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct FieldEntry {
    field_number: u32,
    field_name: String,
    field_type: FxStr,
    field_values: Vec<FieldValueEntry>,
}

impl FieldEntry {
    fn new(field_number: u32, field_name: &str, field_type: &str) -> Self {
        let ftype: FxStr = *FIXTYPES.get(field_type).unwrap();
        Self {
            field_number,
            field_name: field_name.to_owned(),
            field_type: ftype,
            field_values: Vec::new(),
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



#[derive(Debug, Eq, PartialEq)]
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

struct component_groups {
    cgroup_name: String,
    cgroup_fields: HashMap<u32, bool>, // field number to required mapping
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
            let fvalue_entry = FieldValueEntry::new(
                child.attribute("enum").unwrap(),
                child.attribute("description").unwrap(),
            );
            f_entry.field_values.push(fvalue_entry);
            // f_entry.set_valid_value(fvalue_entry);
        }
        dict.fields_by_tag.insert(fnum, f_entry);
        dict.fields_by_name.insert(fname.to_string(), fnum);
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