pub mod lib{
    use std::io::{Read, ErrorKind};
    use std::process::exit;
// The Read trait implements read_to_end and read_to_string functions which
// read from a buffer until EOF is reached, but for applications like output
// piped to stdout there is no EOF, so the functions hangs until timeout. This
// function allows a buffer to be fully read to the end, but instead of 
// expecting EOF the read completes when there are 0 bytes left to read.
    pub fn read_til_empty<T: Read>(buffer: &mut T) -> Vec<u8> {
        let mut total: Vec<u8> = Vec::new();
        let mut temp = [0 as u8; 1024];
        loop{
            match buffer.read(&mut temp) {
                Ok(n_bytes) => { 
                    println!("{} bytes read...", n_bytes);
                    total.extend_from_slice(&temp);
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
            temp = [0; 1024];
        }
        return total;
    }
}
