#![feature(io)]
#![feature(env)]
#![feature(net)]

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::string::String;
use std::old_path::BytesContainer;

fn main() {

    let connection_string = format!("{}:{}", "127.0.0.1", "8080");
    send_receive_echo_message(&connection_string);
}

fn send_receive_echo_message(addr: &str) {
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => {
            stream
        }
        Err(e) => {
            println!("Error opening TCP Stream: {}", e);
            return;
        }
    };
    // let mut world = String::new();
    let mut buf = [0u8; 128];
    loop {
        let world = std::old_io::stdin().read_line().ok().expect("error input");
        stream.write_all(world.as_bytes());
        stream.flush();
        println!("==================");
        // stream.shutdown(Shutdown::Write);
        let read_bytes = match stream.read(&mut buf) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Error reading the input: {}", e);
                return;
            }
        };
        println!("client: {}", buf.to_vec().container_as_str().unwrap());
        buf = [0u8; 128];
        // stream.shutdown(Shutdown::Read);
    }
}
