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
				ErrorKind::PermissionDenied => ErrorCode::EACCES,
				ErrorKind::ConnectionRefused => ErrorCode::ECONNREFUSED,
				ErrorKind::ConnectionReset => ErrorCode::ECONNRESET,
				ErrorKind::ConnectionAborted => ErrorCode::ECONNABORTED,
				ErrorKind::NotConnected => ErrorCode::ENOTCONN,
				ErrorKind::TimedOut => ErrorCode::ETIMEDOUT,
				_ => ErrorCode::EIOERROR,
			},
			desc: e.description(),
			detail: e.detail(),
			iokind: Some(e.kind()),
		}
	}
}