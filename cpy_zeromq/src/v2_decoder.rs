use std::io::Read;
use result::{ZmqResult};
use msg::Msg;
use msg;
use v2_protocol;

pub struct V2Decoder {
    maxmsgsize: i64,
}

impl V2Decoder {
	pub fn new(maxmsgsize: i64) -> V2Decoder {
		V2Decoder {
			maxmsgsize: maxmsgsize,
		}
	}

	pub fn decode(&self, reader: &mut Read) -> ZmqResult<Box<Msg>> {
		let mut buf: &[u8] = [0u8, 128];
		let mut msg_flags = 0u8;
		let read_bytes = match reader.read(&mut buf) {
			Ok(bytes) => bytes,
			Err(e) => {
				println!("read buffer error");
			},
		};

		if buf[0] & v2_protocol::MORE_FLAG != 0 {
			msg_flags |= msg::MORE;
		}
		if (buf[0] & v2_protocol::COMMAND_FLAG != 0 ) {
			msg_flags |= msg::COMMAND;
		}
		
		// let msg_size = 
		// 	if buf[0] & v2_protocol::LARGE_FLAG != 0 {
		// 		// let size = buf[]
		// 	}
	}
}