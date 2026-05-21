use crate::resource::ResourceEvent;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum OpEvent {
	Queue { offset: i32, response_pointer: i32 },
	Join { response_pointer: i32 },
	Resource(ResourceEvent),
}

pub struct EventLoop {
	receiver: Receiver<OpEvent>,
	sender: Sender<OpEvent>,
	buffer_size: usize,
}

impl EventLoop {
	pub async fn next_event(&mut self) -> Option<OpEvent> {
		let event = if self.buffer_size == 0 { None } else { self.receiver.recv().await };

		if let Some(OpEvent::Join { .. }) = event {
			self.buffer_size = self.buffer_size - 1;
		}

		if let Some(OpEvent::Queue { .. }) = event {
			self.buffer_size = self.buffer_size + 1;
		}

		event
	}

	pub fn get_proxy(&self) -> EventLoopProxy {
		EventLoopProxy(self.sender.clone())
	}
}

pub struct EventLoopProxy(Sender<OpEvent>);

impl EventLoopProxy {
	pub async fn send(&self, event: OpEvent) {
		let _ = self.0.send(event).await;
	}
}
