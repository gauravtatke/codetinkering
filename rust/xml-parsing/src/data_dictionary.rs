use std::cmp::{Eq, PartialEq};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{Read, Write};
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::str::FromStr;
use std::{convert::Into, fmt, fs::File, path};

use crate::message::{Group, StringField};
use crate::quickfix_errors::*;
use indexmap::IndexSet;
use roxmltree::{Document, Node, NodeType};

type NodeMap<'a, 'i> = HashMap<String, Node<'a, 'i>>;
type DResult<T> = Result<T, XmlError>;

pub(crate) const HEADER_ID: &str = "Header";
pub(crate) const TRAILER_ID: &str = "Trailer";

#[derive(Debug, Copy, Clone)]
enum FixType {
    Char,
    Boolean,
    Data,
    Float,
    Amt,
    Percentage,
    Price,
    PriceOffset,
    Qty,
    Int,
    Length,
    NumInGroup,
    Seqnum,
    Tagnum,
    Str,
    Country,
    Currency,
    Exchange,
    LocalMktDate,
    MonthYear,
    MultipleValueString,
    UtcDate,
    UtcTimeOnly,
    UtcTimestamp,
    Unknown,
}

impl FixType {
    fn value_of(value: &str) -> Self {
        match value {
            "CHAR" => FixType::Char,
            "BOOLEAN" => FixType::Boolean,
            "DATA" => FixType::Data,
            "FLOAT" => FixType::Float,
            "AMT" => FixType::Amt,
            "PERCENTAGE" => FixType::Percentage,
            "PRICE" => FixType::Price,
            "PRICEOFFSET" => FixType::PriceOffset,
            "QTY" => FixType::Qty,
            "INT" => FixType::Int,
            "LENGTH" => FixType::Length,
            "NUMINGROUP" => FixType::NumInGroup,
            "SEQNUM" => FixType::Seqnum,
            "TAGNUM" => FixType::Tagnum,
            "STRING" => FixType::Str,
            "COUNTRY" => FixType::Country,
            "CURRENCY" => FixType::Currency,
            "EXCHANGE" => FixType::Exchange,
            "LOCALMKTDATE" => FixType::LocalMktDate,
            "MONTHYEAR" => FixType::MonthYear,
            "MULTIPLEVALUESTRING" => FixType::MultipleValueString,
            "UTCDATE" => FixType::UtcDate,
            "UTCTIMEONLY" => FixType::UtcTimeOnly,
            "UTCTIMESTAMP" => FixType::UtcTimestamp,
            _ => FixType::Unknown,
        }
    }
}

#[derive(Debug, Default)]
pub struct DataDictionary {
    begin_string: String,
    fields_by_tag: HashMap<u32, String>,
    fields_by_name: HashMap<String, u32>,
    field_values: HashMap<u32, HashSet<String>>,
    field_type: HashMap<u32, FixType>,
    // mapping of msg_type -> group field. i.e "D" -> <78, 386>
    // {"D" -> {78 -> NoAllocsGroupInfo, 386 -> NoTradingSessionGroupInfo}}
    groups: HashMap<String, HashMap<u32, GroupInfo>>, // can have "header" -> {..}
    fields: IndexSet<u32>, // fields of message. mostly useful in group's dd for field order
    message_types: HashMap<String, String>, // "NewOrderSingle" -> "D"
    messsage_category: HashMap<String, String>, // "D" -> "app"
    messsage_fields: HashMap<String, HashSet<u32>>, // "D" -> <44, 54, ...>, "header" -> <..>
    message_required_fields: HashMap<String, HashSet<u32>>,
}

impl DataDictionary {
    fn set_field_name_number_type(&mut self, name: &str, number: u32, ty: &str) -> DResult<()> {
        if self.fields_by_name.contains_key(name) || self.fields_by_tag.contains_key(&number) {
            // return error
            return Err(XmlError::DuplicateField(format!("{}={}", name, number)));
        }
        self.fields_by_name.insert(name.to_string(), number);
        self.fields_by_tag.insert(number, name.to_string());
        self.field_type.entry(number).or_insert_with(|| FixType::value_of(ty));
        Ok(())
    }

    fn set_field_values(&mut self, fnumber: u32, values: HashSet<String>) {
        self.field_values.entry(fnumber).or_insert(values);
    }

    fn add_fields(&mut self, field: u32) {
        // this adds field to fields indexSet which in tern helps provides field order
        // field order only important for groups, not messages
        self.fields.insert(field);
    }

    fn set_msg_name_type_cat(&mut self, msg_name: &str, msg_type: &str, cat: &str) -> DResult<()> {
        if self.messsage_category.contains_key(msg_type)
            || self.message_types.contains_key(msg_name)
        {
            return Err(XmlError::DuplicateMessage(msg_name.to_string()));
        }
        self.message_types.insert(msg_name.to_string(), msg_type.to_string());
        self.messsage_category.insert(msg_type.to_string(), cat.to_string());
        Ok(())
    }

    fn set_field_for(&mut self, msg_type: &str, fnum: u32, required: bool) -> DResult<()> {
        let msg_fields =
            self.messsage_fields.entry(msg_type.to_string()).or_insert_with(HashSet::new);
        if msg_fields.contains(&fnum) {
            return Err(XmlError::DuplicateField(format!(
                "field {} in message {}",
                fnum, msg_type
            )));
        }
        msg_fields.insert(fnum);
        if required {
            self.message_required_fields.entry(msg_type.to_owned()).and_modify(|v| {
                v.insert(fnum);
            });
        }
        Ok(())
    }

    fn set_group_info(&mut self, msg_type: &str, grp_num: u32, info: GroupInfo) {
        // msg_type is value of 35 tag i.e. "D" or "AE" etc
        // for headers, its literal `header`
        self.groups.entry(msg_type.to_string()).and_modify(|hm| {
            hm.entry(grp_num).or_insert(info);
        });
    }

    fn get_field_num(&self, fname: &str) -> u32 {
        let num = self.fields_by_name.get(fname).expect("field name not found in dictionary");
        *num
    }

    fn get_msg_group(&self, msg_type: &str, group_tag: u32) -> Option<&GroupInfo> {
        self.groups.get(msg_type).and_then(|hmap| hmap.get(&group_tag))
    }

    pub fn from_xml(xml_file: &str) -> Self {
        let mut file_data = String::with_capacity(1024 * 64);
        let mut file = File::open(xml_file).unwrap();
        file.read_to_string(&mut file_data).unwrap();
        Self::from_str(&file_data).unwrap()
    }

    pub fn is_group(&self, msg_type: &str, fld: &StringField) -> bool {
        self.groups.get(msg_type).and_then(|val_map| val_map.get(&fld.tag())).is_some()
    }

    pub fn is_header_field(&self, fld: &StringField) -> bool {
        self.is_msg_field(HEADER_ID, fld)
    }

    pub fn get_group(&self, msg_type: &str, fld: &StringField) -> &GroupInfo {
        self.groups
            .get(msg_type)
            .and_then(|gi| gi.get(&fld.tag()))
            .expect("group not found")
    }

    pub fn get_ordered_fields(&self) -> Vec<u32> {
        self.fields.iter().copied().collect::<Vec<u32>>()
    }

    pub fn is_msg_field(&self, msg_type: &str, fld: &StringField) -> bool {
        self.messsage_fields.get(msg_type).and_then(|val| val.get(&fld.tag())).is_some()
    }

    pub fn is_trailer_field(&self, fld: &StringField) -> bool {
        self.is_msg_field(TRAILER_ID, fld)
    }
}

impl FromStr for DataDictionary {
    type Err = XmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dd = Self::default();
        create_dictionary(s, &mut dd)?;
        Ok(dd)
    }
}
#[derive(Debug, Default)]
pub struct GroupInfo {
    delimiter: u32,
    group_dd: DataDictionary,
}

impl GroupInfo {
    pub fn get_data_dictionary(&self) -> &DataDictionary {
        &self.group_dd
    }

    pub fn get_delimiter(&self) -> u32 {
        self.delimiter
    }
}

fn get_attribute<'a>(attr: &str, node: &Node<'a, '_>) -> DResult<&'a str> {
    node.attribute(attr).ok_or_else(|| {
        XmlError::AttributeNotFound(format!("{} in {:?}", attr, node.tag_name().name()))
    })
}
fn get_name_attr<'a>(node: &Node<'a, '_>) -> DResult<&'a str> {
    get_attribute("name", node)
}

fn get_required_attr(node: &Node) -> DResult<bool> {
    let att = get_attribute("required", node)?;
    Ok(att.eq_ignore_ascii_case("Y"))
}

fn get_begin_str_from_doc(root_node: Node) -> DResult<String> {
    let dict_type = root_node
        .attribute("type")
        .ok_or_else(|| XmlError::AttributeNotFound("fix type in root node".to_string()))?;
    let major_version = root_node
        .attribute("major")
        .ok_or_else(|| XmlError::AttributeNotFound("major version in root node".to_string()))?;
    let minor_verion = root_node
        .attribute("minor")
        .ok_or_else(|| XmlError::AttributeNotFound("minor version in root node".to_string()))?;
    Ok(format!("{}.{}.{}", dict_type, major_version, minor_verion))
}

fn lookup_node<'a, 'input>(
    name: &str, document: &'a Document<'input>,
) -> DResult<Node<'a, 'input>> {
    // find the node in the document with given name
    document
        .root_element()
        .children()
        .find(|node| node.tag_name().name().eq_ignore_ascii_case(name))
        .ok_or_else(|| XmlError::XmlNodeNotFound(name.to_string()))
}

fn get_component_nodes_by_name<'a, 'i>(components: Node<'a, 'i>) -> DResult<NodeMap<'a, 'i>> {
    let mut cmap: HashMap<String, Node> = HashMap::new();
    for node in components.children().filter(|cnode| cnode.is_element()) {
        let cname = get_name_attr(&node)?;
        cmap.insert(cname.to_string(), node);
    }
    Ok(cmap)
}

fn get_field_values(node: Node) -> HashSet<String> {
    HashSet::from_iter(
        node.children()
            .filter(|n| n.is_element() && n.has_tag_name("value"))
            .map(|n| get_attribute("enum", &n).expect("Enum value not present").to_owned()),
    )
}

fn add_fields_and_values(fields: Node, dd: &mut DataDictionary) -> DResult<()> {
    for field_node in fields.children().filter(|node| node.is_element()) {
        let name = get_name_attr(&field_node)?;
        let number = match get_attribute("number", &field_node)?.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                return Err(XmlError::FieldNotParsed {
                    source: e,
                    field: name.to_string(),
                })
            }
        };
        let typ = get_attribute("type", &field_node)?;
        dd.set_field_name_number_type(name, number, typ)?;
        let values = get_field_values(field_node);
        dd.set_field_values(number, values);
    }
    Ok(())
}

#[inline]
fn add_fields_to(
    msg_type: &str, field_name: &str, is_required: bool, dd: &mut DataDictionary,
) -> DResult<u32> {
    let field_number = dd.get_field_num(field_name);
    dd.set_field_for(msg_type, field_number, is_required)?;
    dd.add_fields(field_number);
    Ok(field_number)
}

fn add_xml_group(
    msg_type: &str, group_node: &Node, components: &NodeMap, dd: &mut DataDictionary,
) -> DResult<()> {
    // process the group node and add fields, components, subgroup
    // for the message name and message type
    let mut group_dd = DataDictionary::default();
    let group_required = get_required_attr(group_node)?;
    let mut delimiter = 0u32;
    for grp_child in group_node.children().filter(|&n| n.is_element()) {
        let child_name = grp_child.tag_name().name();
        let first_field: u32 = match child_name {
            "field" => {
                let fname = get_name_attr(&grp_child)?;
                let required = get_required_attr(&grp_child)?;
                // add this field to group_dd for the msg_name
                // this field is required if group is required and field is required
                let required = required && group_required;
                add_fields_to(msg_type, fname, required, &mut group_dd)?
            }
            "component" => {
                // this component fields are also added in group_dd for msg_name
                let comp_name = get_name_attr(&grp_child)?;
                let comp_required = group_required && get_required_attr(&grp_child)?;
                add_xml_component(msg_type, &grp_child, comp_required, components, &mut group_dd)?
            }
            "group" => {
                // this is subgroup inside group
                let sub_group_name = get_name_attr(&grp_child)?;
                let sub_group_required = get_required_attr(&grp_child)? && group_required;
                // this subgroup fields should be added to group's dd but under msg_type
                let field =
                    add_fields_to(msg_type, sub_group_name, sub_group_required, &mut group_dd)?;
                // process group node separately to create GroupInfo
                // and add it to group dd. Mapping should be with msg_type
                add_xml_group(msg_type, &grp_child, components, &mut group_dd)?;
                field
            }
            _ => return Err(XmlError::UnknownXmlTag(child_name.to_string())),
        };
        if delimiter == 0 {
            delimiter = first_field;
        }
    }
    let group_info = GroupInfo {
        delimiter,
        group_dd,
    };
    let group_name = get_name_attr(group_node)?;
    let group_tag = dd.get_field_num(group_name);
    dd.set_group_info(msg_type, group_tag, group_info);
    Ok(())
}

fn add_xml_component(
    msg_type: &str, comp_node: &Node, is_required: bool, components: &NodeMap,
    dd: &mut DataDictionary,
) -> DResult<u32> {
    // first_field is the first field encountered in processing the node
    // it only useful for groups where this serves as the delimiter.
    let mut first_field = 0u32;
    for child in comp_node.children().filter(|n| n.is_element()) {
        let child_name = child.tag_name().name();
        let num = match child_name {
            "field" => {
                let fname = get_name_attr(&child)?;
                // if component is required and component's field is required
                // then field is required for message
                let required = get_required_attr(&child)? && is_required;
                add_fields_to(msg_type, fname, required, dd)?
            }
            "component" => {
                // most likely components do not contain components but
                // adding this for completeness.
                let comp_required = get_required_attr(&child)?;
                let comp_node = components.get(child_name).expect("component node not found");
                // component inside component is only required if outer comp and this are req
                // otherwise not required
                let is_comp_required = comp_required && is_required;
                add_xml_component(msg_type, comp_node, is_comp_required, components, dd)?
            }
            "group" => {
                // this group field is added to message fields
                let group_name = get_name_attr(&child)?;
                // if component is required && group inside component is required then
                // group is required for message
                let group_required = get_required_attr(&child)? && is_required;
                let field = add_fields_to(msg_type, group_name, group_required, dd)?;
                // process group node separately to create GroupInfo
                // and add it to dd for the message
                add_xml_group(msg_type, &child, components, dd)?;
                field
            }
            _ => return Err(XmlError::UnknownXmlTag(child_name.to_string())),
        };
        if first_field == 0 {
            first_field = num;
        }
    }
    Ok(first_field)
}

fn add_xml_message(
    msg_type: &str, node: &Node, components: &NodeMap, dd: &mut DataDictionary,
) -> DResult<()> {
    // first_field is the first field encountered in processing the node
    // it only useful for groups where this serves as the delimiter.
    // let mut first_field = 0u32;
    for child in node.children().filter(|n| n.is_element()) {
        let child_name = child.tag_name().name();
        match child_name {
            "field" => {
                let fname = get_name_attr(&child)?;
                let required = get_required_attr(&child)?;
                add_fields_to(msg_type, fname, required, dd)?;
            }
            "component" => {
                let comp_required = get_required_attr(&child)?;
                let comp_node = components.get(child_name).expect("component node not found");
                add_xml_component(msg_type, comp_node, comp_required, components, dd)?;
            }
            "group" => {
                // this group field is added to message fields
                let group_name = get_name_attr(&child)?;
                let group_required = get_required_attr(&child)?;
                add_fields_to(msg_type, group_name, group_required, dd)?;
                // process group node separately to create GroupInfo
                // and add it to dd for the message type
                add_xml_group(msg_type, &child, components, dd)?;
            }
            _ => return Err(XmlError::UnknownXmlTag(child_name.to_string())),
        };
    }
    Ok(())
}

fn create_dictionary(xml_str: &str, dd: &mut DataDictionary) -> DResult<()> {
    let doc = Document::parse(xml_str)?;
    let begin_string = get_begin_str_from_doc(doc.root_element())?;
    dd.begin_string = begin_string;

    let fields = lookup_node("fields", &doc)?;
    add_fields_and_values(fields, dd)?;

    let component_node = lookup_node("components", &doc)?;
    let component_map: NodeMap = get_component_nodes_by_name(component_node)?;

    let header_node = lookup_node(HEADER_ID, &doc)?;
    add_xml_message("header", &header_node, &component_map, dd)?;

    let trailer_node = lookup_node("trailer", &doc)?;
    add_xml_message("trailer", &trailer_node, &component_map, dd)?;

    let messages = lookup_node("messages", &doc)?;
    for msg_node in messages
        .children()
        .filter(|n| n.is_element() && n.tag_name().name().eq_ignore_ascii_case("message"))
    {
        let message_name = get_name_attr(&msg_node)?;
        let message_category = get_attribute("msgcat", &msg_node)?;
        let message_type = get_attribute("msgtype", &msg_node)?;
        dd.set_msg_name_type_cat(message_name, message_type, message_category)?;
        add_xml_message(message_type, &msg_node, &component_map, dd)?;
    }
    Ok(())
}
