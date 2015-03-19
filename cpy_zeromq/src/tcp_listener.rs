use std::sync::mpsc::Sender;
use result::{ZmqResult, ZmqError};
use socket_base::SocketMessage;
use std::sync::{Arc, RwLock};
use options::Options;
use std::thread;
use std::net::{SocketAddr};
use std::net::TcpListener as TcpListenerSys;
use stream_engine::StreamEngine;

static ACCEPT_TIMEOUT: u64 = 1000;

pub struct TcpListener {
	listener: TcpListener,
    sender_to_socket: Sender<ZmqResult<SocketMessage>>,
    options: Arc<RwLock<Options>>,
}

impl TcpListener {
	pub fn run(&mut self) -> Result<(), ZmqResult<SocketMessage>> {
		loop {
			match self.listener.accept() {
				Ok(stream) => {
					if self.sender_to_socket.send(Ok(SocketMessage::Ping)).is_err() {
						return Ok(());
					}
					StreamEngine::spawn_new(stream, self.options.clone(), self.sender_to_socket.clone(), None);
				},
				Err(e) => {
					try!(self.sender_to_socket.send(Err(ZmqError::from_io_error(e))))
				},
			}
		}
	}

	pub fn spawn_new(addr: SocketAddr, sender: Sender<ZmqResult<SocketMessage>>, options: Arc<RwLock<Options>>) {
		thread::spawn(move || {
			let tcp = TcpListenerSys::bind(format!("{}:{}", addr.ip(), addr.port())).unwrap();
			let mut listener = TcpListener {
				listener: tcp,
				options: options,
				sender_to_socket: sender,
			};
			let _ = listener.run();
		});
		
	}
}