use std::net::TcpStream;
use std::io::*;

fn connect(host: &String, port: &String) -> TcpStream {
    let conn_string = format!("{}:{}", host, port);
    let stream = TcpStream::connect(conn_string.trim())
        .expect("Could not connect to address.");
    println!("Connected to {}", conn_string);
    return stream;
}

pub fn write_loop(host: &String, port: &String) -> std::io::Result<()> {
    let stream = connect(&host, &port);
    stream.set_nodelay(true).expect("set_nonblocking call failed");
    let mut buffer = [0 as u8; 1024];
    let mut response = Vec::new();
    let mut query = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);
    loop {
        loop {
            let n_bytes = reader.read(&mut buffer).unwrap();
            println!("{}, {} new bytes", String::from_utf8_lossy(&response), 
                n_bytes);
            response.extend_from_slice(&buffer);
            if n_bytes < 1024 {
                break;
            }
        }
        println!("{}\n", String::from_utf8_lossy(&mut response));
        response.clear();
        read_query(&mut query);
        query.push('\n');
        writer.write(query.as_bytes()).expect("Connection closed.");
        writer.flush()?;
        query.clear();
    }
}

fn read_query(mut query: &mut String) {
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut query).expect("Error reading query");
    if query.trim().is_empty() {
        query.clear();
        read_query(&mut query);
    }
}
