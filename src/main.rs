/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

use std::net::TcpStream;
use std::io::*;
use std::thread;
use std::time::Duration;

fn main() {
    write_loop().unwrap();
}

fn write_loop() -> std::io::Result<()> {
    let mut stream = connect();
    let mut query = String::new();
    let buffer_length = 512;
    let mut buffer = [0u8; 512];
    assert_eq!(buffer_length, buffer.len());
    let mut response = Vec::new();
    loop { // REPL Loop
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut query).expect("Error reading query");
        if query.trim().is_empty() {
            continue;
        }
        println!("'{}'", query);
        stream.write(query.as_bytes()).expect("Connection closed.");
        thread::sleep(Duration::from_millis(1));
        loop { // Keep reading until message is done being read
            let n_bytes = stream.read(&mut buffer).unwrap();
            response.extend_from_slice(&buffer);
            buffer = [0; 512];
            println!("Read {} bytes in response: ", n_bytes);
            if n_bytes == 0 || n_bytes < buffer_length {
                break;
            }
        }
        println!("{}\n", String::from_utf8_lossy(&response));
        stream.flush()?;
        response.clear();
        query.clear();
    }
}

fn connect() -> TcpStream {
    println!("Enter address (host:port): ");
    //let mut hostname = String::new();
    //std::io::stdin().read_line(&mut hostname).unwrap();
    let hostname = String::from("localhost:6969");
    let stream = TcpStream::connect(hostname.trim())
        .expect("Could not connect to address.");
    println!("Connected!");
    return stream;
}
