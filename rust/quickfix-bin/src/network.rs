use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::str::{self, FromStr};
use std::thread;

use regex::Regex;

use crate::message::*;
use crate::session::*;

const ACCEPTOR_CONN_TYPE: &str = "acceptor";
const INITIATOR_CONN_TYPE: &str = "initiator";

pub trait Connecter {
    fn start(&self) -> Vec<thread::JoinHandle<()>>;
    fn stop();
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConnectionType {
    ACCEPTOR,
    INITIATOR,
}

#[derive(Debug)]
pub struct SocketConnector {
    connection_type: ConnectionType,
    session_map: HashMap<String, Session>,
    sockets: HashSet<SocketAddrV4>,
}

impl Default for SocketConnector {
    fn default() -> Self {
        Self {
            connection_type: ConnectionType::ACCEPTOR,
            session_map: HashMap::new(),
            sockets: HashSet::new(),
        }
    }
}
impl SocketConnector {
    pub fn new(config: &mut SessionConfig) -> Self {
        let mut socket_connector = SocketConnector::default();
        socket_connector.create_sessions(config);
        socket_connector
    }

    pub fn send(&self, msg: Message) {
        println!("{}", msg);
    }

    pub fn recv(&self) -> Message {
        Message::new()
    }

    pub fn set_connection_type(&mut self, con_ty: String) {
        if con_ty.eq_ignore_ascii_case(ACCEPTOR_CONN_TYPE) {
            self.connection_type = ConnectionType::ACCEPTOR;
        } else if con_ty.eq_ignore_ascii_case(INITIATOR_CONN_TYPE) {
            self.connection_type = ConnectionType::INITIATOR;
        } else {
            panic!(format!(
                "Invalid connection type param. Only {} and {} are allowed",
                ACCEPTOR_CONN_TYPE, INITIATOR_CONN_TYPE
            ));
        }
    }

    pub fn get_connection_type(&self) -> ConnectionType {
        self.connection_type
    }

    pub fn set_session(&mut self, sid: String, session: Session) {
        self.session_map.insert(sid, session);
    }

    pub fn create_sessions(&mut self, config: &mut SessionConfig) {
        let default_setting = config.default_setting().clone();
        let conn_type = default_setting.get_connection_type();
        // if conn_type.is_none() {
        //     panic!("No connection type provided");
        // }
        // self.set_connection_type(conn_type.unwrap());
        self.set_connection_type(conn_type);
        let settings_vec: Vec<&mut SessionSetting> =
            config.iter_mut().filter(|s| !s.is_empty()).collect();
        for stng in settings_vec.into_iter() {
            println!("setting {:?}", stng);
            let merge_set = stng.merge_setting(&default_setting);
            let new_session = Session::with_settings(&merge_set);
            self.set_session(new_session.session_id.to_string(), new_session);
            let sock_addr: SocketAddrV4;
            if self.get_connection_type() == ConnectionType::ACCEPTOR {
                sock_addr = SocketAddrV4::new(
                    Ipv4Addr::LOCALHOST,
                    stng.get_socket_accept_port().expect("no port specified"),
                );
                self.sockets.insert(sock_addr);
            } else {
                let ipv4 = Ipv4Addr::from_str(
                    stng.get_socket_connect_host()
                        .expect("no host specified")
                        .as_ref(),
                )
                .expect("cannot parse host to Ipv4Addr");
                sock_addr = SocketAddrV4::new(
                    ipv4,
                    stng.get_socket_connect_port().expect("no port specified"),
                );
                self.sockets.insert(sock_addr);
            }
        }
    }
}

struct FixReader<B: BufRead> {
    buf_reader: B,
}

impl<B: BufRead> FixReader<B> {
    fn new(buf_read: B) -> Self {
        FixReader {
            buf_reader: buf_read,
        }
    }

    fn read_message(&mut self, buff: &mut String) -> std::io::Result<usize> {
        // regular expression for end of fix message
        lazy_static! {
            static ref EOM_RE: Regex =
                // Regex::new(format!("{}10=\\d{{{}}}{}", SOH, 3, SOH).as_str()).unwrap();
                Regex::new(format!("{}10=\\d+{}", SOH, SOH).as_str()).unwrap();
        }
        let bytes_used = {
            let data_bytes = match self.buf_reader.fill_buf() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            let str_data = str::from_utf8(data_bytes).unwrap();
            println!("str_data {}", str_data);
            match EOM_RE.find(str_data) {
                Some(mat) => {
                    buff.push_str(&str_data[..mat.end()]);
                    mat.end()
                }
                None => 0,
            }
        };
        println!("bytes used {}", bytes_used);
        self.buf_reader.consume(bytes_used);
        Ok(bytes_used)
    }
}

impl Connecter for SocketConnector {
    fn start(&self) -> Vec<thread::JoinHandle<()>> {
        let mut join_handles: Vec<thread::JoinHandle<()>> = Vec::new();
        for socket in &self.sockets {
            println!("Socket {}", socket);
            let listener = TcpListener::bind(socket).expect("could not bind to socket");
            let new_thread = thread::Builder::new()
                .name(format!("thread for socket {}", socket))
                .spawn(move || {
                    let (stream, _) = listener.accept().unwrap();
                    let mut buff = String::with_capacity(512);
                    let mut fix_reader = FixReader::new(BufReader::new(stream));
                    loop {
                        buff.clear();
                        fix_reader.read_message(&mut buff);
                        println!(
                            "from thread id = {:?}, data = {}",
                            thread::current().id(),
                            buff
                        );
                        thread::sleep(std::time::Duration::from_millis(5000));
                    }
                })
                .unwrap();
            join_handles.push(new_thread);
        }
        join_handles
    }

    fn stop() {}
}

#[cfg(test)]
mod networkio_tests {
    use super::*;
    use crate::message::*;
    use crate::session::*;

    fn test_message() -> String {
        let mut msg = Message::new();
        msg.header_mut().set_string(8, "FIX4.3".to_string());
        msg.header_mut().set_string(49, "Gaurav".to_string());
        msg.header_mut().set_string(56, "Tatke".to_string());

        msg.body_mut().set_int(34, 8765);
        msg.body_mut().set_float(44, 1.87856);
        msg.body_mut().set_bool(654, true);
        msg.body_mut().set_char(54, 'b');
        msg.body_mut().set_string(1, "BOX_AccId".to_string());

        msg.trailer_mut().set_int(10, 101);
        msg.to_string()
    }

    #[test]
    fn io_test() {
        let mut session_config = SessionConfig::from_toml("src/FixConfig.toml");
        let mut acceptor = SocketConnector::new(&mut session_config);
        let mut stream = TcpStream::connect("127.0.0.1:10114").expect("could not connect");
        for i in 0..5 {
            let msg = test_message();
            println!("seinding message: {}", msg);
            stream.write(msg.as_bytes());
        }
    }
}
