mod application;
mod database;
mod execute_op;
mod memory;
mod resource;
mod event_loop;

use execute_op::execute_op;
use futures::{
	future::BoxFuture,
	task::{waker_ref, ArcWake},
	FutureExt,
};
use memory::MemorySnapshot;
use std::{
	fs::read,
	sync::{Arc, Mutex},
	task::{Context, Poll, Wake},
};
use tokio::sync::mpsc::{channel, Sender};
use wasmtime::{Caller, Config, Engine, Func, Instance, Module, Store};

enum OpEvent<'a> {
	Queue { offset: i32, response_pointer: i32 },
	Poll { task: Task<'a> },
}

pub struct Task<'a> {
	offset: i32,
	response_pointer: i32,
	future: Mutex<Option<BoxFuture<'a, ()>>>,
	sender: Sender<OpEvent<'a>>,
}

impl Wake for Task<'_> {
	fn wake(self: Arc<Self>) {}
	fn wake_by_ref(arc_self: &Arc<Self>) {
		arc_self.sender.try_send(OpEvent::Poll { task: arc_self.clone() }).expect("too many operations are ongoing");
	}
}

struct State<'a> {
	tasks_count: usize,
	sender: Sender<OpEvent<'a>>,
}

#[tokio::main]
async fn main() {
	let mut config = Config::new();
	config.async_support(true);

	let engine = Engine::new(&config).unwrap();
	let wasm_bytes = read("target/wasm32-unknown-unknown/debug/example_app.wasm").unwrap();
	let module = Module::new(&engine, wasm_bytes).unwrap();

	let imports = module.imports().collect::<Vec<_>>();
	if imports.len() != 1 {
		panic!("expected 1 import, but found {}", imports.len())
	}

	let (sender, mut receiver) = channel(u32::MAX as usize);

	let mut store = Store::new(
		&engine,
		State {
			tasks_count: 0,
			sender: sender.clone(),
		},
	);

	let queue_op = Func::wrap2_async(&mut store, |mut caller: Caller<'_, State>, offset: i32, response_pointer: i32| {
		Box::new(async move {
			let data = caller.data_mut();
			data.tasks_count = data.tasks_count + 1;

			let _ = data.sender.send(OpEvent::Queue { offset, response_pointer }).await;
		})
	});

	let instance = Instance::new_async(&mut store, &module, &[queue_op.into()]).await.unwrap();

	let join_op = instance.get_typed_func::<(i32, i32), ()>(&mut store, "join_op").unwrap();
	let main = instance.get_typed_func::<(), ()>(&mut store, "__start").unwrap();

	let memory = Arc::new(instance.get_memory(&mut store, "memory").unwrap());

	main.call_async(&mut store, ()).await.unwrap();

	loop {
		// if we have no ops pending, we are done
		{
			let tasks_count = store.data().tasks_count;
			if tasks_count <= 0 {
				break;
			}
		}

		// get the next task to be run
		let task = match receiver.recv().await {
			Some(task) => task,
			None => break,
		};

		match task {
			OpEvent::Queue { offset, response_pointer } => {
				let memory_snapshot = MemorySnapshot::from_memory(&store, &memory, offset as usize);

				let future = Mutex::new(Some(execute_op(memory_snapshot).boxed()));
				let task = Task {
					future,
					offset,
					response_pointer,
					sender: sender.clone(),
				};

				sender.send(OpEvent::Poll { task }).await.unwrap();
			}
			OpEvent::Poll { task } => {
				let mut future = task.future.lock().unwrap().take().unwrap();

				let waker = waker_ref(&task);
				let context = &mut Context::from_waker(&waker);
				let poll = future.as_mut().poll(context);

				match poll {
					Poll::Pending => {
						task.future.lock().unwrap().replace(future);

						sender.send(OpEvent::Poll { task }).await.unwrap();
					}
					Poll::Ready(_) => {
						// notify the process that we are finished
						join_op.call_async(&mut store, (task.offset, task.response_pointer)).await.unwrap();

						// remember that we have one less op pending
						let state = store.data_mut();
						state.tasks_count = state.tasks_count - 1;
					}
				}
			}
		}
	}
}
