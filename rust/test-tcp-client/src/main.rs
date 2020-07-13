#![allow(unused_imports)]
#![allow(dead_code)]

extern crate rand;

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::str;
use std::{thread, time};

use rand::prelude::*;
pub const SOH: char = '\u{01}';

fn create_fix_message() -> String {
    let mut rng = rand::thread_rng();
    let mut msg = String::with_capacity(64);
    for i in 0..10 {
        let tag: u16 = rng.gen_range(1, 500);
        let value: u64 = rng.gen_range(1000, 10000);
        let s = format!("{}={}", tag, value);
        msg.push_str(s.as_str());
        msg.push(SOH);
    }
    let trailer_value: u16 = rng.gen();
    let trailer = format!("10={}", trailer_value);
    msg.push_str(trailer.as_str());
    msg.push(SOH);
    msg
}

fn send_fix_message<A: ToSocketAddrs>(addrs: A) -> io::Result<()> {
    let mut stream = TcpStream::connect(addrs).expect("could not connect");
    let std_in = io::stdin();
    let mut buff = String::new();
    loop {
        buff.clear();
        println!("send message [y/n]");
        std_in.read_line(&mut buff);
        
        if buff.starts_with("y") || buff.starts_with("Y") {
            let msg = create_fix_message();
            println!("sending string: {}", msg);
            stream.write(msg[..msg.len()/2].as_bytes());
            thread::sleep(time::Duration::from_millis(5000));
            stream.write(msg[(msg.len()/2)+1..].as_bytes());
        } else {
            return Ok(());
        }
        // thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // send_random_bytes("127.0.0.1:4375");
    match start_server() {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

fn send_random_bytes<A: ToSocketAddrs>(addr: A) {
    let mut stream = TcpStream::connect(addr).expect("could not connect");
    loop {
        let buff = ['c' as u8; 16];
        stream.write(&buff);
        stream.write(&[b'\n']);
        thread::sleep(time::Duration::from_millis(5000));
    }
}

fn start_server() -> io::Result<usize> {
    let listener = TcpListener::bind("127.0.0.1:4378").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf_reader = BufReader::new(stream);
        loop {
            let byte_used = {
                let res = match buf_reader.fill_buf() {
                    Ok(n) => {
                        if n.len() == 0 {
                            println!("nothing came");
                            break;
                        } else {
                            println!("{}", str::from_utf8(n).unwrap());
                        }
                        n
                    },
                    Err(e) => {
                        println!("Error occured");
                        return Err(e);
                    }
                };
                res.len()
            };
            buf_reader.consume(byte_used);
        }
    }
    Ok(0 as usize)
}
