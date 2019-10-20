/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

use std::env;
mod connect;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        panic!("Use command with: rustcat <ipaddr> <port>");
    }
    let host: &String = &args[1];
    let port: &String = &args[2];
    connect::write_loop(&host, &port).unwrap();
}
