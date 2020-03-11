#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
use std::io::{self, BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::{str, thread, time};

use regex::bytes::Regex;

const SOH: char = '\u{01}';
const MAX_BUFF_SIZE: usize = 8 * 1024;

pub trait MsgRead {
    type Output;
    type MsgDelim;

    fn read_message(&mut self, delim: Self::MsgDelim) -> Result<Self::Output, std::io::Error>;
}

pub struct MsgReader<B> {
    inner: B,
    aux_buff: Vec<u8>,
}

impl<B: BufRead> MsgReader<B> {
    pub fn with_capacity(reader: B, aux_cap: usize) -> Self {
        Self {
            inner: reader,
            aux_buff: Vec::with_capacity(aux_cap),
        }
    }

    pub fn new(reader: B) -> Self {
        MsgReader::with_capacity(reader, MAX_BUFF_SIZE)
    }

    pub fn read_message(&mut self) -> Result<std::borrow::Cow<str>, io::Error> {
        lazy_static! {
            static ref EOM_RE: Regex =
                Regex::new(format!("{}10=\\d{{3}}{}", SOH, SOH).as_str()).unwrap();
        }
        loop {
            let (found, bytes_used) = {
                let avail_bytes = match self.inner.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                match EOM_RE.find(avail_bytes) {
                    Some(re_match) => {
                        self.aux_buff.extend_from_slice(&avail_bytes[..re_match.end()]);
                        (true, re_match.end())
                    }
                    None => {
                        self.aux_buff.extend_from_slice(avail_bytes);
                        (false, avail_bytes.len())
                    }
                }
            };
            if found {
                //let ret_string = str::from_utf8(&self.aux_buff).unwrap().to_owned();
                //self.aux_buff.clear();
                self.inner.consume(bytes_used);
                //return Ok(ret_string);
                let res = String::from_utf8_lossy(self.aux_buff.as_slice());
                self.aux_buff.clear();
                return Ok(res);
            } else if bytes_used == 0 {
                // return empty string
                //return Ok(String::new());
            }
        }
    }
}

fn main() {
    test_buffreader("127.0.0.1:4375");
}

fn test_buffreader<A: ToSocketAddrs>(sock: A) {
    let listener = TcpListener::bind(sock).expect("could not bind to socket");
    let (stream, sock_addr) = listener.accept().unwrap();
    let mut buf_reader = BufReader::new(stream);
    loop {
        let mut sbuf = String::new();
        buf_reader.read_line(&mut sbuf);
        println!("rcvd: {}", sbuf);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
