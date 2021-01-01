use crate::new_data_dictionary::*;
use crate::message::SOH;
use crate::quickfix_errors::*;

pub fn validate_tag(msg: &str, dict: &DataDict) -> Result<(), NewFixError> {
    // validate that tag is correct according to data_dictionary
    // and value is in permissible range
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

        match dict.is_tag_valid_for_message(tag, msg_type) {
            Ok(v) => Ok(()),
            Err(e) => Err(e),
        };
    }
    Ok(())
}

pub fn validate_checksum(message: &str) -> Result<(), NewFixError> {
    let tag_vec: Vec<&str> = message.split(SOH).collect();
    let recvd_checksum_field: Vec<&str> = tag_vec[tag_vec.len()-1].split('=').collect();
    if !recvd_checksum_field[0].starts_with("10=") {
        return Err(NewFixError {
            kind: NewFixErrorKind::RequiredTagMissing,
        })
    }
    let mut calc_checksum = 0u32;
    for tag in tag_vec {
        calc_checksum = calc_checksum + tag.as_bytes().len() as u32 + 1;
    }
    let check_str = format!("{:0>3}", calc_checksum % 256);
    if check_str != recvd_checksum_field[1] {
        return Err(NewFixError {
            kind: NewFixErrorKind::InvalidCheckcksum
        })
    }
    Ok(())
}

pub fn validate_bodylength(message: &str) -> Result<(), NewFixError> {
    let mut body_len: u32 = 0;
    let all_tags: Vec<&str> = message.split(SOH).collect();
    for tag in &all_tags[2..all_tags.len()-1] {
        // adding 1 for SOH character
        body_len = body_len + (*tag).len() as u32 + 1;

    }
    let received_body_len = get_body_length(message)?;
    if received_body_len == body_len {
        return Ok(())
    }
    return Err(NewFixError {
        kind: NewFixErrorKind::InvalidBodyLength
    })
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

pub fn get_body_length(msg_str: &str) -> Result<u32, NewFixError> {
    for tag_value in msg_str.split(SOH) {
        if tag_value.starts_with("9=") {
            let tag_value_str: Vec<&str> = tag_value.split('=').collect();
            match tag_value_str[1].parse::<u32>() {
                Ok(num) => return Ok(num),
                Err(e) => return Err(NewFixError {
                        kind: NewFixErrorKind::ParseError(
                            FixTypeFieldParseErrorKind::NotInt),
                })
            }
        }
    }
    // tag not found, raise error
    Err(NewFixError {
        kind: NewFixErrorKind::RequiredTagMissing
    })
}

// 8=FIX.4.2|9=156|35=D|34=124|49=ABC_DEFG04|52=20100208-18:51:42|56=CCG|115=XYZ|11=NF 0015/02082010|54=2|38=1000|55=RRC|40=2|44=55.36|59=0|1=ABC123ZYX|21=1|207=N|47=A|9487=CO|10=050|
