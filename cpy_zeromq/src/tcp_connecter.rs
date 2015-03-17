use std::sync::mpsc::Sender;
use result::{ZmqResult};
use socket_base::SocketMessage;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::duration::Duration;
use options::Options;
use std::thread;
use std::net::{TcpStream};

pub struct TcpConnecter {
    sender_to_socket: Sender<ZmqResult<SocketMessage>>,
    addr: SocketAddr,
    options: Arc<RwLock<Options>>,
    current_reconnect_ivl: Duration,
}

impl TcpConnecter {
	fn run(&mut self) -> Result<(), ZmqResult<SocketMessage>> {
		loop {
			// FLAG DEV
			match TcpStream::connect() {
				Some() => expr,
			}
		}
	}

	pub fn spawn_new(add: SocketAddr, sender: Sender<ZmqResult<SocketMessage>>, options: Arc<RwLock<Options>>) {
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