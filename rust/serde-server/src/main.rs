#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use::std::net::{TcpListener, TcpStream};
use::std::io::{stdin, BufRead, BufReader, Error, Write};
use::std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data: Vec<u8> = Vec::new();
    let mut stream = BufReader::new(stream);
    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data);
        if bytes_read == 0 {
            return Ok(());
        }
        let input: Point3D = serde_json::from_slice(&data)?;
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        write!(stream.get_mut(), "{}", f64::from(value).sqrt())?;
        write!(stream.get_mut(), "{}", "\n")?;
    }
}


fn main() {
    let args: Vec(_) = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide --client or --server as argument");
        std::process::exit();
        
        //server case
        if args[1] == "--server" {
            let listener = TcpListener::bind("0.0.0.0:8888").expect("could not bind");
            for stream in listener.incoming() {
                match stream {
                    Err(e) => eprintln!("failed: {}", e),
                    Ok(stream) => {
                        thread::spawn(move || {
                            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                        });
                    }

                }
            }
        }
        else if args[1] == "--client" {

        }
    }
}
