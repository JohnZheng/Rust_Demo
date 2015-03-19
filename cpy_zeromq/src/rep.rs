use socket_base::SocketBase;
use socket::ZmqSocket;
use result::{ZmqResult, ZmqError};
use consts::{SocketType, ErrorCode};
use msg;
use msg::Msg;

enum State {
    Initial,
    Receiving,
    Sending,
}

pub struct RepSocket {
    base: SocketBase,
    state: State,
    last_identity: uint,
}

impl ZmqSocket for RepSocket {
	fn bind(&self, addr: &str) -> ZmqResult<()> {
		self.base.bind(addr)
	}

	fn msg_recv(&mut self) -> ZmqResult<Box<Msg>> {
		let (id, ret) = match self.state {
			State::Initial => self.base.recv_fist(),
			State::Receiving => (self.last_identity, self.base.recv_from(self.last_identity)),
			_ => return Err(ZmqError::new(
				ErrorCode::EFSM, "Operation cannot be accomplished in current state")),
		};
		self.last_identity = id;
		self.state = match self.flags & msg::MORE {
			0 => State::Sending,
			_ => State::Receiving,
		};
		Ok(ret)
	}
}

pub fn new(base: SocketBase) -> RepSocket {
	base.set_type(SocketType::REP);
	RepSocket {
		base: base,
		state: State::Initial,
		last_identity: 0,
	}
}