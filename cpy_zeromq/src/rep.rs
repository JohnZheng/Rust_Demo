use socket_base::SocketBase;

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

pub fn new(base: SocketBase) -> RepSocket {
	base.set_type(SocketType::REP);
	RepSocket {
		base: base,
		state: State,
		last_identity: uint,
	}
}