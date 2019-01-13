use std::fs::File;
use std::io::{Read, Write};
use std::path;
use std::collections::{HashMap, HashSet};
use std::fmt;

use roxmltree::{Document, Node, NodeType};

#[derive(Debug)]
pub struct DataDictionary {
    // field_set: HashSet<String>,
    fields_by_tag: HashMap<u32, FieldEntry>,
    tag_name_map: HashMap<String, u32>,
    components: HashMap<String, ComponentEntry>,
    groups: HashMap<String, Group>,
    messages: HashMap<String, MessageEntry>
}

impl DataDictionary {
    fn new() -> Self {
        Self {
            // field_set: HashSet::new(),
            fields_by_tag: HashMap::new(),
            tag_name_map: HashMap::new(),
            components: HashMap::new(),
            groups: HashMap::new(),
            messages: HashMap::new()
        }
    }

    pub fn add_field(&mut self, fld: FieldEntry) {
        // TODO: add check so that tag should not be present already - duplicate tag, invalid xml
        self.fields_by_tag.insert(fld.number, fld);
    }

    pub fn get_field(&self, tag: &u32) -> Option<&FieldEntry> {
        self.fields_by_tag.get(tag)
    }

    pub fn add_field_tag(&mut self, name_ref: &str, tagnum: u32) {
        self.tag_name_map.insert(name_ref.to_string(), tagnum);
    }

    pub fn get_field_tag(&self, name: &str) -> Option<&u32> {
        // TODO: better error handling
        self.tag_name_map.get(name)
    }

    pub fn add_group(&mut self, gname: &str, grp: Group) {
        self.groups.insert(gname.to_string(), grp);
    }

    pub fn add_component(&mut self, cname: &str, comp: ComponentEntry) {
        self.components.insert(cname.to_string(), comp);
    }

    pub fn get_group(&self, gname: &str) -> Option<&Group> {
        self.groups.get(gname)
    }

    pub fn get_group_mut(&mut self, gname: &str) -> Option<&mut Group> {
        self.groups.get_mut(gname)
    }

    pub fn get_component(&self, cname: &str) -> Option<&ComponentEntry> {
        self.components.get(cname)
    }

    pub fn get_component_mut(&mut self, cname: &str) -> Option<&mut ComponentEntry> {
        self.components.get_mut(cname)
    }

    pub fn add_message(&mut self, mname: &str, mentry: MessageEntry) {
        self.messages.insert(mname.to_string(), mentry);
    }

    fn get_message(&self, mname: &str) -> Option<&MessageEntry> {
        self.messages.get(mname)
    }

    fn get_message_mut(&mut self, mname: &str) -> Option<&mut MessageEntry> {
        self.messages.get_mut(mname)
    }

    pub fn fields_by_name(&self) -> &HashMap<String, u32> {
        &self.tag_name_map
    }

    pub fn fields_by_tag(&self) -> &HashMap<u32, FieldEntry> {
        &self.fields_by_tag
    }

    pub fn messages(&self) -> &HashMap<String, MessageEntry> {
        &self.messages
    }
}

#[derive(Debug)]
pub struct FieldEntry {
    number: u32,
    name: String,
    ftype: String,
    values: HashSet<FieldValueEntry>,
}

impl FieldEntry {
    fn new(number: u32, name: &str, ftype: &str) -> Self {
        Self {
            number,
            name: name.to_string(),
            ftype: ftype.to_string(),
            values: HashSet::new()
        }
    }

    fn set_valid_value(&mut self, val: FieldValueEntry) {
        self.values.insert(val);
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn field_type(&self) -> &String {
        &self.ftype
    }

    fn values(&self) -> &HashSet<FieldValueEntry> {
        &self.values
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct FieldValueEntry {
    value: String,
    description: String
}

impl FieldValueEntry {
    fn new(val: &str, desc: &str) -> Self {
        Self {
            value: val.to_string(),
            description: desc.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ComponentEntry {
    comp_name: String,
    comp_groups: HashMap<String, bool>,
    comp_fields: HashMap<String, bool>,
}

impl ComponentEntry {
    fn new(cname: &str) -> Self {
        Self {
            comp_name: cname.to_string(),
            comp_groups: HashMap::new(),
            comp_fields: HashMap::new()
        }
    }

    fn add_group(&mut self, gname: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.comp_groups.insert(gname.to_string(), required);
    }

    fn add_field(&mut self, fname: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.comp_fields.insert(fname.to_string(), required);
    }
}

#[derive(Debug)]
pub struct Group {
    group_name: String,

    // name of field and required
    group_fields: HashMap<String, bool>,
    // name of component and required
    group_comp: HashMap<String, bool>
}

impl Group {
    fn new(gname: &str) -> Self {
        Self {
            group_name: gname.to_string(),
            group_fields: HashMap::new(),
            group_comp: HashMap::new()
        }
    }

    fn add_field(&mut self, fname: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.group_fields.insert(fname.to_string(), required);
    }

    fn add_component(&mut self, cname: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.group_comp.insert(cname.to_string(), required);
    }
}

#[derive(Debug)]
pub struct MessageEntry {
    msg_name: String,
    msg_category: String,
    msg_type: String,
    msg_fields: HashMap<String, bool>,
    msg_groups: HashMap<String, bool>,
    msg_component: HashMap<String, bool>
}

impl MessageEntry {
    fn new(name: &str, mtype: &str, category: &str) -> Self {
        Self {
            msg_name: name.to_string(),
            msg_type: mtype.to_string(),
            msg_category: category.to_string(),
            msg_fields: HashMap::new(),
            msg_groups: HashMap::new(),
            msg_component: HashMap::new()
        }
    }

    fn add_field(&mut self, name: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.msg_fields.insert(name.to_string(), required);
    }

    fn is_field_required(&self, name: &str) -> bool {
        match self.msg_fields.get(name) {
            Some(&required) => required,
            None => false
        }
    }

    fn add_group(&mut self, name: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.msg_groups.insert(name.to_string(), required);
    }

    fn is_group_required(&self, name: &str) -> bool {
        match self.msg_groups.get(name) {
            Some(&required) => required,
            None => false
        }
    }


    fn add_component(&mut self, name: &str, req: &str) {
        let mut required = false;
        if req.eq_ignore_ascii_case("y") {
            required = true;
        }
        self.msg_component.insert(name.to_string(), required);
    }

    fn is_component_required(&self, name: &str) -> bool {
        match self.msg_component.get(name) {
            Some(&required) => required,
            None => false
        }
    }

}

pub fn create_data_dict(fix_xml: &str) -> Option<DataDictionary> {
    let mut file_data = String::with_capacity(1024*64);
    let mut file = File::open(fix_xml).unwrap();
    file.read_to_string(&mut file_data).unwrap();
    let doc = Document::parse(&file_data).unwrap();
    let mut dictionary = DataDictionary::new();

    let field_node = doc.root_element()
                            .children()
                            .find(|node| node.tag_name().name() == "fields").unwrap();
    field_handler(field_node, &mut dictionary);

    let component_node = doc.root_element()
                            .children()
                            .find(|node| node.tag_name().name() == "components").unwrap();
    component_handler(component_node, &mut dictionary);

    let header_node = doc.root_element()
                            .children()
                            .find(|node| node.tag_name().name() == "header").unwrap();
    header_trailer_handler(header_node, &mut dictionary);

    let trailer_node = doc.root_element()
                            .children()
                            .find(|node| node.tag_name().name() == "trailer").unwrap();
    header_trailer_handler(trailer_node, &mut dictionary);

    let message_node = doc.root_element()
                        .children()
                        .find(|node| node.tag_name().name() == "messages").unwrap();
    message_handler(message_node, &mut dictionary);
    Some(dictionary)
}

fn field_handler(field_node: Node, dict: &mut DataDictionary) {
    for node in field_node.children().filter(|n| n.node_type() == NodeType::Element) {
        let fname = node.attribute("name").unwrap();
        let fnum = node.attribute("number").unwrap().parse::<u32>().unwrap();
        let ftype = node.attribute("type").unwrap();
        let mut f_entry =  FieldEntry::new(fnum, fname, ftype);
        for child in node.children().filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("value")) {
            let fvalue_entry = FieldValueEntry::new(
                child.attribute("enum").unwrap(),
                child.attribute("description").unwrap()
            );
            f_entry.set_valid_value(fvalue_entry);
        }
        dict.add_field_tag(fname, fnum);
        dict.add_field(f_entry);
    }
}

fn component_handler(comp_node: Node, dict: &mut DataDictionary) {
    for node in comp_node.children().filter(|n| n.node_type() == NodeType::Element) {
        let comp_name = node.attribute("name").unwrap();
        let mut cmp_entry = ComponentEntry::new(comp_name);
        for field_group in node.children().filter(|n| n.node_type() == NodeType::Element) {
            match field_group.tag_name().name() {
                "field" => {
                    let fname = field_group.attribute("name").unwrap();
                    cmp_entry.add_field(fname, field_group.attribute("required").unwrap());
                },
                "group" => {
                    let grp_name = field_group.attribute("name").unwrap();
                    let grp_req = field_group.attribute("required").unwrap();
                    let mut group = Group::new(grp_name);
                    for group_field in field_group.children().filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("field")) {
                        group.add_field(group_field.attribute("name").unwrap(), 
                            group_field.attribute("required").unwrap());
                    }
                    dict.add_group(grp_name, group);
                    cmp_entry.add_group(grp_name, grp_req);
                },
                _ => {} // Do nothing TODO: Error handling
            }
        }
        dict.add_component(comp_name, cmp_entry);
    }
}

fn message_handler(node: Node, dict: &mut DataDictionary) {
    for msg_node in node.children().filter(|n| n.node_type() == NodeType::Element) {
        let msg_name = msg_node.attribute("name").unwrap();
        let msg_type = msg_node.attribute("msgtype").unwrap();
        let msg_cat = msg_node.attribute("msgcat").unwrap();
        let mut message = MessageEntry::new(msg_name, msg_type, msg_cat);
        for msg_child_node in msg_node.children().filter(|n| n.node_type() == NodeType::Element) {
            match msg_child_node.tag_name().name() {
                "field" => {
                    let fname = msg_child_node.attribute("name").unwrap();
                    message.add_field(fname, msg_child_node.attribute("required").unwrap());
                },
                "group" => {
                    let grp_name = msg_child_node.attribute("name").unwrap();
                    let grp_req = msg_child_node.attribute("required").unwrap();
                    if dict.get_group(grp_name).is_none() {
                        let mut group = Group::new(grp_name);
                        for group_field in msg_child_node
                            .children()
                            .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("field")) {
                                group.add_field(group_field.attribute("name").unwrap(), 
                                group_field.attribute("required").unwrap());
                        }
                        dict.add_group(grp_name, group);
                    }
                    message.add_group(grp_name, grp_req);
                },
                "component" => {
                    let cname = msg_child_node.attribute("name").unwrap();
                    let req = msg_child_node.attribute("name").unwrap();
                    message.add_component(cname, req);
                },
                _ => {} // TODO: Error handling
            }
        }
        dict.add_message(msg_name, message);
    }

}

fn header_trailer_handler(node: Node, dict: &mut DataDictionary) {
    let cmp_name = node.tag_name();
    let mut header_entry = ComponentEntry::new(cmp_name.name());
    for field_group in node.children().filter(|n| n.node_type() == NodeType::Element) {
            match field_group.tag_name().name() {
                "field" => {
                    let fname = field_group.attribute("name").unwrap();
                    header_entry.add_field(fname, field_group.attribute("required").unwrap());
                },
                "group" => {
                    let grp_name = field_group.attribute("name").unwrap();
                    let grp_req = field_group.attribute("required").unwrap();
                    if dict.get_group(grp_name).is_none() {
                        let mut group = Group::new(grp_name);
                        for group_field in field_group
                            .children()
                            .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("field")) {
                                group.add_field(group_field.attribute("name").unwrap(), 
                                group_field.attribute("required").unwrap());
                        }
                        dict.add_group(grp_name, group);
                    }
                    header_entry.add_group(grp_name, grp_req);
                },
                _ => {} // Do nothing TODO: Error handling
            }
        }
        dict.add_component(cmp_name.name(), header_entry);
}

pub fn generate_code(file_path: &str, dict: DataDictionary) {
    let mut file = File::create(file_path).unwrap();
    file.write("#[derive(Debug)]\npub enum TagNum {\n".as_bytes());
    for field in dict.fields_by_name() {
        file.write_fmt(format_args!("{} => {},\n", field.0, field.1));
    }
    file.write("}".as_bytes());
}