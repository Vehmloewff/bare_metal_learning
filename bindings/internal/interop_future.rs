use std::{
	future::Future,
	pin::Pin,
	task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use agincourt_runtime_ops::Op;

pub struct InteropFuture<'a>(pub *mut InteropInner<'a>);

pub struct InteropInner<'a> {
	pub result: Option<Box<Op<'a>>>,
	pub waker: Option<Waker>,
}

impl<'a> Future for InteropFuture<'a> {
	type Output = Op<'a>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Op<'a>> {
		let mut inner = unsafe { Box::from_raw(self.0) };

		if let Some(result) = inner.result.take() {
			return Poll::Ready(*result);
		}

		inner.waker.replace(cx.waker().clone());
		let _ = Box::into_raw(inner);

		Poll::Pending
	}
}

static V_TABLE: RawWakerVTable = RawWakerVTable::new(waker_clone as _, waker_wake as _, waker_wake_by_ref as _, waker_drop as _);

fn poll_future(ptr: *const ()) {
	let task = unsafe { &mut *(ptr as *mut Task) };
	let waker = unsafe { Waker::from_raw(waker_clone(ptr)) };
	let context = &mut Context::from_waker(&waker);

	task.poll(context)
}

unsafe fn waker_wake(ptr: *const ()) {
	poll_future(ptr)
}

unsafe fn waker_wake_by_ref(ptr: *const ()) {
	poll_future(ptr)
}

unsafe fn waker_drop(_ptr: *const ()) {}

unsafe fn waker_clone(ptr: *const ()) -> RawWaker {
	RawWaker::new(ptr, &V_TABLE)
}

struct Task(Box<dyn Future<Output = ()>>);

impl Task {
	fn poll(&mut self, mut context: &mut Context) {
		let reference = self.0.as_mut();
		let pinned = unsafe { Pin::new_unchecked(reference) };

		let _ = pinned.poll(&mut context);
	}
}

pub fn async_execute_future(future: impl Future<Output = ()> + 'static) {
	let pointer = Box::into_raw(Box::new(Task(Box::new(future))));

	poll_future(pointer as *const ());
}
