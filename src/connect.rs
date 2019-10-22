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
    let mut response = String::new();
    let mut query = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);
    loop {
        read_query(&mut query);
        query.push('\n');
        writer.write(query.as_bytes()).expect("Connection closed.");
        writer.flush()?;
        let mut n_bytes = reader.read_line(&mut response).unwrap();
        while n_bytes != 0 {
            println!("{}, {} new bytes", response, n_bytes);
            n_bytes = reader.read_line(&mut response).unwrap();
        }
        if response.len() == 0 {
            println!("Empty response, connection has been closed.");
            break Ok(());
        }
        println!("{}\n", /*String::from_utf8_lossy*/(&mut response));
        response.clear();
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
