use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::result;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io::Read, slice::Iter};
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};
use yaserde::{de, YaDeserialize, YaSerialize};
use yaserde_derive::YaDeserialize;

#[derive(Debug, Default, YaDeserialize)]
pub struct XmlFix {
    #[yaserde(attribute, rename = "type")]
    pub fix_type: String,
    #[yaserde(attribute)]
    pub major: u16,
    #[yaserde(attribute)]
    pub minor: u16,
    #[yaserde(attribute)]
    servicepack: String,
    #[yaserde(child)]
    pub header: XmlHeader,
    #[yaserde(child)]
    pub trailer: XmlTrailer,
    #[yaserde(rename = "components")]
    pub supported_components: XmlComponents,
    #[yaserde(rename = "messages")]
    pub supported_messages: XmlMessages,
    #[yaserde(rename = "fields")]
    pub supported_fields: XmlFields,
    // fields: String,
}

impl XmlFix {
    pub fn fields_iter(&self) -> Iter<XmlFieldStruct> {
        self.supported_fields.iter()
    }

    pub fn messages_iter(&self) -> Iter<XmlMessage> {
        self.supported_messages.messages.iter()
    }

    pub fn components_iter(&self) -> Iter<XmlComponentStruct> {
        self.supported_components.components.iter()
    }

    pub fn get_components_name_map(&self) -> HashMap<String, XmlComponentStruct> {
        return self
            .components_iter()
            .map(|comp| (comp.name.clone(), comp.clone()))
            .collect::<HashMap<String, XmlComponentStruct>>();
    }

    pub fn get_fields_by_name(&self) -> HashMap<String, u32> {
        // returns field name to number mapping
        let result: HashMap<String, u32> =
            self.fields_iter().map(|f| (f.name.clone(), f.number)).collect();
        result
    }

    pub fn get_fields_by_tag(&self) -> HashMap<u32, String> {
        // returns number to name mapping
        let result: HashMap<u32, String> =
            self.fields_iter().map(|f| (f.number, f.name.clone())).collect();
        result
    }

    pub fn get_fields_by_type(&self) -> HashMap<u32, String> {
        // returns field number and type mapping
        let result: HashMap<u32, String> =
            self.fields_iter().map(|f| (f.number, f.ftype.clone())).collect();
        result
    }

    pub fn get_fields_by_values(&self) -> HashMap<u32, HashSet<String>> {
        // return field number to values mapping
        let result: HashMap<u32, HashSet<String>> = self
            .fields_iter()
            .map(|f| {
                let values: HashSet<String> =
                    HashSet::from_iter(f.values.iter().map(|en| en.enum_val.clone()));
                (f.number, values)
            })
            .collect();
        result
    }

    pub fn get_message_name_for_num(&self) -> HashMap<String, u32> {
        todo!()
    }

    pub fn get_message_name_category(&self) -> HashMap<String, String> {
        let result: HashMap<String, String> =
            self.messages_iter().map(|msg| (msg.name.clone(), msg.msgcat.clone())).collect();
        result
    }

    pub fn get_message_name_type(&self) -> HashMap<String, String> {
        let result: HashMap<String, String> = self
            .messages_iter()
            .map(|msg| (msg.name.clone(), msg.msgtype.clone()))
            .collect();
        result
    }

    #[inline]
    fn get_fields_tag_for(&self, mtype: &str, name: Option<&str>) -> HashMap<u32, bool> {
        // name is "header", "trailer" or message or component name
        // mtype is "header", "trailer", "message" or "component"
        // returns field num to required mapping
        let fields_by_name = self.get_fields_by_name();
        let fields = match mtype.to_ascii_lowercase().as_str() {
            "header" => &self.header.fields,
            "trailer" => &self.trailer.fields,
            "component" => {
                let comp = self
                    .components_iter()
                    .find(|&comp| comp.name.eq_ignore_ascii_case(name.unwrap()))
                    .unwrap();
                &comp.fields
            }
            "message" => {
                let message = self
                    .messages_iter()
                    .find(|&msg| msg.name.eq_ignore_ascii_case(name.unwrap()))
                    .unwrap();
                &message.fields
            }
            _ => panic!("invalid mtype {}", mtype),
        };
        let result: HashMap<u32, bool> = fields
            .iter()
            .map(|field| {
                (
                    fields_by_name.get(&field.name).unwrap().clone(),
                    field.required.eq_ignore_ascii_case("y"),
                )
            })
            .collect();
        result
    }

    pub fn get_header_fields(&self) -> HashMap<u32, bool> {
        self.get_fields_tag_for("header", None)
    }

    pub fn get_trailer_fields(&self) -> HashMap<u32, bool> {
        self.get_fields_tag_for("trailer", None)
    }

    pub fn get_message_fields(&self, msg_name: &str) -> HashMap<u32, bool> {
        self.get_fields_tag_for("message", Some(msg_name))
    }

    pub fn get_component_fields(&self, comp_name: &str) -> HashMap<u32, bool> {
        self.get_fields_tag_for("component", Some(comp_name))
    }

    fn get_components_in(&self, msg: &str) -> HashMap<String, bool> {
        let message = self.messages_iter().find(|&m| m.name.eq_ignore_ascii_case(msg)).unwrap();
        let result = message
            .components
            .as_slice()
            .iter()
            .map(|c| (c.name.clone(), c.required.to_ascii_lowercase() == "y"))
            .collect();
        result
    }

    pub fn get_header_components(&self) -> HashMap<String, bool> {
        HashMap::new() // header has no components
    }

    pub fn get_trailer_components(&self) -> HashMap<String, bool> {
        HashMap::new() // trailer has no components
    }

    pub fn get_message_components(&self, name: &str) -> HashMap<String, bool> {
        self.get_components_in(name)
    }

    pub fn get_component_groups(&self, comp_name: &str) -> &Vec<XmlGroup> {
        let component = self
            .components_iter()
            .find(|&cmp| cmp.name.eq_ignore_ascii_case(comp_name))
            .unwrap();
        &component.groups
    }

    pub fn get_message_groups(&self, msg_name: &str) -> &Vec<XmlGroup> {
        let message = self
            .messages_iter()
            .find(|&msg| msg.name.eq_ignore_ascii_case(msg_name))
            .unwrap();
        &message.groups
    }

    pub fn get_group_names_for(&self, ty: &str, name: &str) -> Vec<String> {
        // ty: is type which can be "header", "component", "message"
        // if ty is header, name is ignored
        let groups = match ty.to_ascii_lowercase().as_str() {
            "header" => &self.header.groups,
            "component" => self.get_component_groups(name),
            "message" => self.get_message_groups(name),
            _ => panic!("unknown xml tag"),
        };
        groups.iter().map(|g| g.name.clone()).collect()
    }
}

#[derive(Debug, Default, YaDeserialize)]
pub struct XmlComponents {
    #[yaserde(rename = "component")]
    components: Vec<XmlComponentStruct>,
}

#[derive(Debug, Default, YaDeserialize)]
pub struct XmlMessages {
    #[yaserde(rename = "message")]
    messages: Vec<XmlMessage>,
}

#[derive(Debug, YaDeserialize, Default)]
pub struct XmlFields {
    #[yaserde(rename = "field")]
    fields: Vec<XmlFieldStruct>,
}

impl XmlFields {
    fn iter(&self) -> Iter<XmlFieldStruct> {
        self.fields.iter()
    }
}

#[derive(Debug, Default, YaDeserialize)]
pub struct XmlHeader {
    #[yaserde(rename = "field")]
    pub fields: Vec<XmlField>,
    #[yaserde(rename = "group")]
    pub groups: Vec<XmlGroup>,
}

#[derive(Debug, Default, YaDeserialize, PartialEq, Clone)]
pub struct XmlField {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub required: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct XmlGroup {
    pub name: String,
    pub required: String,
    pub fields: Vec<XmlField>,
    pub sub_group: Vec<XmlGroup>,
    pub components: Vec<XmlComponent>,
}

impl XmlGroup {
    fn set_name(&mut self, name: String) {
        self.name = name.to_string();
    }

    fn set_required(&mut self, required: String) {
        self.required = required;
    }

    fn set_group(&mut self, group: XmlGroup) {
        self.sub_group.push(group);
    }

    fn set_field(&mut self, field: XmlField) {
        self.fields.push(field);
    }

    fn set_component(&mut self, component: XmlComponent) {
        self.components.push(component);
    }

    pub fn get_fields(&self) -> HashMap<String, bool> {
        self.fields
            .as_slice()
            .iter()
            .map(|f| (f.name.clone(), f.required.to_lowercase() == "y"))
            .collect()
    }

    pub fn get_components(&self) -> HashMap<String, bool> {
        // returns components and required map
        self.components
            .as_slice()
            .iter()
            .map(|f| (f.name.clone(), f.required.to_lowercase() == "y"))
            .collect()
    }

    pub fn get_delimiter_field_name(&self) -> String {
        // returns the first field number which is delimiter
        self.fields.get(0).unwrap().name.clone()
    }

    pub fn is_required(&self) -> bool {
        self.required.to_lowercase() == "y"
    }
}

fn get_attributes(attributes: &Vec<OwnedAttribute>) -> Vec<String> {
    attributes
        .iter()
        .map(|a| format!("{}={}", &a.name.local_name, &a.value))
        .collect::<Vec<String>>()
}

fn get_attribute(attr: &str, attributes: &Vec<OwnedAttribute>) -> Option<String> {
    for attrib in attributes.iter() {
        if attrib.name.local_name == attr {
            return Some(attrib.value.to_owned());
        }
    }
    None
}

impl YaDeserialize for XmlGroup {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let mut group: XmlGroup = Default::default();
        let peek_event = reader.peek()?;
        // peek the element first and verify that its group element
        if let XmlEvent::StartElement {
            name, attributes, ..
        } = peek_event
        {
            if !name.local_name.eq_ignore_ascii_case("group") {
                return Err(format!(
                    "Wrong element. Expected - group, Found - {} with attributes {:?}",
                    &name.local_name,
                    get_attributes(&attributes)
                ));
            }
            // add name and required based on the peeked element
            group.set_name(get_attribute("name", &attributes).unwrap());
            group.set_required(get_attribute("required", &attributes).unwrap());
            // skip the next element cuz so far peeked element served the purpose
            let _xml = reader.next_event();
            loop {
                // let peek_event = reader.peek()?;
                match reader.peek()? {
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        if name.local_name.eq_ignore_ascii_case("group") {
                            // this is group inside a group
                            // note: do not skip next element after processing the peeked element
                            // intuitively it should not cause problem cuz we already processed
                            // peeked element but it messes with YaSerde's further deserialization
                            // in this kind of recursive deserialization
                            let another_group = XmlGroup::deserialize(reader)?;
                            group.set_group(another_group);
                        } else if name.local_name.eq_ignore_ascii_case("field") {
                            // fields inside the group
                            // skip the next element cuz already processed the same
                            let field = XmlField::deserialize(reader)?;
                            group.set_field(field);
                            let _xml = reader.next_event();
                        } else if name.local_name.eq_ignore_ascii_case("component") {
                            // component inside the group
                            // skip the next element cuz already processed the same
                            let component = XmlComponent::deserialize(reader)?;
                            group.set_component(component);
                            let _xml = reader.next_event();
                        }
                    }
                    XmlEvent::EndElement { name } => {
                        if name.local_name.eq_ignore_ascii_case("group") {
                            // if end element is for group, break out of loop
                            // do not skip next element as it disturbs the further deserialization
                            // if next element is called here then some deserializer goes into
                            // infinite loop and never yields the result. not sure why it happens.
                            break;
                        }
                        // skip the end element if its of any other type
                        let _xml = reader.next_event();
                    }
                    // break if its any other element
                    _ => break,
                }
            }
        }
        Ok(group)
    }
}

#[derive(Debug, YaDeserialize, Default)]
pub struct XmlTrailer {
    #[yaserde(rename = "field")]
    fields: Vec<XmlField>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
pub struct XmlMessage {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub msgtype: String, // '8', 'D' etc
    #[yaserde(attribute)]
    pub msgcat: String, // admin or app
    #[yaserde(rename = "field")]
    pub fields: Vec<XmlField>,
    #[yaserde(rename = "component")]
    pub components: Vec<XmlComponent>,
    #[yaserde(rename = "group")]
    pub groups: Vec<XmlGroup>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq, Clone)]
pub struct XmlComponentStruct {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(rename = "field")]
    pub fields: Vec<XmlField>,
    #[yaserde(rename = "group")]
    pub groups: Vec<XmlGroup>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq, Clone)]
pub struct XmlComponent {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub required: String,
}

impl XmlComponent {
    fn create_component(name: &str, req: &str) -> Self {
        Self {
            name: name.to_string(),
            required: req.to_string(),
        }
    }
}
#[derive(Debug, YaDeserialize, Default)]
pub struct XmlFieldStruct {
    #[yaserde(attribute)]
    pub number: u32,
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute, rename = "type")]
    pub ftype: String,
    #[yaserde(rename = "value")]
    pub values: Vec<XmlValue>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
pub struct XmlValue {
    #[yaserde(attribute, rename = "enum")]
    pub enum_val: String,
    #[yaserde(attribute)]
    pub description: String,
}

fn print_yaserde_fix(xml_file: &str) {
    let xml_str = fs::read_to_string(xml_file).unwrap();
    let before_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("time before {}", before_time);
    let fix43 = de::from_str::<XmlFix>(&xml_str).unwrap();
    let after_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("{:#?}", &fix43);
}

pub fn get_fix_xml(xml_file: &str) -> XmlFix {
    let xml_str = fs::read_to_string(xml_file).unwrap();
    let fix = de::from_str(&xml_str).unwrap();
    fix
}

#[cfg(test)]
mod fix_xml_test {
    use super::*;

    fn create_group(
        name: &str, req: &str, fields: Vec<XmlField>, groups: Vec<XmlGroup>,
        comps: Vec<XmlComponent>,
    ) -> XmlGroup {
        XmlGroup {
            name: name.to_string(),
            required: req.to_string(),
            fields: fields,
            sub_group: groups,
            components: comps,
        }
    }

    fn create_component(name: &str, req: &str) -> XmlComponent {
        XmlComponent {
            name: name.to_string(),
            required: req.to_string(),
        }
    }

    fn create_field(name: &str, req: &str) -> XmlField {
        XmlField {
            name: name.to_string(),
            required: req.to_string(),
        }
    }

    fn create_message(
        name: &str, ty: &str, cat: &str, fld: Vec<XmlField>, grp: Vec<XmlGroup>,
        comp: Vec<XmlComponent>,
    ) -> XmlMessage {
        XmlMessage {
            name: name.to_string(),
            msgtype: ty.to_string(),
            msgcat: cat.to_string(),
            fields: fld,
            components: comp,
            groups: grp,
        }
    }

    fn create_component_struct(
        name: &str, fld: Vec<XmlField>, grp: Vec<XmlGroup>,
    ) -> XmlComponentStruct {
        Self {
            name: name.to_string(),
            fields: fld,
            groups: grp,
        }
    }

    #[test]
    fn simple_group_de_test() {
        let group_xml = r#"
            <group name="NoHops" required="N">
                <field name="HopCompID" required="N"/>
                <field name="HopSendingTime" required="N"/>
                <field name="HopRefID" required="N"/>
            </group>
        "#;
        let expected_group = create_group(
            "NoHops",
            "N",
            vec![
                create_field("HopCompID", "N"),
                create_field("HopSendingTime", "N"),
                create_field("HopRefID", "N"),
            ],
            vec![],
            vec![],
        );
        let actual_group: XmlGroup = de::from_str::<XmlGroup>(group_xml).unwrap();
        assert_eq!(actual_group, expected_group);
    }

    #[test]
    fn vec_group_test() {
        let xml = r#"
            <message name="ExecutionReport" msgtype="8" msgcat="app">
                <field name="OrderID" required="Y"/>
                <field name="ClOrdID" required="N"/>
                <group name="NoContraBrokers" required="N">
                    <field name="ContraBroker" required="N"/>
                    <field name="ContraTrader" required="N"/>
                </group>
                <field name="ExecType" required="Y"/>
                <group name="NoContAmts" required="N">
                        <field name="ContAmtType" required="N"/>
                        <field name="ContAmtValue" required="N"/>
                </group>
            </message>
        "#;
        let actual_message = de::from_str::<XmlMessage>(xml).unwrap();
        let expected_message = create_message(
            "ExecutionReport",
            "8",
            "app",
            vec![
                create_field("OrderID", "Y"),
                create_field("ClOrdID", "N"),
                create_field("ExecType", "Y"),
            ],
            vec![
                create_group(
                    "NoContraBrokers",
                    "N",
                    vec![
                        create_field("ContraBroker", "N"),
                        create_field("ContraTrader", "N"),
                    ],
                    vec![],
                    vec![],
                ),
                create_group(
                    "NoContAmts",
                    "N",
                    vec![
                        create_field("ContAmtType", "N"),
                        create_field("ContAmtValue", "N"),
                    ],
                    vec![],
                    vec![],
                ),
            ],
            vec![],
        );
        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn group_with_subgroup_test() {
        let xml = r#"
            <message name="ExecutionReport" msgtype="8" msgcat="app">
                <field name="OrderID" required="Y"/>
                <field name="ClOrdID" required="N"/>
                <group name="NoContraBrokers" required="N">
                    <field name="ContraBroker" required="N"/>
                    <field name="ContraTrader" required="N"/>
                    <group name="NoContAmts" required="N">
                            <field name="ContAmtType" required="N"/>
                            <field name="ContAmtValue" required="N"/>
                    </group>
                </group>
                <field name="ExecType" required="Y"/>
            </message>
        "#;
        let actual_message = de::from_str::<XmlMessage>(xml).unwrap();
        let expected_message = create_message(
            "ExecutionReport",
            "8",
            "app",
            vec![
                create_field("OrderID", "Y"),
                create_field("ClOrdID", "N"),
                create_field("ExecType", "Y"),
            ],
            vec![create_group(
                "NoContraBrokers",
                "N",
                vec![
                    create_field("ContraBroker", "N"),
                    create_field("ContraTrader", "N"),
                ],
                vec![create_group(
                    "NoContAmts",
                    "N",
                    vec![
                        create_field("ContAmtType", "N"),
                        create_field("ContAmtValue", "N"),
                    ],
                    vec![],
                    vec![],
                )],
                vec![],
            )],
            vec![],
        );
        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn group_with_components_test() {
        let xml = r#"
            <message name="ExecutionReport" msgtype="8" msgcat="app">
                <field name="OrderID" required="Y"/>
                <field name="ClOrdID" required="N"/>
                <group name="NoLegs" required="N">
                    <component name="InstrumentLeg" required="N"/>
                    <component name="NestedParties" required="N"/>
                    <field name="LegRefID" required="N"/>
                    <field name="LegPrice" required="N"/>
                </group>
            </message>
        "#;
        let actual_message = de::from_str::<XmlMessage>(xml).unwrap();
        let expected_message = create_message(
            "ExecutionReport",
            "8",
            "app",
            vec![create_field("OrderID", "Y"), create_field("ClOrdID", "N")],
            vec![create_group(
                "NoLegs",
                "N",
                vec![create_field("LegRefID", "N"), create_field("LegPrice", "N")],
                vec![],
                vec![
                    create_component("InstrumentLeg", "N"),
                    create_component("NestedParties", "N"),
                ],
            )],
            vec![],
        );
        assert_eq!(actual_message, expected_message);
    }

    #[test]
    fn components_with_group_test() {
        let xml = r#"
            <component name="InstrumentLeg">
                <field name="LegSymbol" required="N"/>
                <field name="LegSecurityID" required="N"/>
                <group name="NoLegs" required="N">
                    <component name="InstrumentTest" required="N"/>
                    <component name="NestedParties" required="N"/>
                    <field name="LegRefID" required="N"/>
                    <field name="LegPrice" required="N"/>
                </group>
            </component>
        "#;
        let actual_component = de::from_str::<XmlComponentStruct>(xml).unwrap();
        let expected_component = create_component_struct(
            "InstrumentLeg",
            vec![
                create_field("LegSymbol", "N"),
                create_field("LegSecurityID", "N"),
            ],
            vec![create_group(
                "NoLegs",
                "N",
                vec![create_field("LegRefID", "N"), create_field("LegPrice", "N")],
                vec![],
                vec![
                    create_component("InstrumentTest", "N"),
                    create_component("NestedParties", "N"),
                ],
            )],
        );
        assert_eq!(actual_component, expected_component);
    }
}
