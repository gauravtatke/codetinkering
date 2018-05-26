//extern crate serde;

use serde::ser::{self, Serialize, Serializer, SerializeStruct};
use serde_derive::*;
use std::default::Default;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(tag="type")]
pub enum MessageType {
    HeartBeat,
    Logout,
    //ExecutionReport(u32) => 8,
    Logon,
    // NewOrderSingle(String) => "D",
    // MarketDataRequest(String) => "V"
    // MarketDataSnapshotFullRefresh(String) => "W"
}

impl MessageType {
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::HeartBeat => "0",
            MessageType::Logout => "5",
            MessageType::Logon => "A",
        }
    }
}

impl Default for MessageType {
    fn default() -> MessageType {
        MessageType::Logon
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag="98")]
pub enum EncryptMethod {
    Des = 2,
    PemDesMd5 = 6,
    PgpDesMd5 = 5,
    PkcsDes = 3,
    NoEncr = 0,
    Pkcs = 1,
    PgpDes = 4,
}

impl Default for EncryptMethod {
    fn default() -> EncryptMethod {
        EncryptMethod::NoEncr
    }
}

pub enum ResetSeqNumFlag {
    Yes,
    No,
}

impl Default for ResetSeqNumFlag {
    fn default() -> ResetSeqNumFlag {
        ResetSeqNumFlag::No
    }
}

impl ResetSeqNumFlag {
    pub fn as_str(&self) -> &str {
        match self {
            ResetSeqNumFlag::Yes => "Y",
            ResetSeqNumFlag::No => "N"
        }
    }
}


pub enum MsgDirection {
    Send,
    Recv,
}

impl MsgDirection {
    pub fn as_str(&self) -> &str {
        match self {
            MsgDirection::Send => "S",
            MsgDirection::Recv => "R"
        }
    }
}

#[derive(Default)]
pub struct Logon {
    pub message_type: MessageType,
    pub encrypt_method: EncryptMethod,
    pub heartbeat_int: u32,
    pub raw_data_length: Option<u32>,
    pub raw_data: Option<u32>,
    pub reset_seqnum_flag: Option<String>,
    pub max_msg_size: Option<u32>,
    pub no_msg_type: Option<u32>,
    pub no_msg_type_grp: Option<Vec<NoMsgTypeGroup>>,
    pub test_msg_indic: Option<bool>,
    pub username: Option<String>,
    pub password: Option<String>
}

impl Logon {
    pub fn new() -> Self {
        Logon {
            message_type: MessageType::Logon,
            encrypt_method: EncryptMethod::NoEncr,
            ..Default::default()
        }
    } 
}

impl Serialize for Logon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {   
        let mut s = serializer.serialize_struct("Logon", 12)?;
        s.serialize_field("35", &self.message_type)?;
        s.serialize_field("98", &self.encrypt_method)?;
        s.serialize_field("108", &self.heartbeat_int)?;
        s.serialize_field("95", &self.raw_data)?;
        s.serialize_field("96", &self.raw_data)?;
        s.serialize_field("141", &self.reset_seqnum_flag)?;
        s.serialize_field("554", &self.username)?;
        s.serialize_field("555", &self.password)?;
        s.serialize_field("383", &self.max_msg_size)?;
        s.serialize_field("384", &self.no_msg_type)?;
        s.serialize_field("repeatinggrpNOTWORKING", &self.password)?;
        s.serialize_field("464", &self.test_msg_indic)?;
        s.end()
    }
}



#[derive(Default)]
pub struct NoMsgTypeGroup {
    ref_msg_type: Option<String>,
    msg_direction: Option<char>,
}


