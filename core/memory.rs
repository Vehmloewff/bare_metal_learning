use agincourt_runtime_ops::{MemoryLayout, Op};
use wasmtime::{Memory, Store};

pub struct MemorySnapshot {
	layout: MemoryLayout,
	op_ptr: usize,
}

impl MemorySnapshot {
	pub fn from_memory<T>(store: &Store<T>, memory: &Memory, op_ptr: usize) -> MemorySnapshot {
		MemorySnapshot {
			layout: unsafe { MemoryLayout::new(memory.data_ptr(store), memory.data_size(store)) },
			op_ptr,
		}
	}

	pub fn get_op(&mut self) -> &mut Op {
		self.layout.dereference(self.op_ptr).unwrap()
	}

	pub fn get_layout(&self) -> &MemoryLayout {
		&self.layout
	}
}
