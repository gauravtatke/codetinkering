use std::fs::File;
use std::io::{Read, Write};
use std::path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use roxmltree::{Document, Node, NodeType};

#[derive(Debug)]
pub struct DataDictionary<'a> {
    fields_by_tag: HashMap<u32, FieldEntry<'a>>,
    tag_name_map: HashMap<&'a str, u32>,
    components: HashMap<String, ComponentEntry<'a>>,
    groups: HashMap<String, Group<'a>>,
}

impl<'a> DataDictionary<'a> {
    fn new() -> Self {
        Self {
            fields_by_tag: HashMap::new(),
            tag_name_map: HashMap::new(),
            components: HashMap::new(),
            groups: HashMap::new()
        }
    }

    fn add_field(&mut self, fld: FieldEntry<'static>) {
        // TODO: add check so that tag should not be present already - duplicate tag, invalid xml
        self.tag_name_map.insert(fld.name, fld.number);
        self.fields_by_tag.insert(fld.number, fld);
    }

    fn get_field(&self, tag: &u32) -> Option<&FieldEntry> {
        self.fields_by_tag.get(tag)
    }

    fn get_field_tag(&self, name: &str) -> Option<&u32> {
        self.tag_name_map.get(name)
    }
}

#[derive(Debug)]
struct FieldEntry<'a> {
    number: u32,
    name: &'a str,
    ftype: &'a str,
    values: HashSet<FieldValueEntry<'a>>,
}

impl<'a> FieldEntry<'a> {
    fn new(number: u32, name: &'a str, ftype: &'a str) -> Self {
        Self {
            number,
            name,
            ftype,
            values: HashSet::new()
        }
    }

    fn set_valid_value(&mut self, val: FieldValueEntry<'a>) {
        self.values.insert(val);
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct FieldValueEntry<'a> {
    value: &'a str,
    description: &'a str
}

impl<'a> FieldValueEntry<'a> {
    fn new(val: &'a str, desc: &'a str) -> Self {
        Self {
            value: val,
            description: desc,
        }
    }
}

#[derive(Debug)]
struct ComponentEntry<'a> {
    comp_name: &'a str,
    comp_groups: HashMap<&'a str, bool>,
    comp_fields: HashMap<&'a str, bool>,
}

impl<'a> ComponentEntry<'a> {
    fn new(cname: &'a str) -> Self {
        Self {
            comp_name: cname,
            comp_groups: HashMap::new(),
            comp_fields: HashMap::new()
        }
    }

    fn add_group(&mut self, gname: &'a str, req: char) {
        let mut required = false;
        if req == 'Y' || req == 'y' {
            required = true;
        }
        self.comp_groups.insert(gname, required);
    }

    fn add_field(&mut self, fname: &'a str, req: char) {
        let mut required = false;
        if req == 'Y' || req == 'y' {
            required = true;
        }
        self.comp_fields.insert(fname, required);
    }
}

#[derive(Debug)]
struct Group<'a> {
    group_name: &'a str,

    // name of field and required
    group_fields: HashMap<&'a str, bool>,
}

impl<'a> Group<'a> {
    fn new(gname: &'a str) -> Self {
        Self {
            group_name: gname,
            group_fields: HashMap::new(),
        }
    }

    fn add_field(&mut self, fname: &'a str, req: char) {
        let mut required = false;
        if req == 'Y' || req == 'y' {
            required = true;
        }
        self.group_fields.insert(fname, required);
    }
}

pub fn create_data_dict(fix_xml: &str) -> Option<DataDictionary> {
    let mut file_data = String::with_capacity(1024*64);
    let mut file = File::open(fix_xml).unwrap();
    file.read_to_string(&mut file_data).unwrap();
    let doc = Document::parse(&file_data).unwrap();
    let mut dictionary = DataDictionary::new();
    for root_child in doc.root_element().children().filter(|node| node.node_type() == NodeType::Element) {
        match root_child.tag_name().name() {
            "fields" => {
                field_handler(root_child, &mut dictionary);
            },
            "components" => {
                component_handler(root_child, &mut dictionary);
            },
            _ => {
                println!("No processing this");
            }
        }
    }
    Some(dictionary)
}

fn field_handler(field_node: Node, dict: &mut DataDictionary) {
    for node in field_node.children().filter(|n| n.node_type() == NodeType::Element) {
        let mut f_entry = FieldEntry::new(
            node.attribute("number").unwrap().parse::<u32>().unwrap(),
            node.attribute("name").unwrap(),
            node.attribute("type").unwrap(),
        );
        for child in node.children().filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("value")) {
            let fvalue_entry = FieldValueEntry::new(
                child.attribute("enum").unwrap(),
                child.attribute("description").unwrap()
            );
            f_entry.set_valid_value(fvalue_entry);
        }
        dict.add_field(f_entry);
    }
}

fn component_handler(comp_node: Node, dict: &mut DataDictionary) {

}
