#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod application;
mod message;
mod network;
mod quickfix_errors;
mod session;
mod types;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use crate::application::*;
use crate::message::store::*;
use crate::message::*;
use crate::network::*;
use crate::session::*;
use crate::types::*;

fn test_message_1() -> String {
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
    // assert!(!msg.to_string().is_empty());
}

fn send_message(stream: &mut TcpStream, input_str: String) {
    stream
        .write(input_str.as_bytes())
        .expect("could not write to output stream");
    // loop {
    //     let mut buffer: Vec<u8> = Vec::new();
    //     stream.write(input_str.as_bytes()).expect("could not write to output stream");
    //     let mut reader = BufReader::new(&stream);
    //     reader.read_until(b'\n', &mut buffer).expect("could not read into buffer");
    //     print!("{}", str::from_utf8(&buffer).expect("could not write buffer as string"));
    // }
}

fn main() {
    println!("running main");
    let mut session_config = SessionConfig::from_toml("src/FixConfig.toml");
    let mut message_store = DefaultMessageStore::new();
    let mut log_store = DefaultLogStore::new();
    let application = DefaultApplication::new();
    let acceptor = SocketConnector::new(
        &mut session_config,
        &mut message_store,
        &mut log_store,
        application,
    );
    let handles = acceptor.start();
    for handle in handles {
        handle.join().expect("thread joined has panicked");
    }
}
