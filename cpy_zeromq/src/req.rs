use socket_base::SocketBase;
use socket::ZmqSocket;
use result::{ZmqResult};

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