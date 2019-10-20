use std::net::TcpStream;

pub fn connect(host: &String, port: &String) -> TcpStream {
    let conn_string = format!("{}:{}", host, port);
    let stream = TcpStream::connect(conn_string.trim())
        .expect("Could not connect to address.");
    println!("Connected to {}", conn_string);
    return stream;
}
