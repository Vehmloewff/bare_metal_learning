#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Size {
	Large(u64),
	Small(u32),
}

impl Size {
	pub fn from_usize(n: usize) -> Size {
		if usize::BITS == 32 {
			Size::Small(n as u32)
		} else if usize::BITS == 64 {
			Size::Large(n as u64)
		} else {
			panic!("unexpected bit count of usize: {}", usize::BITS)
		}
	}

	pub fn from_ptr<T>(ptr: *const T) -> Size {
		Size::from_usize(ptr as usize)
	}

	pub fn zero() -> Size {
		Size::from_usize(0)
	}

	pub fn as_usize(self) -> usize {
		match self {
			Size::Small(n) => n as usize,
			Size::Large(n) => n as usize,
		}
	}

	pub fn as_isize(self) -> isize {
		match self {
			Size::Small(n) => n as isize,
			Self::Large(n) => n as isize,
		}
	}

	pub fn as_ptr<T>(self) -> *const T {
		self.as_usize() as *const T
	}

	pub fn as_mut_ptr<T>(self) -> *mut T {
		self.as_usize() as *mut T
	}

	pub fn add(&mut self, n: usize) {
		match self {
			Size::Small(x) => *x = *x + n as u32,
			Size::Large(x) => *x = *x + n as u64,
		}
	}
}
