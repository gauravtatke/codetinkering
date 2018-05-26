use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error, BufRead, BufReader};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from {}", stream.peer_addr()?);
    let mut buffer = String::with_capacity(1024);
    // let mut bufstream = BufReader::new(stream);
    loop {
        buffer.clear();
        let byte_read = stream.read_to_string(&mut buffer)?; // need to press ctrl+d in order to trigger EOF
        if byte_read == 0 {
            return Ok(());
        }
        println!("Received = {}", buffer);
        for token in buffer.split("^A") {
            println!("tag & value = {}", token);
        }
        stream.write(&"got the message".as_bytes())?;
    }
}

fn handle_client_with_buffer(stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from {}", stream.peer_addr()?);
    println!("using buffered version");
    let mut strbuf = String::new();
    let mut reader = BufReader::new(stream);
    loop {
        strbuf.clear();
        let byte_read = reader.read_line(&mut strbuf)?;
        if byte_read == 0 {
            return Ok(());
        }
        println!("received = {}", strbuf);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("could not bind");
    let mut count = 0;
    for mut stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if count % 2 == 0 {
                    thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
                }
                else {
                    thread::spawn(move || {
                    handle_client_with_buffer(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
                }
                
            }
            Err(err) => { eprintln!("failed {}", err);}        
        }
        count = count + 1;
    } 
}
