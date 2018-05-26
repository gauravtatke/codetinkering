use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
                                .expect("could not connect");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input)
                    .expect("could not read from stdin");
        stream.write(input.as_bytes())
                .expect("could not write to output stream");
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer)
                .expect("could not read into buffer");
        print!("{}", str::from_utf8(&buffer)
                        .expect("could not write buffer as string"));

    }
}
