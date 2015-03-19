use result::{ZmqResult, ZmqError};
use socket_base::{SocketMessage};
use std::net::{TcpStream, Shutdown};
use std::sync::{Arc, RwLock};
use options::Options;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;
use consts::ErrorCode;
use v2_decoder::V2Decoder;
use v2_encoder::V2Encoder;
use msg::Msg;

const V2_GREETING_SIZE: uint = 12;
const NO_PROGRESS_LIMIT: uint = 1000;
const SIGNATURE_SIZE: uint = 10;
const ZMTP_1_0: u8 = 0;
const ZMTP_2_0: u8 = 1;
const REVISION_POS: uint = 10;

pub struct StreamEngine {
    sender_to_socket: Sender<ZmqResult<SocketMessage>>,
    stream: TcpStream,
    options: Arc<RwLock<Options>>,
    _death_notifier: Option<Sender<u8>>,
}

impl StreamEngine {
	fn run(&mut self) -> ZmqResult<()> {
		info!("Connection is made: {} -> {}", self.stream.socket_addr(), self.stream.peer_addr());

		let (bytes_tx, bytes_rx) = channel();
		let (waiter_tx, bytes_rx) = channel();
		let stream = self.stream.clone();
		thread::spawn(move ||{
			stream_bytes_write(bytes_rx, stream, waiter_tx);
		});

		let mut signature = Box::new(vec!());
		signature.push(0xffu8);
		signature.push(((self.options.read().identity_size + 1) as u64).to_u8());
		signature.push(0x7fu8);
		if bytes_tx.send(signature).is_err() {
			return Err(ZmqError::new(
				ErrorCode::EIOERROR,
				"Connection closed"));
		}

		let (decoder, encoder) = try!(self.handshake(bytes_tx));
		let _= waiter_tx.recv();
		debug!("Handshake is done: {} -> {}", self.stream.socket_add(), self.stream.peer_addr());

		let (msg_tx, msg_rx) = channel();
		let stream = self.stream.clone();
		thread::spawn(move || {
			stream_msg_write(msg_rx, stream, encoder);
		});

		let (tx, rx) = channel();
		debug!("Feeding the peer channels to the socket object.");
		if self.sender_to_socket.send(Ok(SocketMessage::OnConnected(msg_tx, rx))).is_err() {
			warn!("Socket object is gone!");
			return Ok(());
		}

		loop {
			match decoder.decode(&mut self.stream) {
				Ok(msg) => if tx.send(msg).is_err {
					return Ok(());
				},
				Err(e) => {
					let _ = self.sender_to_socket.send(Err(e));
					break;
				},
			}
		}
		Ok(())
	}

	fn handshake(&mut self, sender: Sender<Box<Vec<u8>>>) -> ZmqResult<(V2Decoder, V2Encoder)> {
		let mut greeting_recv = [0u8, ..V2_GREETING_SIZE];
		let mut greeting_bytes_read = 0;
		let mut zeros = 0;
		let mut type_sent = false;
		let mut version_sent = false;

		while greeting_bytes_read < V2_GREETING_SIZE {
			match self.stream.read(greeting_recv.slice_from_mut(greeting_bytes_read)) {
				Ok(0) => {
					zeros += 1;
					if zeros > NO_PROGRESS_LIMIT {
						return Err(ZmqError::new(ErrorCode::EIOERROR, "No progress in handshake"));
					} else {
						continue;
					}
				},
				Ok(n) => {
					greeting_bytes_read += n;
				},
				Err(e) => {
					return Err(ZmqError::from_io_error(e));
				},
			}

			if greeting_recv[0] != 0xff {
				return Err(ZmqError::new(ErrorCode::EPROTONOSUPPORT, "ZMTP 1.0 is not supported"));
			}

			if greeting_bytes_read < SIGNATURE_SIZE {
				continue;
			}

			if greeting_recv[9] & 0x01 == 0 {
				return Err(ZmqError::new(ErrorCode::EPROTONOSUPPORT, "ZMTP 1.0 is not supported"));
			}

			if !version_sent {
				version_sent = true;
				sender.send(Box::new([1u8].to_vec()));
			}

			if greeting_bytes_read > SIGNATURE_SIZE && !type_sent {
				type_sent = true;
				match greeting_recv[10] {
					ZMTP_1_0 | ZMTP_2_0 => {
						sender.send(Box::new((self.options.read().type_ as u8).to_vec()));
					},
					_ => {
						sender.send(Box::new((self.options.read().type_ as u8).to_vec()));
					}
				}
			}
		}

		if greeting_recv[0] != 0xff || greeting_recv[9] & 0x01 == 0 {
			return Err(ZmqError::new(ErrorCode::EPROTONOSUPPORT, "ZMTP 1.0 is not supported"));
		} else if greeting_recv[REVISION_POS] == ZMTP_1_0 {
			return Err(ZmqError::new(ErrorCode::EPROTONOSUPPORT, "ZMTP 1.0 is not supported"));
		} else {
			return Ok((V2Decoder::new(self.options.read().maxmsgsize), V2Encoder::new()));
		}
	}

	pub fn spawn_new(stream: TcpStream, options: Arc<RwLock<Options>>, 
		sender: Sender<ZmqResult<SocketMessage>>, death_notifier: Option<Sender<u8>>) {
		thread::spawn(move || {
			let mut engine = StreamEngine {
				sender_to_socket: sender,
				stream: stream,
				options: options,
				_death_notifier: death_notifier,
			};
			let _ = engine.run();
		});
	}
}

fn stream_bytes_write(receiver: Receiver<Box<Vec<u8>>>, mut stream: TcpStream, _waiter: Sender<u8>) {
	loop {
		match receiver.recv() {
			Ok(buf) => {
				match stream.write(buf.as_slice()) {
					Err(_) => break,
					_ => (),
				}
			},
			_ => break,
		}
	}
}

fn stream_msg_write(msg_receiver: Receiver<Box<Msg>>, mut stream: TcpStream, encoder:V2Encoder) {
	loop {
		match msg_receiver.recv() {
			Ok(msg) => {
				debug!("Sending message: {} @ {} -> {}", msg, stream.socket_addr(), stream.peer_addr());
				match encoder.encode(msg, &mut stream) {
					Err(_) => break,
					_ => (),
				}
			},
		}
	}
	let _ = stream.shutdown(Shutdown::Read);
}