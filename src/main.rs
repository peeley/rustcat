/*
 *  RustCat - A port of NetCat to Rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */

#[macro_use]
extern crate clap;
use std::fs::{File, remove_file};
mod connect;
mod listen;

use rustcat::lib::CliArgs;

fn main() {
    let matches = clap_app!(rustcat => 
        (version: "0.1.0")
        (author: "Noah Snelson <noah.snelson@protonmail.com>")
        (about: "Rewrite of the netcat tool in Rust.")
        (@arg listen: -l --listen conflicts_with[host] 
            "Listen for incoming connection on port.")
        (@arg host: +takes_value conflicts_with[listen]
            "IP or hostname to connect to.")
        (@arg listenport: -p --port +takes_value requires[listen]
            "Port to listen on.")
        (@arg connectport: +takes_value conflicts_with[listen]
            "Port to connect to.")
        (@arg command: -e --execute +takes_value requires[listen]
            "Pipe incoming queries to specified program.")
        (@arg output: -o --output +takes_value
            "Hexdump incoming and outgoing traffic to file.")
    ).get_matches();
    if matches.is_present("output") {
        let filename = matches.value_of("output").unwrap();
        match File::open(filename) {
            Ok(_) => remove_file(filename),
            Err(_) => Ok(()),
        }.unwrap();
    }
    if matches.is_present("listen"){
        let args = CliArgs{
            port: String::from(matches.value_of("listenport").unwrap()),
            host: String::new(),
            command: matches.value_of("command").map(String::from),
            output: matches.value_of("output").map(String::from),
        };
        listen::listen_loop(args).unwrap();
    }
    else{
        let args = CliArgs{
            port: String::from(matches.value_of("connectport").unwrap()),
            host: String::from(matches.value_of("host").unwrap()),
            command: matches.value_of("command").map(String::from),
            output: matches.value_of("output").map(String::from),
        };
        connect::write_loop(args).unwrap();
    }
}
