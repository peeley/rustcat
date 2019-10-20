/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

use std::io::*;
use std::thread;
use std::time::Duration;
use std::env;
mod connect;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        panic!("Use command with: rustcat <ipaddr> <port>");
    }
    let host: &String = &args[1];
    let port: &String = &args[2];
    write_loop(&host, &port).unwrap();
}

fn write_loop(host: &String, port: &String) -> std::io::Result<()> {
    let mut stream = connect::connect(&host, &port);
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
        query.push('\n');
        println!("'{}'", query);
        let w_bytes = stream.write(query.as_bytes())
            .expect("Connection closed.");
        println!("Wrote {} bytes", w_bytes);
        thread::sleep(Duration::from_millis(1));
        loop { // Keep reading until message is done being read
            let r_bytes = stream.read(&mut buffer).unwrap();
            response.extend_from_slice(&buffer);
            buffer = [0; 512];
            println!("Read {} bytes in response: ", r_bytes);
            if r_bytes == 0 /*|| r_bytes < buffer_length*/ {
                break;
            }
        }
        println!("{}\n", String::from_utf8_lossy(&response));
        stream.flush()?;
        response.clear();
        query.clear();
    }
}

