use consts;
use result::ZmqResult;
use msg::Msg;

pub trait ZmqSocket {
	fn getsockopt(&self, option: consts::SocketOption) -> int;
	fn bind(&self, endpoint: &str) -> ZmqResult<()>;
	fn connect(&self, endpoint: &str) -> ZmqResult<()>;
	fn msg_recv(&mut self) -> ZmqResult<Box<Msg>>;
	fn msg_send(&mut self, msg: Box<Msg>) -> ZmqResult<()>;
}