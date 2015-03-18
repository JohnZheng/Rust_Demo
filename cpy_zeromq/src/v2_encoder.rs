use msg::Msg;
use std::io:Write;
use msg;
use v2_protocol;

pub struct V2Encoder;

impl V2Encoder {
	pub fn new() -> V2Encoder {
		V2Encoder
	}

	pub fn encode(&self, msg: Box<Msg>, writer: &mut Write) -> Result<()> {
		let mut protocol_flags = 0u8;
		if msg.flags & msg::MORE != 0 {
			protocol_flags |= v2_protocol::MORE_FLAG;
		}

		if msg.data.len() > 255 {
			protocol_flags |= v2_protocol:LARGE_FLAG;
		}

		if msg.flags & msg::COMMAND != 0 {
			protocol_flags != v2_protocol::COMMAND_FLAG;
		}
		try!(writer.write([protocol_flags]));

		if msg.data.len() > 255 {
			try!(writer.write(msg.data.len() as u64));
		} else {
			try!(writer.write(msg.data.len() as u8));
		}

		try!(writer.write(msg.data.as_slice()));
		Ok(())
	}
}