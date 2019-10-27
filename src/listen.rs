use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read, Write};
use std::process::{Command, Stdio, Child};

use rustcat::lib;

pub fn listen_loop(port: &str, command_name: Option<&str>) -> Result<()> {
    let mut command: Option<Child> = match command_name {
        Some(name) => Some(Command::new(name)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn().unwrap()),
        None => None
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Listening on port {}", port);
    loop {
        for stream in listener.incoming() {
            println!("Incoming connection...");
            handle_incoming(&mut stream.unwrap(), &mut command);
        }
    }
}

fn handle_incoming(stream: &mut TcpStream, command: &mut Option<Child>) {
    let mut incoming;
    let mut output: Vec<u8>;
    loop {
        incoming = [0 as u8; 1024];
        stream.read(&mut incoming)
            .expect("UNABLE TO DECODE INCOMING MESSAGE");
        println!("{}", String::from_utf8_lossy(&incoming));
        match command {
            Some(c) => {
                println!("Writing to command...");
                c.stdin.as_mut().unwrap().write_all(&incoming).unwrap();
                println!("Reading response...");
                output = lib::read_til_empty(&mut c.stdout.as_mut().unwrap());
                println!("{}", String::from_utf8_lossy(&output));
            }
            None => {
                output = incoming.to_vec();
            }
        }
        stream.write(&output).unwrap();
        stream.flush().unwrap();
    }
}
