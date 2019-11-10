use std::net::{TcpListener};
use std::io::{Result, Write, BufReader, BufRead, BufWriter};
use std::process::{Command, Stdio, Child};
use rustcat::lib::{CliArgs, hexdump};

pub fn listen_loop(args: CliArgs) -> Result<()> {
    let mut command: Option<Child> = match args.command {
        Some(name) => Some(Command::new(name)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn().unwrap()),
        None => None
    };
    let port = args.port;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Listening on port {}", port);
    let mut output: String;
    let mut incoming = String::new();
    let outfile = args.output;
    loop {
        for stream_opt in listener.incoming() {
            println!("Incoming connection...");
            let stream = stream_opt.unwrap();
            let mut query_reader = BufReader::new(&stream);
            let mut response_writer = BufWriter::new(&stream);
            query_reader.read_line(&mut incoming).unwrap();
            if outfile.is_some() {
                hexdump(true, incoming.len(), incoming.as_bytes(),
                                outfile.as_ref().unwrap());
            }
            println!("Query: {}", &incoming);
            match command.as_mut() {
                Some(mut c) => { 
                    output = handle_command(&incoming.as_bytes(), &mut c);
                }
                None => {
                    output = incoming.clone();
                }
            }
            if outfile.is_some() {
                hexdump(false, output.len(), output.as_bytes(),
                                outfile.as_ref().unwrap());
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
    let mut reader = BufReader::new(command.stdout.as_mut().unwrap());
    println!("Writing to command...");
    command.stdin.as_mut().unwrap().write_all(&incoming).unwrap();
    command.stdin.as_mut().unwrap().flush().unwrap();
    println!("Reading response...");
    let mut line = String::new();
    loop {
        let n_bytes = reader.read_line(&mut line).unwrap();
        println!("Read {} bytes: {}", n_bytes, line);
        output.push_str(&line);
        output.push('\n');
        if n_bytes == 0 {
            break;
        }
    }
    println!("{}", &output);
    return output;
}
