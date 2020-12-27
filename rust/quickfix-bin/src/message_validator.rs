use crate::new_data_dictionary::*;
use crate::message::SOH;
use crate::quickfix_errors::*;

pub fn validate_tag(msg: &str, dict: &DataDict) -> Result<(), NewFixError> {
    // validate that tag is correct according to data_dictionary
    // and value is permissible
    // get the message type
    let msg_type = get_message_type(msg).unwrap();
    for tag_value_str in msg.split(SOH) {
        let tag_and_val: Vec<&str> = tag_value_str.split('=').collect();
        let tag = match tag_and_val[0].parse::<u32>() {
            Ok(t) => t,
            Err(e) => 
                return Err(NewFixError {
                    kind: NewFixErrorKind::ParseError (
                        FixTypeFieldParseErrorKind::NotInt
                    ),
                })
        };
        let val = tag_and_val[1];
        let result = dict.is_tag_value_valid(tag, val);
        if result.is_err() {
        return result;
        }
    }
    Ok(())
}

pub fn get_message_type(msg_str: &str) -> Option<&str> {
    for tag_value in msg_str.split(SOH) {
        if tag_value.starts_with("35") {
            let tag_value_str: Vec<&str> = tag_value.split('=').collect();
            return Some(tag_value_str[1]);
        }
    }
    return None;
}

pub fn get_begin_str(msg_str: &str) -> Option<&str> {
    for tag_value in msg_str.split(SOH) {
        if tag_value.starts_with("8=") {
            let tag_value_str: Vec<&str> = tag_value.split('=').collect();
            return Some(tag_value_str[1]);
        }
    }
    return None;
}

pub fn get_body_length(msg_str: &str) -> Option<u32> {
    for tag_value in msg_str.split(SOH) {
        if tag_value.starts_with("9=") {
            let tag_value_str: Vec<&str> = tag_value.split('=').collect();
            let val_int = tag_value_str[1].parse::<u32>().ok();
        }
    }
    return None;
}

// 8=FIX.4.2|9=156|35=D|34=124|49=ABC_DEFG04|52=20100208-18:51:42|56=CCG|115=XYZ|11=NF 0015/02082010|54=2|38=1000|55=RRC|40=2|44=55.36|59=0|1=ABC123ZYX|21=1|207=N|47=A|9487=CO|10=050|
