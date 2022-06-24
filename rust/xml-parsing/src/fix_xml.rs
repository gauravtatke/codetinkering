use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io::Read, slice::Iter};
use yaserde::{YaDeserialize, YaSerialize,de};
use yaserde_derive::YaDeserialize;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug, Default, YaDeserialize)]
pub struct Fix {
    #[yaserde(attribute, rename = "type")]
    fix_type: String,
    #[yaserde(attribute)]
    major: u16,
    #[yaserde(attribute)]
    minor: u16,
    #[yaserde(attribute)]
    servicepack: String,
    #[yaserde(child)]
    header: Header,
    #[yaserde(child)]
    trailer: Trailer,
    #[yaserde(rename = "components")]
    supported_components: Components,
    #[yaserde(rename = "messages")]
    supported_messages: Messages,
    #[yaserde(rename = "fields")]
    supported_fields: Fields,
    // fields: String,
}

#[derive(Debug, Default, YaDeserialize)]
struct Components {
    #[yaserde(rename = "component")]
    components: Vec<ComponentStruct>,
}

impl Components {
    fn iter(&self) -> Iter<ComponentStruct> {
        self.components.iter()
    }
}

#[derive(Debug, Default, YaDeserialize)]
struct Messages {
    #[yaserde(rename = "message")]
    messages: Vec<Message>,
}

impl Messages {
    pub fn iter(&self) -> Iter<Message> {
        self.messages.iter()
    }
}

#[derive(Debug, YaDeserialize, Default)]
struct Fields {
    #[yaserde(rename = "field")]
    fields: Vec<FieldStruct>,
}

impl Fields {
    fn iter(&self) -> Iter<FieldStruct> {
        self.fields.iter()
    }
}

#[derive(Debug, Default, YaDeserialize)]
struct Header {
    #[yaserde(rename = "field")]
    fields: Vec<Field>,
    #[yaserde(rename = "group")]
    groups: Vec<Group>,
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
struct Field {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    required: String,
}

impl Field {
    fn create(name: &str, req: &str) -> Self {
        Self {
            name: name.to_string(),
            required: req.to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Group {
    name: String,
    required: String,
    fields: Vec<Field>,
    sub_group: Vec<Group>,
    components: Vec<Component>,
}

impl Group {
    fn set_name(&mut self, name: String) {
        self.name = name.to_string();
    }

    fn set_required(&mut self, required: String) {
        self.required = required;
    }

    fn set_group(&mut self, group: Group) {
        self.sub_group.push(group);
    }

    fn set_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    fn set_component(&mut self, component: Component) {
        self.components.push(component);
    }

    fn create(
        name: &str,
        req: &str,
        fields: Vec<Field>,
        groups: Vec<Group>,
        comps: Vec<Component>,
    ) -> Self {
        Self {
            name: name.to_string(),
            required: req.to_string(),
            fields: fields,
            sub_group: groups,
            components: comps,
        }
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

impl YaDeserialize for Group {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let mut group: Group = Default::default();
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
                            // intuitively it should not case problem cuz we already processed
                            // peeked element but it messes with YaSerde's further deserialization
                            // in this kind of recursive deserialization
                            let another_group = Group::deserialize(reader)?;
                            group.set_group(another_group);
                        } else if name.local_name.eq_ignore_ascii_case("field") {
                            // fields inside the group
                            // skip the next element cuz already processed the same
                            let field = Field::deserialize(reader)?;
                            group.set_field(field);
                            let _xml = reader.next_event();
                        } else if name.local_name.eq_ignore_ascii_case("component") {
                            // component inside the group
                            // skip the next element cuz already processed the same
                            let component = Component::deserialize(reader)?;
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
struct Trailer {
    #[yaserde(rename = "field")]
    fields: Vec<Field>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
struct Message {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    msgtype: String,
    #[yaserde(attribute)]
    msgcat: String,
    #[yaserde(rename = "field")]
    fields: Vec<Field>,
    #[yaserde(rename = "component")]
    components: Vec<Component>,
    #[yaserde(rename = "group")]
    groups: Vec<Group>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
struct ComponentStruct {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(rename = "field")]
    fields: Vec<Field>,
    #[yaserde(rename = "group")]
    groups: Vec<Group>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
struct Component {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    required: String,
}

impl Component {
    fn create(name: &str, req: &str) -> Self {
        Self {
            name: name.to_string(),
            required: req.to_string(),
        }
    }
}
#[derive(Debug, YaDeserialize, Default)]
struct FieldStruct {
    #[yaserde(attribute)]
    number: u32,
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute, rename = "type")]
    ftype: String,
    #[yaserde(rename = "value")]
    values: Vec<Value>,
}

#[derive(Debug, YaDeserialize, Default, PartialEq)]
struct Value {
    #[yaserde(attribute, rename = "enum")]
    enum_val: String,
    #[yaserde(attribute)]
    description: String,
}

fn print_yaserde_fix(xml_file: &str) {
    let xml_str = fs::read_to_string(xml_file).unwrap();
    let before_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("time before {}", before_time);
    let fix43 = de::from_str::<Fix>(&xml_str).unwrap();
    let after_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("{:#?}", &fix43);
}

pub fn get_fix(xml_file: &str) -> Fix {
    let xml_str = fs::read_to_string(xml_file).unwrap();
    let fix = de::from_str(&xml_str).unwrap();
    fix
}

#[cfg(test)]
mod fix_xml_test {
    use super::*;

    impl Message {
        fn create(
            name: &str,
            ty: &str,
            cat: &str,
            fld: Vec<Field>,
            grp: Vec<Group>,
            comp: Vec<Component>,
        ) -> Message {
            Self {
                name: name.to_string(),
                msgtype: ty.to_string(),
                msgcat: cat.to_string(),
                fields: fld,
                components: comp,
                groups: grp,
            }
        }
    }

    impl ComponentStruct {
        fn create(name: &str, fld: Vec<Field>, grp: Vec<Group>) -> Self {
            Self {
                name: name.to_string(),
                fields: fld,
                groups: grp,
            }
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
        let expected_group = Group::create(
            "NoHops",
            "N",
            vec![
                Field::create("HopCompID", "N"),
                Field::create("HopSendingTime", "N"),
                Field::create("HopRefID", "N"),
            ],
            vec![],
            vec![],
        );
        let actual_group: Group = de::from_str::<Group>(group_xml).unwrap();
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
        let actual_message = de::from_str::<Message>(xml).unwrap();
        let expected_message = Message::create(
            "ExecutionReport",
            "8",
            "app",
            vec![
                Field::create("OrderID", "Y"),
                Field::create("ClOrdID", "N"),
                Field::create("ExecType", "Y"),
            ],
            vec![
                Group::create(
                    "NoContraBrokers",
                    "N",
                    vec![
                        Field::create("ContraBroker", "N"),
                        Field::create("ContraTrader", "N"),
                    ],
                    vec![],
                    vec![],
                ),
                Group::create(
                    "NoContAmts",
                    "N",
                    vec![
                        Field::create("ContAmtType", "N"),
                        Field::create("ContAmtValue", "N"),
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
        let actual_message = de::from_str::<Message>(xml).unwrap();
        let expected_message = Message::create(
            "ExecutionReport",
            "8",
            "app",
            vec![
                Field::create("OrderID", "Y"),
                Field::create("ClOrdID", "N"),
                Field::create("ExecType", "Y"),
            ],
            vec![Group::create(
                "NoContraBrokers",
                "N",
                vec![
                    Field::create("ContraBroker", "N"),
                    Field::create("ContraTrader", "N"),
                ],
                vec![Group::create(
                    "NoContAmts",
                    "N",
                    vec![
                        Field::create("ContAmtType", "N"),
                        Field::create("ContAmtValue", "N"),
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
        let actual_message = de::from_str::<Message>(xml).unwrap();
        let expected_message = Message::create(
            "ExecutionReport",
            "8",
            "app",
            vec![Field::create("OrderID", "Y"), Field::create("ClOrdID", "N")],
            vec![Group::create(
                "NoLegs",
                "N",
                vec![
                    Field::create("LegRefID", "N"),
                    Field::create("LegPrice", "N"),
                ],
                vec![],
                vec![
                    Component::create("InstrumentLeg", "N"),
                    Component::create("NestedParties", "N"),
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
        let actual_component = de::from_str::<ComponentStruct>(xml).unwrap();
        let expected_component = ComponentStruct::create(
            "InstrumentLeg",
            vec![
                Field::create("LegSymbol", "N"),
                Field::create("LegSecurityID", "N"),
            ],
            vec![Group::create(
                "NoLegs",
                "N",
                vec![
                    Field::create("LegRefID", "N"),
                    Field::create("LegPrice", "N"),
                ],
                vec![],
                vec![
                    Component::create("InstrumentTest", "N"),
                    Component::create("NestedParties", "N"),
                ],
            )],
        );
        assert_eq!(actual_component, expected_component);
    }
}
