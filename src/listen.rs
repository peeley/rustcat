use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read, Write};
use std::process::{Command, Stdio, Child};

pub fn listen_loop(port: &str, command_name: Option<&str>) -> Result<()> {
    let command: Option<Child> = match command_name {
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
            handle_incoming(&mut stream.unwrap(), command);
        }
    }
}

fn handle_incoming(stream: &mut TcpStream, command: Option<Child>) {
    let mut incoming;
    loop {
        incoming = [0; 1024];
        stream.read(&mut incoming)
            .expect("UNABLE TO DECODE INCOMING MESSAGE");
        println!("{}", String::from_utf8_lossy(&incoming));
        match command {
            Some(c) => c.stdin.unwrap().write_all(&incoming).unwrap(),
            None => {
                stream.write(&incoming).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
