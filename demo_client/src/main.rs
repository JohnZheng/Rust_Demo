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
        stream.write_all(world.trim_right().as_bytes());
        stream.flush();
        loop {
            let read_bytes = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(e) => {
                    println!("Error reading the input: {}", e);
                    return;
                }
            };
            if read_bytes != 0 {
                println!("client: {} read_bytes {}", buf.to_vec().container_as_str().unwrap(), read_bytes);
                buf = [0u8; 128];
                break;
            }
        }
    }
}
