use std::net::TcpStream;
use std::io::*;
use std::time::Duration;

use rustcat::lib;

fn connect(host: &String, port: &String) -> TcpStream {
    let conn_string = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&conn_string)
        .expect("Could not connect to address.");
    println!("Connected to {}", conn_string);
    stream.set_read_timeout(Some(Duration::from_millis(150))).unwrap();
    return stream;
}

pub fn write_loop(args: lib::CliArgs) -> std::io::Result<()> {
    let host = args.host;
    let port = args.port;
    let stream = connect(&host, &port);
    let mut response: Vec<u8>;
    let mut query = String::new();
    let outfile = args.output;
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);
    loop {
        response = lib::read_til_empty(&mut reader);
        response.retain(|&x| x != 0x00);
        println!("{}\n", String::from_utf8_lossy(&mut response));
        if outfile.is_some() {
            lib::hexdump(true, response.len(), 
                            &response.as_slice(), outfile.as_ref().unwrap());
        }
        response.clear();
        read_user_query(&mut query);
        if outfile.is_some() {
            lib::hexdump(false, query.len(), 
                            &query.as_bytes(), outfile.as_ref().unwrap());
        }
        query.push('\n');
        let n_bytes = writer.write(query.as_bytes())
            .expect("Unable to write query, connection closed.");
        if n_bytes == 0{
            println!("No bytes written, connection closed.");
            break Ok(());
        }
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
