use std::collections::HashMap;

use crate::message::*;
use crate::session::*;

const ACCEPTOR_CONN_TYPE: &'static str = "acceptor";
const INITIATOR_CONN_TYPE: &'static str = "initiator";

pub trait Connecter {
    fn start();
    fn stop();
}

#[derive(Debug)]
pub enum ConnectionType {
    ACCEPTOR,
    INITIATOR,
}

#[derive(Debug)]
pub struct SocketConnector {
    connection_type: ConnectionType,
    session_map: HashMap<String, Session>,
}

impl SocketConnector {
    pub fn send(&self, msg: Message) {
        println!("{}", msg);
    }

    pub fn recv(&self) -> Message {
        Message::new()
    }

    pub fn set_connection_type(&mut self, con_ty: &String) {
        if con_ty.eq_ignore_ascii_case(ACCEPTOR_CONN_TYPE) {
            self.connection_type = ConnectionType::ACCEPTOR;
        } else if con_ty.eq_ignore_ascii_case(INITIATOR_CONN_TYPE) {
            self.connection_type = ConnectionType::INITIATOR;
        } else {
            panic!(format!("Invalid connection type param. Only {} and {} are allowed", ACCEPTOR_CONN_TYPE, INITIATOR_CONN_TYPE));
        }
    }

    pub fn set_session(&mut self, sid: String, session: Session) {
        self.session_map.insert(sid, session);
    }

    pub fn create_sessions(&mut self, config: &SessionConfig) {
        let default_session_setting = config.default_setting();
        
    }
}
