# rustcat
A full rewrite of the GNU NetCat networking tool in Rust.


The project aims to fully mirror and be interoperable with the original NetCat,
but this project is very much a work in progress.

## Install
Install the binary with:
```
git clone https://github.com/peeley/rustcat.git
cd rustcat
cargo install --path .
```
As indicated by cargo, ~/cargo/.bin must be added to PATH to execute programs
installed by cargo.

## Features
- Connect to other hosts over TCP and send raw bytes
- Listen for incoming TCP connections and read bytes
- Send incoming bytes to executables (-e option in NetCat). This is rightfully
  identified as a huge security vulnerability, but this project is aiming
  to be a pure rewrite rather than a fork and as such should include this
  option.
- Hexdump incoming/outgoing traffic

## TODO
- Connect/listen over UDP in addition to TCP
- Port scanning mode
