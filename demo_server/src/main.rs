#![feature(io)]
#![feature(env)]
#![feature(net)]

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::old_path::BytesContainer;

fn main() {
    let connection_string = format!("{}:{}", "127.0.0.1", "8080");
    start_server(&connection_string);
}

fn start_server(addr: &str) {
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => {
            listener
        },
        Err(e) => {
            println!("Error creating TCP Connection listener: {}", e);
            return;
        }
    };

    fn handle_client_read(stream: &mut TcpStream, tx: Sender<Vec<u8>>) {
        let mut buf = [0u8; 128];
        loop {
            let read_bytes = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(e) => {
                    println!("Error reading the input: {}", e);
                    return;
                }
            };
            if (read_bytes != 0) {
                tx.send(buf.to_vec());
                buf = [0u8; 128];
            }
        }
    }

    fn handle_client_write(stream: &mut TcpStream, rx: Receiver<Vec<u8>>) {
        loop {
            let msg = rx.recv().unwrap();
            let recvMsg = format!("{}", msg.container_as_str().unwrap());
            println!("server: {}", recvMsg );
            match stream.write_all(recvMsg.trim_right().container_as_bytes()) {
                Ok(_) => {},
                Err(_) => {println!("Error!!!");}
            }
            stream.flush();
        }
    }

    for stream in listener.incoming() {
    	match stream {
    		Ok(stream) => {
                let mut read_stream = stream.try_clone().unwrap();
                let (tx, rx) = channel();
    			thread::spawn(move || {
    				handle_client_read(&mut read_stream, tx);
    			});
                let mut write_stream = stream.try_clone().unwrap();
                thread::spawn(move || {
                    handle_client_write(&mut write_stream, rx);
                });
    		},
    		Err(e) => {
    			println!("Failed to accept connection: {}", e);
    			return;
    		}
    	}
    }
}   
