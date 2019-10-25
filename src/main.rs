/*
 *  RustCat - A port of NetCat to rust.
 *  Written by Noah Snelson, in the year of Our Lord 2019.
 */
#[macro_use]
extern crate clap;
mod connect;
mod listen;

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
    ).get_matches();
    if matches.is_present("listen"){
        let port = String::from(matches.value_of("listenport").unwrap());
        listen::listen(&port).unwrap();
    }
    else{
        let host = String::from(matches.value_of("host").unwrap());
        let port = String::from(matches.value_of("connectport").unwrap());
        connect::write_loop(&host, &port).unwrap();
    }
}
