#![feature(io)]
#![feature(env)]
#![feature(net)]

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::string::String;

fn main() {

    let connection_string = format!("{}:{}", "127.0.0.1", "8080");
    send_receive_echo_message(&connection_string, "hello");
}

fn send_receive_echo_message(addr: &str, message: &str) {
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => {
            stream
        }
        Err(e) => {
            println!("Error opening TCP Stream: {}", e);
            return;
        }
    };

    let _ = stream.write_all(message.as_bytes());
    // stream.flush();


    let _ = stream.write_all(" test".as_bytes());
    // stream.flush();
}
