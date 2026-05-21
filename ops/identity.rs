#[repr(C)]
#[derive(Debug)]
pub struct ResourceHandle(pub u64);

impl ResourceHandle {
	pub fn log() -> ResourceHandle {
		ResourceHandle(0)
	}
}

/// An identity, which can be either a host, user and host, or a user, host, and application.
///
/// Here is an example:
///
/// vehmloewff@agincourt.com/some-really-long-name
/// ---------- --------- ---------------------
///   24 max    24 max           24 max
///
/// If a particular section is too long for it's 24-byte range, the rest of the available space should be filled 0x00 bytes
#[repr(C)]
#[derive(Debug)]
pub struct Identity([u8; 72]);

// TODO impl identity
