use std::fs::File;
use std::io::Read;
use std::path;
use std::collections::HashMap;
use std::collections::HashSet;

use roxmltree::{Document, NodeType};

#[derive(Debug)]
pub struct DataDictionary {
    fields: HashMap<u32, FieldEntry>,
}

impl DataDictionary {
    fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }
}

#[derive(Debug)]
struct FieldEntry {
    number: u32,
    name: String,
    ftype: String,
    values: HashSet<FieldValueEntry>,
}

impl FieldEntry {
    fn new(number: u32, name: String, ftype: String) -> Self {
        Self {
            number,
            name,
            ftype,
            values: HashSet::new()
        }
    }

    fn set_valid_value(&mut self, val: FieldValueEntry) {
        self.values.insert(val);
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct FieldValueEntry {
    value: String,
    description: String
}

pub fn create_data_dict(fix_xml: &str) -> Option<DataDictionary> {
    let mut file_data = String::with_capacity(1024*64);
    let mut file = File::open(fix_xml).unwrap();
    file.read_to_string(&mut file_data).unwrap();
    let doc = Document::parse(&file_data).unwrap();
    for d in doc.root_element().children().filter(|node| node.node_type() != NodeType::Text) {
        if d.has_tag_name("fields") {
            for fld in d.children().filter(|node| node.node_type() != NodeType::Text) {
                println!("field node {:?}", fld);
            }
        }
        // println!("Node {:?}", d);
    }
    Some(DataDictionary::new())
}


