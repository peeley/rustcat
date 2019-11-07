pub mod lib{
    use std::io::{Write, Read, ErrorKind};
    use std::process::exit;
    use std::fs::OpenOptions;
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

    pub fn hexdump(received: bool, n_bytes: usize, chars: &[u8], filename: &str) {
        let num_lines = (chars.len() / 16) as usize;
        let mut hexdump = String::new();
        let mode = if received { "Received" } else { "Sent" };
        hexdump.push_str(&format!("{} {} bytes to the socket\n", mode, n_bytes));
        let mut char_idx: usize;
        for line_num in 0..num_lines {
            hexdump.push_str(&format!("{:08X}  ", line_num*16));
            for char_offset in 0..16 {
                char_idx = line_num * 16 + char_offset;
                hexdump.push_str(&format!("{:02X} ", chars[char_idx]));
                if (char_offset+1) % 4 == 0 { hexdump.push(' '); }
            }
            hexdump.push('\t');
            for char_offset in 0..16 {
                char_idx = line_num * 16 + char_offset;
                if chars[char_idx] <= 126 && chars[char_idx] >= 32 {
                    hexdump.push(chars[char_idx] as char);
                }
                else{
                    hexdump.push('.');
                }
            }
            hexdump.push('\n');
        }
        let mut file = OpenOptions::new().create(true)
                                        .append(true)
                                        .open(filename).unwrap();
        file.write(hexdump.as_bytes())
            .expect("Error writing hexdump to file.");
        file.flush().unwrap();
    }
}
