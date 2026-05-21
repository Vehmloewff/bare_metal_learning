use crate::Size;
use std::{
	marker::PhantomData,
	mem::size_of,
	slice::{from_raw_parts, from_raw_parts_mut},
	str::from_utf8,
};

#[cfg(feature = "runtime")]
pub struct MemoryLayout {
	start_ptr: usize,
	length: usize,
}

#[cfg(feature = "runtime")]
impl MemoryLayout {
	/// Construct a memory layout based on the hosted memory of an application. Function is unsafe because caller must ensure that ptr and length
	/// are actually the valid ptr and length of the memory and that they will remain valid for the lifetime of `MemoryLayout`
	pub unsafe fn new(ptr: *mut u8, length: usize) -> MemoryLayout {
		let start_ptr = ptr as usize;

		MemoryLayout { start_ptr, length }
	}

	pub fn dereference<T>(&self, offset: usize) -> Option<&mut T> {
		let length = size_of::<T>();

		if offset + length > self.length {
			None
		} else {
			Some(unsafe { &mut *((self.start_ptr + offset) as *mut T) })
		}
	}

	/// Safely gets a reference to bytes in `ByteList`
	pub fn get_bytes(&self, list: &ByteList) -> Option<&[u8]> {
		if (list.ptr.as_usize() + list.len.as_usize()) > self.length {
			return None;
		}

		let real_ptr = (self.start_ptr + list.ptr.as_usize()) as *const u8;

		Some(unsafe { from_raw_parts(real_ptr, list.len.as_usize()) })
	}

	/// Safely gets a mutable reference to bytes in `ByteList`
	pub fn get_bytes_mut(&self, list: &mut ByteList) -> Option<&mut [u8]> {
		if (list.ptr.as_usize() + list.len.as_usize()) > self.length {
			return None;
		}

		let real_ptr = (self.start_ptr + list.ptr.as_usize()) as *mut u8;

		Some(unsafe { from_raw_parts_mut(real_ptr, list.len.as_usize()) })
	}

	/// Safely copies the bytes in the list to a vector
	pub fn read_bytes(&self, list: &ByteList) -> Option<Vec<u8>> {
		Some(self.get_bytes(list)?.to_vec())
	}

	/// Safely gets a str reference to the list of bytes. None is returned if the bytes are not valid UTF-8
	pub fn get_str(&self, list: &ByteList) -> Option<&str> {
		Some(from_utf8(self.get_bytes(list)?).ok()?)
	}

	/// Safely reads an owned string from the bytes. None is returned if the bytes are note valid UTF-8
	pub fn read_string(&self, list: &ByteList) -> Option<String> {
		Some(String::from_utf8(self.read_bytes(list)?).ok()?)
	}
}

/// A representation of a list of bytes. Note: this structure is fully safe as long as it is construct using the provided constructors.
///
/// To read/write data to/from the byte list as a runtime, use `MemoryLayout`
#[repr(C)]
#[derive(Debug)]
pub struct ByteList<'a> {
	ptr: Size,
	len: Size,
	phantom_life: &'a PhantomData<u8>,
}

pub trait ByteListExt {
	fn as_byte_list<'a>(&'a self) -> ByteList<'a>;
}

impl ByteListExt for Vec<u8> {
	fn as_byte_list<'a>(&'a self) -> ByteList<'a> {
		let ptr = self.as_ptr();

		ByteList {
			ptr: Size::from_ptr(ptr),
			len: Size::from_usize(self.len()),
			phantom_life: &PhantomData,
		}
	}
}

impl ByteListExt for &[u8] {
	fn as_byte_list<'a>(&'a self) -> ByteList<'a> {
		let ptr = self.as_ptr();

		ByteList {
			ptr: Size::from_ptr(ptr),
			len: Size::from_usize(self.len()),
			phantom_life: &PhantomData,
		}
	}
}

impl ByteListExt for &mut [u8] {
	fn as_byte_list<'a>(&'a self) -> ByteList<'a> {
		let ptr = self.as_ptr();

		ByteList {
			ptr: Size::from_ptr(ptr),
			len: Size::from_usize(self.len()),
			phantom_life: &PhantomData,
		}
	}
}
