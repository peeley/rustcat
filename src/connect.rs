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
    let mut stream = connect(&host, &port);
    let mut response = Vec::new();
    let mut query: String = String::new();
    loop {
        read_query(&mut query);
        query.push('\n');
        stream.write(query.as_bytes()).expect("Connection closed.");
        stream.read_to_end(&mut response).unwrap();
        if response.len() == 0 {
            println!("Empty response, connection has been closed.");
            break Ok(());
        }
        println!("{}\n", String::from_utf8_lossy(&response));
        stream.flush()?;
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
