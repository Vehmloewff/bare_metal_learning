use agincourt_runtime_ops::Op;

use super::interop_future::{InteropFuture, InteropInner};

extern "C" {
	fn queue_op(pointer: *mut Op, future_pointer: usize);
}

#[no_mangle]
unsafe extern "C" fn join_op(pointer: *mut Op, future_pointer: usize) {
	let future = &mut *(future_pointer as *mut InteropInner);
	let op = Box::from_raw(pointer);

	future.result.replace(op);

	if let Some(waker) = future.waker.take() {
		waker.wake()
	} else {
		panic!("no waker")
	}
}

pub async fn run_op<'a>(op: Op<'a>) -> Op<'a> {
	let inner = Box::new(InteropInner { result: None, waker: None });

	let op_ptr = Box::into_raw(Box::new(op));
	let future_ptr = Box::into_raw(inner);

	unsafe { queue_op(op_ptr, future_ptr as usize) };

	InteropFuture(future_ptr).await
}
