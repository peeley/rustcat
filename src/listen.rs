use std::net::{TcpListener, TcpStream};
use std::io::*;

pub fn listen(port: &String) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Listening on port {}", port);
    loop {
        for stream in listener.incoming() {
            println!("Incoming connection...");
            handle_incoming(&mut stream.unwrap());
        }
    }
}

fn handle_incoming(stream: &mut TcpStream) {
    let mut incoming;
    loop {
        incoming = [0; 1024];
        stream.read(&mut incoming)
            .expect("UNABLE TO DECODE INCOMING MESSAGE");
        println!("{}", String::from_utf8_lossy(&incoming));
        stream.write(&incoming).unwrap();
        stream.flush().unwrap();
    }
}
