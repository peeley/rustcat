/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

use std::net::TcpStream;
use std::io::*;

fn main() {
    write_loop().unwrap();
}

fn write_loop() -> std::io::Result<()> {
    println!("Enter address (host:port): ");
    let mut hostname = String::new();
    std::io::stdin().read_line(&mut hostname).unwrap();
    let mut stream = TcpStream::connect(hostname.trim())
        .expect("Could not connect to address.");
    println!("Connected!");
    let mut query = String::new();
    let buffer_length = 512;
    let mut buffer = [0u8; 512];
    assert_eq!(buffer_length, buffer.len());
    let mut response = Vec::new();
    loop{
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut query).expect("Error reading query");
        stream.write(query.as_bytes()).expect("Connection closed.");
        loop{
            let n_bytes = stream.read(&mut buffer).unwrap();
            println!("Read {} bytes: ", n_bytes);
            response.extend_from_slice(&buffer);
            buffer = [0; 512];
            if n_bytes == 0 {
                break;
            }
        }
        println!("{}\n", String::from_utf8_lossy(&response));
        stream.flush()?;
        response.clear();
        query.clear();
    }
}
