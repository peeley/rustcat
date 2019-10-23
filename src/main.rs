/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

use std::env;
mod connect;
mod listen;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Use command with: rustcat <ipaddr> <port>");
    }
    let mode: &String = &args[1];
    let host: &String = &args[2];
    let port: &String = &args[3];
    match mode.as_ref() {
        "w" => connect::write_loop(&host, &port).unwrap(),
        "l" => listen::listen(&port).unwrap(),
        _ => panic!("Incorrect program args!"),
    }
}
