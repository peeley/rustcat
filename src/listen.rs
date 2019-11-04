use std::net::{TcpListener};
use std::io::{Result, Write, BufReader, BufRead, BufWriter};
use std::process::{Command, Stdio, Child};

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
    let mut output: String;
    let mut incoming = String::new();
    loop {
        for stream_opt in listener.incoming() {
            println!("Incoming connection...");
            let stream = stream_opt.unwrap();
            let mut query_reader = BufReader::new(&stream);
            let mut response_writer = BufWriter::new(&stream);
            query_reader.read_line(&mut incoming).unwrap();
            println!("Query: {}", &incoming);
            match command.as_mut() {
                Some(mut c) => { 
                    output = handle_command(&incoming.as_bytes(), &mut c);
                }
                None => {
                    output = incoming.clone();
                }
            }
            response_writer.write(&output.as_bytes()).unwrap();
            response_writer.flush().unwrap();
            incoming.clear();
            output.clear();
        }
    }
}

fn handle_command(incoming: &[u8], command: &mut Child) -> String {
    let mut output = String::new();
    let reader = BufReader::new(command.stdout.as_mut().unwrap());
    println!("Writing to command...");
    command.stdin.as_mut().unwrap().write_all(&incoming).unwrap();
    command.stdin.as_mut().unwrap().flush().unwrap();
    println!("Reading response...");
    for line_opt in reader.lines(){
        let line = line_opt.unwrap();
        output.push_str(&line);
        output.push('\n');
        if line.len() <= 1 {
            break;
        }
        println!("{}", &output);
    }
    println!("{}", &output);
    return output;
}
