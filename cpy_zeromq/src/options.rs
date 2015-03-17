use consts::SocketOption;
use std::time::duration::Duration;

pub struct Options {
    pub identity_size: u8,
    pub type_: int,
    pub reconnect_ivl: Duration,
    pub reconnect_ivl_max: Duration,
    pub maxmsgsize: i64,
}

impl Options {
	pub fn new() -> Options {
		Options {
			identity_size: 0,
			type_: -1,
			maxmsgsize: -1,
			reconnect_ivl: Duration::milliseconds(100),
			reconnect_ivl_max: Duration::zero(),
		}
	}

	pub fn getsockopt(&self, option: SocketOption) -> int {
		match option {
			SocketOption::TYPE => self.type_ as int,
		}
	}
}