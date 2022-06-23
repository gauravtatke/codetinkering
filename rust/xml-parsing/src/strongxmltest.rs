use std::fs;
use strong_xml::{XmlRead, XmlError, XmlReader, XmlResult};

#[derive(Debug, Default, XmlRead, PartialEq)]
#[xml(tag = "fix")]
struct Fix {
    #[xml(attr = "type")]
    fix_type: String, 
    #[xml(attr = "major")]
    major: u16,
    #[xml(attr = "minor")]
    minor: u16,
    #[xml(attr = "servicepack")]
    service_pack: String,
    #[xml(default, child = "header")]
    header: Header,
    #[xml(default, child = "trailer")]
    trailer: Trailer,
    #[xml(default, child = "components")]
    components: Vec<Component>,
    #[xml(default, child = "messages")]
    messages: Vec<Message>,
    #[xml(child = "fields")]
    supported_fields: Fields, 
}

#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "fields")]
struct Fields {
    #[xml(child = "field")]
    fields: Vec<Tag>
}

#[derive(Debug, Default, XmlRead, PartialEq)]
#[xml(tag = "header")]
struct Header {
    #[xml(child = "field")]
    header_fields: Vec<Field>,
    #[xml(child = "group")]
    header_groups: Vec<Group>,
}


#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "trailer")]
struct Trailer {}

#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "message")]
struct Message {}

#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "component")]
struct Component {}

#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "field")]
struct Tag {
    #[xml(attr = "number")]
    number: u32,
    #[xml(attr = "name")]
    name: String,
    #[xml(attr = "type")]
    ftype: String,
    #[xml(child = "value")]
    values: Vec<Value> 
}

type BoxGroup = Box<Group>;

#[derive(Debug, Default, XmlRead, PartialEq)]
#[xml(tag = "group")]
struct Group {
    #[xml(attr = "name")]
    name: String,
    #[xml(attr = "required")]
    required: String,
    #[xml(child = "field")]
    group_fields: Vec<Field>,
    #[xml(child = "component")]
    group_comp: Vec<Component>,
    #[xml(default, child = "group")]
    subgroups: Vec<Group>
}

impl<'a> XmlRead<'a> for BoxGroup {
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        Err(XmlError::UnexpectedEof)
    }
}

#[derive(Debug, XmlRead, PartialEq)]
#[xml(tag = "field")]
struct Field {
    #[xml(attr = "name")]
    name: String,
    #[xml(attr = "required")]
    required: String,
}

#[derive(Debug, XmlRead, Default, PartialEq)]
#[xml(tag = "value")]
struct Value {
    #[xml(attr = "enum")]
    enum_val: String,
    #[xml(attr = "description")]
    description: String
}

pub fn print_strongxml_fix(xml_file: &str) {
    let xml_str = fs::read_to_string(xml_file).unwrap();
//     let xml_str2 = r#"
//     <fix type="FIX" major="4" minor="3" servicepack="0">
//         <header></header>
//         <trailer></trailer>
//         <messages></messages>
//         <fields>
//             <field number="626" name="AllocType" type="INT">
//                 <value enum="6" description="BUYSIDE_READY_TO_BOOK_6"/>
//                 <value enum="2" description="BUYSIDE_PRELIMINARY"/>
//             </field>
//             <field number="614" name="LegContractMultiplier" type="FLOAT"/>
//             <field number="615" name="LegCouponRate" type="PERCENTAGE"/>
//         </fields>
//     </fix>"#;
//     let val_xml = r#"<value enum="6" description="BUYSIDE_READY_TO_BOOK_6"/>"#;
//     let fields_xml = r#"
//         <fix type="FIX" major="4" minor="3" servicepack="0">
//             <fields>
//                 <field number="626" name="AllocType" type="INT">
//                     <value enum="6" description="BUYSIDE_READY_TO_BOOK_6"/>
//                     <value enum="2" description="BUYSIDE_PRELIMINARY"/>
//                 </field>
//                 <field number="614" name="LegContractMultiplier" type="FLOAT"/>
//                 <field number="615" name="LegCouponRate" type="PERCENTAGE"/>
//             </fields>
//         </fix>
// "#;

    // println!("{}", &xml_str);
    let fields = Fix::from_str(&xml_str);
    println!("{:?}", &fields);
}