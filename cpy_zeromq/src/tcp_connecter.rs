use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use result::{ZmqResult, ZmqError};
use socket_base::SocketMessage;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::duration::Duration;
use options::Options;
use std::thread;
use std::net::{TcpStream};
use stream_engine::StreamEngine;

pub struct TcpConnecter {
    sender_to_socket: Sender<ZmqResult<SocketMessage>>,
    addr: SocketAddr,
    options: Arc<RwLock<Options>>,
    current_reconnect_ivl: Duration,
}

impl TcpConnecter {
	fn run(&mut self) -> Result<(), ZmqResult<SocketMessage>> {
		loop {
			match TcpStream::connect(format!("{}:{}", self.add.ip(), self.addr.port()).as_slice()) {
				Ok(stream) => {
					if self.sender_to_socket.send(Ok(SocketMessage::Ping)).is_err() {
						return Ok(());
					}

					let (tx, rx) = channel();
					StreamEngine::spawn_new(
						stream, self.options.clone(), self.sender_to_socket.clone(), Some(tx));
					let _= rx.recv();
				},
				Err(e) => {
					try!(self.sender_to_socket.send(Err(ZmqError::from_io_error(e))))
				},
			}
		}
	}

	pub fn spawn_new(addr: SocketAddr, sender: Sender<ZmqResult<SocketMessage>>, options: Arc<RwLock<Options>>) {
		thread::spawn(move || {
			let reconnect_ivl = options.read().reconnect_ivl;
			let mut connecter = TcpConnecter {
				sender_to_socket: sender,
				addr: addr,
				options: options,
				current_reconnect_ivl: reconnect_ivl,
			};
			let _ = connecter.run();
		});
	}
}