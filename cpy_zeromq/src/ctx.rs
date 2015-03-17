use inproc::InprocManager;
use consts::SocketType;
use socket::ZmqSocket;
use socket_base::SocketBase;
use req;
use rep;

pub struct Context {
    inproc_mgr: InprocManager,
}

impl Context {
	pub fn new() -> Context {
		Context {
			inproc_mgr: InprocManager::new(),
		}
	}

	pub fn socket(&self: type_: SocketType) -> Box<ZmqSocket + Send> {
		let base = SocketBase::new(self.inproc_mgr.clone_sender());
		match type_ {
			SocketType::REQ => Box::new(req::new(base)) as Box<ZmqSocket + Send>,
			SocketType::REP => Box::new(rep::new(base)) as Box<ZmqSocket + Send>,
		}
	}
}