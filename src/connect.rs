use std::net::TcpStream;
use std::io::*;
use std::time::Duration;

use rustcat::lib;

fn connect(host: &String, port: &String) -> TcpStream {
    let conn_string = format!("{}:{}", host, port);
    let stream = TcpStream::connect(conn_string.trim())
        .expect("Could not connect to address.");
    println!("Connected to {}", conn_string);
    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    return stream;
}

pub fn write_loop(host: &String, port: &String) -> std::io::Result<()> {
    let stream = connect(&host, &port);
    let mut response: Vec<u8>;
    let mut query = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);
    loop {
        response = lib::read_til_empty(&mut reader);
        println!("{}\n", String::from_utf8_lossy(&mut response));
        response.clear();
        read_user_query(&mut query);
        query.push('\n');
        let n_bytes = writer.write(query.as_bytes())
            .expect("Connection closed.");
        println!("{} bytes written, connection closed.", n_bytes);
        writer.flush().expect("Unable to flush stream.");
        query.clear();
    }
}

fn read_user_query(mut query: &mut String) {
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut query).expect("Error reading query");
    if query.trim().is_empty() {
        query.clear();
        read_user_query(&mut query);
    }
}
