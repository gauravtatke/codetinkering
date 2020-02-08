extern crate rand;

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
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

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:10114").expect("could not connect");
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
