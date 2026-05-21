use std::{
	io::{stdout, Write},
	slice,
};

use agincourt_runtime_ops::{Op, ReadError};
use wasmtime::Memory;

use crate::memory::MemorySnapshot;

pub struct OpExecutor {}

impl OpExecutor {
	pub fn define_joiner() {}

	pub fn execute() {}
}

pub struct Task {}

pub async fn execute_op(snapshot: MemorySnapshot) {
	match snapshot.get_op() {
		Op::Read {
			resource,
			resource_offset,
			bytes,
			bytes_read,
			is_eof,
			error,
		} => {}
	}
	// 		length,
	// 		bytes_read,
	// 	} => *op = Op::ReadError(ReadError::NotReadable),
	// 	Op::Write {
	// 		resource,
	// 		create,
	// 		resource_offset,
	// 		source_ptr,
	// 		length,
	// 		bytes_written,
	// 	} => {
	// 		let real_source_ptr = unsafe { memory_ptr.offset(source_ptr.as_isize()) };
	// 		let buf = unsafe { slice::from_raw_parts(real_source_ptr, length.as_usize()) };

	// 		bytes_written.add(stdout().write(buf).unwrap())
	// 	}
	// 	_ => panic!("unknown op"),
	// }
}
