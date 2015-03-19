use socket_base::SocketBase;
use socket::ZmqSocket;
use result::{ZmqResult, ZmqError};
use msg::Msg;
use msg;
use consts::ErrorCode;

enum State {
    Initial,
    Sending,
    Receiving,
}

pub struct ReqSocket {
    base: SocketBase,
    state: State,
    last_identity: uint,
    send_count: uint,
}

impl ZmqSocket for ReqSocket {
	fn connect(&self, addr: &str) -> ZmqResult<()> {
		self.base.connect(addr);
	}

	fn msg_send(&mut self, msg: Box<Msg>) -> ZmqResult<()> {
		let flags = msg.flags;
		match self.state {
			State::Initial => {
				let (count, id) = self.base.round_robin(self.send_count);
				self.send_count = count;
				self.base.send_to(id, msg);
				self.last_identity = id;
			},
			State::Sending => {
				self.base.send_to(self.last_identity, msg);
			},
			_ => Err(ZmqError::new(ErrorCode::EFSM,
				"Operation connot be accomplished in current state")),
		}
		self.state = match flags & msg::MORE {
			0 => State::Receiving,
			_ => State::Sending,
		};
		Ok(())
	}
}

pub fn new(base: SocketBase) -> ReqSocket {
	base.set_type(SocketBase::REQ);
	ReqSocket {
		base: base,
		state: State::Initial,
		last_identity: 0,
		send_count: 0,
	}
}