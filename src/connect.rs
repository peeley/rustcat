use std::net::TcpStream;
use std::io::*;
use std::time::Duration;
use std::process::exit;

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
    let mut buffer;
    let mut response = Vec::new();
    let mut query = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);
    loop {
        loop {
            buffer = [0; 1024];
            match reader.read(&mut buffer) {
                Ok(n_bytes) => { 
                    response.extend_from_slice(&buffer);
                    if n_bytes == 0 {
                        break;
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => break,
                    ErrorKind::BrokenPipe => {
                        println!("Connection closed.");
                        exit(1);
                    }
                    _ => panic!(e),
                }
            }
        }
        println!("{}\n", String::from_utf8_lossy(&mut response));
        response.clear();
        read_query(&mut query);
        query.push('\n');
        let n_bytes = writer.write(query.as_bytes())
            .expect("Connection closed.");
        println!("{} bytes written, connection closed.", n_bytes);
        writer.flush().expect("Unable to flush stream.");
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
