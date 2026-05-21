// TODO:
// Implement a virtual resource that reads and writes with futures. This will be used to handle application communication, log files, etc.
//
// pub struct VirtualResource {
// 	bytes: Vec<u8>,
// 	did_finish_writing: bool,
// }

// impl VirtualResource {
// 	pub fn new() -> VirtualResource {
// 		VirtualResource {
// 			bytes: Vec::new(),
// 			did_finish_writing: false,
// 		}
// 	}
// }

// impl Resource for VirtualResource {
// 	fn read(&mut self, mut bytes: &mut [u8]) -> ResourceResult {
// 		let bytes_read = match bytes.write(&self.bytes) {
// 			Ok(len) => len,
// 			Err(_) => return ResourceResult::IoError,
// 		};

// 		self.bytes.drain(0..bytes_read);

// 		ResourceResult::Ok(bytes_read)
// 	}

// 	fn write(&mut self, bytes: &[u8]) -> ResourceResult {
// 		self.bytes.extend_from_slice(bytes);

// 		ResourceResult::Ok(bytes.len())
// 	}

// 	fn signal_eof(&mut self) {
// 		self.did_finish_writing = true
// 	}

// 	fn will_have_more_data(&self) -> bool {
// 		!self.did_finish_writing
// 	}
// }
