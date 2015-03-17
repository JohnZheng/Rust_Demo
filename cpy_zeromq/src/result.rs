use consts::ErrorCode;
use std::io;
use std::io::{Error, ErrorKind};

pub type ZmqResult<T> = Result<T, ZmqError>;

#[derive(Debug)]
pub struct ZmqError {
    pub code: ErrorCode,
    pub desc: &'static str,
    pub detail: Option<String>,
    pub iokind: Option<ErrorKind>,
}

impl ZmqError {
	pub fn new(code: ErrorCode, desc: &'static str) -> ZmqError {
		ZmqError {
			code: code,
			desc: desc,
			detail: None,
			iokind: None,
		}
	}

	pub fn from_io_error(e: Error) -> ZmqError {
		ZmqError {
			code: match e.kind() {
				io::PermissionDenied => ErrorCode::EACCES,
				io::ConnectionRefused => ErrorCode::ECONNREFUSED,
				io::ConnectionReset => ErrorCode::ECONNRESET,
				io::ConnectionAborted => ErrorCode::ECONNABORTED,
				io::NotConnected => ErrorCode::ENOTCONN,
				io::TimedOut => ErrorCode::ETIMEOUT,
				_ => ErrorCode::EIOERROR,
			},
			desc: e.description(),
			detail: e.detail(),
			iokind: Some(e.kind()),
		}
	}
}