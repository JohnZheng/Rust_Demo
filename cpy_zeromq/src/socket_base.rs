use std::sync::mpsc::{Sender, Receiver};
use msg::Msg;
use std::sync::{Arc, RwLock};
use result::{ZmqResult, ZmqError};
use inproc::InprocCommand;
use consts::ErrorCode;
use std::net::SocketAddr;

pub enum SocketMessage {
    Ping,
    OnConnected(Sender<Box<Msg>>, Receiver<Box<Msg>>),
}

pub struct SocketBase {
    options: Arc<RwLock<Options>>,
    tx: Sender<ZmqResult<SocketMessage>>,
    rx: Receiver<ZmqResult<SocketMessage>>,
    peers: HashMap<uint, Peer>,
    ids: Vec<uint>,
    inproc_sender: Sender<InprocCommand>,
}

impl SocketBase {
	pub fn new(sender: Sender<InprocCommand>) -> SocketBase {
		let (tx, rx) = channel();
		SocketBase {
			options: Arc::new(RwLock::new(Options::new())),
			tx: tx,
			rx: rx,
			peers: HashMap::new(),
			ids: Vec::new(),
			inproc_sender: sender,
		}
	}

	pub fn set_type(&self, type_: consts::SocketType) {
		self.options.write().type_ = type_ as int;
	}

	pub fn connect(&self, addr: &str) -> ZmqResult<()> {
		let (protocol, address) = try!(parse_uri(addr));
		match protocol {
			"tcp" => {
				match SocketAddr::from_str(address) {
					Some() => expr,
				}
			},
		}
	}
}


fn parse_uri(uri: &'r str) -> ZmqResult<(&'r str, &'r str)> {
	match uri.find_str("://") {
		Some(pos) => {
			let protocol = uri.slice_to(pos);
			let address = uri.slice_from(pos + 3);
			if protocol.len() == 0 || address.len() == 0 {
				Err(ZmqError::new(
					ErrorCode::EINVAL,
					"Invaild argument: missing protocol or address"))
			} else {
				Ok((protocol, address))
			}
		},
		None => Err(ZmqError::new(
			ErrorCode::EINVAL,
			"Invaild argument: missing ://")),
	}
}