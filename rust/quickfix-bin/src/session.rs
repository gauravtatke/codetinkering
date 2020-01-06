use std::collections::VecDeque;
use std::fmt::{self, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};

use crate::message::*;
use crate::network::*;

use serde_derive::Deserialize;

const FIX42_BEGIN_STR: &'static str = "FIX.4.2";
const FIX43_BEGIN_STR: &'static str = "FIX.4.3";
const FIX44_BEGIN_STR: &'static str = "FIX.4.4";

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    default: SessionSetting,
    sessions: Option<Vec<SessionSetting>>,
}

impl SessionConfig {
    pub fn from_toml(config: &str) -> Self {
        let mut toml_file = File::open(config).expect("Could not find file");
        let mut contents = String::with_capacity(1024);
        toml_file
            .read_to_string(&mut contents)
            .expect("Could not read from file");
        let mut session_config: SessionConfig = toml::from_str(&contents).unwrap();
        if session_config.default.connection_type.is_none() {
            panic!("connection_type not present in default section. Valid values are acceptor or initiator")
        }
        if session_config.is_session_empty() {
            // initialize it with default
            session_config.sessions = Some(vec![session_config.default.clone()])
        }
        session_config
    }

    pub fn default_setting(&self) -> &SessionSetting {
        &self.default
    }

    //TODO: give correct implementation
    pub fn is_session_empty(&self) -> bool {
        if self.sessions.is_none() {
            return true;
        }
        self.sessions.as_ref().iter().any(|&s| !s.is_empty())
    }

    pub fn iter(&self) -> std::slice::Iter<SessionSetting> {
        self.sessions.as_ref().unwrap().iter()
    }
}

impl IntoIterator for &SessionConfig {
    type Item = SessionSetting;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let new_session_option = self.sessions.clone();
        new_session_option
            .unwrap_or_else(|| vec![self.default.clone()])
            .into_iter()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SessionSetting {
    connection_type: Option<String>,
    begin_string: Option<String>,
    sender_compid: Option<String>,
    target_compid: Option<String>,
    socket_accept_port: Option<u16>,
    socket_connect_host: Option<String>,
    socket_connect_port: Option<u16>,
}

impl SessionSetting {
    pub fn is_empty(&self) -> bool {
        // returns true if all variables are None,
        // false otherwise
        true
    }
}

#[derive(Debug)]
pub struct SessionId {
    begin_string: String,
    sender_compid: String,
    target_compid: String,
    id: String,
}

impl SessionId {
    fn new(bgn_str: &str, sender: &str, target: &str) -> Self {
        // let mut id = String::with_capacity(16);
        // id.push_str(string: &str)
        let mut sid = SessionId {
            sender_compid: sender.to_owned(),
            target_compid: target.to_owned(),
            begin_string: bgn_str.to_owned(),
            id: String::new(),
        };

        sid.set_id();
        sid
    }

    fn set_id(&mut self) {
        let sid = self.to_string();
        self.id = sid;
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}->{}",
            self.begin_string, self.sender_compid, self.target_compid
        )
    }
}

#[derive(Debug)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug)]
pub struct Session {
    session_id: String,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    io_conn: Option<SocketConnector>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            heartbeat_intrvl: 30,
            is_active: false,
            reset_on_disconnect: false,
            reset_on_logon: true,
            msg_q: VecDeque::with_capacity(16),
            state: SessionState::new(),
            io_conn: None,
        }
    }
}

impl Session {
    fn new(sid_str: String) -> Self {
        Self {
            session_id: sid_str,
            ..Default::default()
        }
    }

    // fn with_default_settings(default_setting: &SessionSetting) -> Self {

    // }

    fn with_sessionid(sid: &SessionId) -> Self {
        Self {
            session_id: sid.to_string(),
            ..Default::default()
        }
    }

    fn set_socket_connector(&mut self, conn: SocketConnector) {
        self.io_conn = Some(conn);
    }

    fn send_msg(&mut self, msg: Message) {
        if let Some(con) = self.io_conn.as_ref() {
            con.send(msg);
        } else {
            self.msg_q.push_back(msg);
        }
    }

    fn recv_msg(&self) -> Message {
        if let Some(con) = self.io_conn.as_ref() {
            con.recv()
        } else {
            Message::new()
        }
    }
}

#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn session_config_test() {
        let session_config = SessionConfig::from_toml("src/FixConfig.toml");
        println!("{:?}", session_config);
    }
}
