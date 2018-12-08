use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ToString;
use std::fmt;
use std::error::Error;
use std::convert::Into;

#[derive(Debug)]
pub enum FXType {
    Int(i64),
    Length(u32),
    TagNum(u32),
    DayOfMonth(u8),
    SeqNum(u64),
    Float(f64),
    Price(f64),
    PriceOffset(f64),
    Amt(f64),
    Percent(f64),
    Qty(f64),
    Char(char),
    Bool(bool),
    Str(String),
    Currency(String)
}

pub struct FXField {
    tag: u32,
    value: FXType
}


impl FXField {
    fn new(tagnum: u32, val: FXType) -> FXField {
        FXField {
            tag: tagnum,
            value: val
        }
    }
}


// impl FromStr for FXField {
//     type Err = FieldParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Err(FieldParseError {
//             kind: FieldParseErrorKind::EqualSignNotFound
//         })
//     }

// }



pub struct GenericFxField {
    tag: u32,
    value: String
}

impl FromStr for GenericFxField {
    type Err = FieldParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('=') {
            Some(index) => {
                let tagnum: u32 = s[..index].parse()?;
                let raw_val = String::from(&s[index+1..]);
                Ok(GenericFxField {
                    tag: tagnum,
                    value: raw_val
                })
            },
            None => Err(FieldParseError {
                kind: FieldParseErrorKind::EqualSignNotFound
            })
        }
    }
}

impl GenericFxField {
    fn new(tag: u32, value: String) -> GenericFxField {
        GenericFxField {
            tag,
            value
        }
    }
    
    fn set_value<T: ToString>(&mut self, tag: u32, val: T) {
        self.value = val.to_string();
    }
}

// Sample of hashmap
// field_values {
//     44: "Price",
//     40: "Char"
//     38: "Amt"
//     56: "String"
//     44: "Bool"
// }

fn validate(gfd: GenericFxField) -> Result<FXField, FieldParseError> {
       if gfd.tag == 44 {
           let price_val: f64 = gfd.value.parse()?;
           Ok(FXField {
               tag: gfd.tag,
               value: FXType::Price(price_val)
           })
       } else if gfd.tag == 40 {
            let order_val: char = gfd.value.parse()?;
            Ok(FXField {
               tag: gfd.tag,
               value: FXType::Char(order_val)
           })
           
       } else if gfd.tag == 56 {
            let tagetcompid_val: String = gfd.value.parse()?;
            Ok(FXField {
                tag: gfd.tag,
                value: FXType::Str(tagetcompid_val)
            })
       } else {
           Err(FieldParseError {
               kind: FieldParseErrorKind::InvalidValueForTag
           })
       }
}


trait FieldMap {
    
    fn field(&self, fld_tag: u32) -> Option<&GenericFxField>;
    fn group(&self, grp_tag: u32) -> Option<&Vec<GenGroup>>;
    fn group_mut(&self, grp_tag: u32) -> Option<&mut Vec<GenGroup>>
    fn set_field(&mut self, fld_tag: u32, fld_val: GenericFxField);
    fn set_group(&mut self, grp_tag: u32, fld_val: GenericFxField, delim: u32) -> &mut Vec<GenGroup>;
}


struct GenGroup {
    delim_tag: u32,
    field: HashMap<u32, GenericFxField>,
    group: HashMap<u32, Vec<GenGroup>>
}


impl GenGroup {
    fn new(delim: u32) -> Self {
        Self {
            delim_tag: delim, 
            field: HashMap::new(),
            group: HashMap::new()
        }
    }
    
    fn set_fld<T: ToString>(&mut self, tag: u32, pr: T) {
        let gfx_fld = GenericFxField::new(tag, pr.to_string());
        self.field.insert(tag, gfx_fld);
    }
    

}

impl FieldMap for GenGroup {
    fn field(&self, fld_tag: u32) -> Option<&GenericFxField> {
        self.field.get(&fld_tag)
    }
    
    fn group(&self, grp_tag: u32) -> Option<&Vec<GenGroup>> {
        &self.group.get(&grp_tag)
    }
    
    fn set_field(&mut self, fld_tag: u32, fld_val: GenericFxField) {
        self.field.insert(fld_tag, fld_val);
    }
    
    fn set_group(&mut self, grp_tag: u32, grp_val: GenericFxField, delim: u32) -> &mut Vec<GenGroup> {
        let num_rep_group:u32 = grp_val.value::parse()
        self.set_field(grp_tag, grp_val);
        self.group
        
        
    }
}

struct GenMessage {
    msg_field: HashMap<u32, GenericFxField>,
    rep_group: HashMap<u32, GenGroup>
}

impl GenMessage {
    fn new() -> Self {
        Self {
            msg_field: HashMap::new(),
            rep_group: HashMap::new()
        }
    }    
}


//***************** Error Handling ********************//

#[derive(Debug)]
pub enum FieldParseErrorKind {
    EqualSignNotFound,
    InvalidTagNumber,
    InvalidValueForTag,
    // ValueOutOfRangeForTag,
}

#[derive(Debug)]
pub struct FieldParseError {
    kind: FieldParseErrorKind
}

impl fmt::Display for FieldParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FieldParseErrorKind::EqualSignNotFound => {
                write!(f, "Equal(=) sign not found ")
            },
            _ => {
                write!(f, "Something went wrong while parsing")
            }
        }
    }
}

impl Error for FieldParseError {
    fn description(&self) -> &str {
        match self.kind {
            FieldParseErrorKind::EqualSignNotFound => "Missing equal sign",
            _ => "Someting weird happen while parsing"
        }
    }
}

impl From<std::num::ParseIntError> for FieldParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        FieldParseError {
            kind: FieldParseErrorKind::InvalidTagNumber
        }
    }
}

impl From<std::char::ParseCharError> for FieldParseError {
    fn from(error: std::char::ParseCharError) -> Self {
        FieldParseError {
            kind: FieldParseErrorKind::InvalidValueForTag
        }
    }
}

impl From<std::string::ParseError> for FieldParseError {
    fn from(error: std::string::ParseError) -> Self {
        FieldParseError {
            kind: FieldParseErrorKind::InvalidValueForTag
        }
    }
}

impl From<std::num::ParseFloatError> for FieldParseError {
    fn from(error: std::num::ParseFloatError) -> Self {
        FieldParseError {
            kind: FieldParseErrorKind::InvalidValueForTag
        }
    }
}

//*********** Error Handling Ends ****************//

#[allow(unused_variables)]
fn main() {
    let target = "56=gafdgadfg";
    let price = "44=1.9876";
    let order = "40=D";
    let gen_tgt_fld = target.parse::<GenericFxField>().unwrap();
    let gen_pr_fld = price.parse::<GenericFxField>().unwrap();
    let gen_ord_fld = order.parse::<GenericFxField>().unwrap();
    
    println!("target tag = {}, target val = {}", gen_tgt_fld.tag, gen_tgt_fld.value);
    println!("price tag = {}, price val = {}", gen_pr_fld.tag, gen_pr_fld.value);
    println!("order tag = {}, order val = {}", gen_ord_fld.tag, gen_ord_fld.value);
    
    let tgt_field = validate(gen_tgt_fld).unwrap();
    let price_field = validate(gen_pr_fld).unwrap();
    let order_field = validate(gen_ord_fld).unwrap();
    
    println!("spec target tag = {}, spec target val = {:?}", tgt_field.tag, tgt_field.value);
    println!("spec price tag = {}, spec price val = {:?}", price_field.tag, price_field.value);
    println!("spec order tag = {}, spec order val = {:?}", order_field.tag, order_field.value);
    
}