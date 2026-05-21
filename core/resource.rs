use async_trait::async_trait;
use agincourt_runtime_ops::ResourceHandle;
use tokio::sync::oneshot::Sender;

pub enum ResourceEvent {
	AddResource {
		writer: Option<ContainedResourceWriter>,
		reader: Option<ContainedResourceReader>,
		response: Sender<ResourceHandle>,
	},
	GetResourceWriter {
		handle: ResourceHandle,
		response: Sender<ResourceAcquisition<ContainedResourceWriter>>,
	},
	GetResourceReader {
		handle: ResourceHandle,
		response: Sender<ResourceAcquisition<ContainedResourceReader>>,
	},
}

pub struct ResourceEventBuilder {
	proxy: EventLoopProxy,
}

pub enum ResourceAcquisition<T> {
	Busy,
	NotExists,
	Ok(T),
}

pub enum ResourceResult {
	PermissionDenied,
	ResourceClosed,
	IoError,
	Ok(usize),
	Done(usize),
}

#[async_trait]
pub trait ResourceWriter {
	async fn write(&mut self, bytes: &[u8]) -> ResourceResult;

	async fn signal_eof(&mut self);
}

#[async_trait]
pub trait ResourceReader {
	async fn read(&mut self, bytes: &mut [u8]) -> ResourceResult;
}

pub struct ContainedResourceWriter(Box<dyn ResourceWriter>);

impl ContainedResourceWriter {
	pub fn new(writer: impl ResourceWriter) -> ContainedResourceWriter {
		ContainedResourceWriter(Box::new(writer))
	}

	async fn write(&mut self, bytes: &[u8]) -> ResourceResult {
		self.0.write(bytes).await
	}

	async fn signal_eof(&mut self) {
		self.0.signal_eof().await
	}
}

pub struct ContainedResourceReader(Box<dyn ResourceReader>);

impl ContainedResourceReader {
	pub fn new(reader: impl ResourceReader) -> ContainedResourceReader {
		ContainedResourceReader(Box::new(reader))
	}

	async fn read(&mut self, bytes: &mut [u8]) -> ResourceResult {
		self.0.read(bytes).await
	}
}

struct ResourcePair {
	reader: Option<ContainedResourceReader>,
	writer: Option<ContainedResourceWriter>,
}

pub struct ResourcesManager {
	resources: Vec<Option<ResourcePair>>,
	open_slots: Vec<usize>,
}

impl ResourcesManager {
	pub fn handle_event(&mut self, event: ResourceEvent) {}
}

// pub struct Resource {
// 	reader: Option<Mutex<Box<dyn ResourceReader>>>,
// 	writer: Option<Mutex<Box<dyn ResourceWriter>>>,
// }

// impl Resource {
// 	pub fn new(reader: impl ResourceReader, writer: impl ResourceWriter) -> Resource {
// 		Resource { reader: None, writer: None }
// 	}

// 	pub fn set_reader(&mut self, reader: impl ResourceReader + 'static) {
// 		self.reader = Some(Mutex::new(Box::new(reader)))
// 	}

// 	pub fn set_writer(&mut self, writer: impl ResourceWriter + 'static) {
// 		self.writer = Some(Mutex::new(Box::new(writer)))
// 	}

// 	pub async fn read(&self, bytes: &mut [u8]) -> ResourceResult {
// 		match &self.reader {
// 			Some(reader) => reader.lock().await.read(bytes).await,
// 			None => ResourceResult::PermissionDenied,
// 		}
// 	}

// 	pub async fn write(&self, bytes: &[u8]) -> ResourceResult {
// 		match &self.writer {
// 			Some(writer) => writer.lock().await.write(bytes).await,
// 			None => ResourceResult::PermissionDenied,
// 		}
// 	}

// 	pub async fn signal_eof(&self) {
// 		match &self.writer {
// 			Some(writer) => writer.lock().await.signal_eof().await,
// 			None => (),
// 		}
// 	}
// }

// pub struct ApplicationResources {
// 	resources: Vec<Option<Resource>>,
// 	open_slots: Vec<usize>,
// }

// impl ApplicationResources {
// 	pub fn new() -> ApplicationResources {
// 		ApplicationResources {
// 			resources: Vec::new(),
// 			open_slots: Vec::new(),
// 		}
// 	}

// 	pub fn add(&mut self, resource: Resource) -> ResourceHandle {
// 		if let Some(open_slot) = self.open_slots.pop() {
// 			self.resources.get_mut(open_slot).unwrap().replace(resource);

// 			ResourceHandle(open_slot as u64)
// 		} else {
// 			let index = self.resources.len();
// 			self.resources.push(Some(resource));

// 			ResourceHandle(index as u64)
// 		}
// 	}

// 	pub fn get(&self, handle: ResourceHandle) -> Option<&Resource> {
// 		match self.resources.get(handle.0 as usize) {
// 			Some(result) => result.as_ref(),
// 			None => None,
// 		}
// 	}

// 	pub fn close(&mut self, handle: ResourceHandle) -> Option<Resource> {
// 		match self.resources.get_mut(handle.0 as usize) {
// 			Some(result) => result.take(),
// 			None => None,
// 		}
// 	}
// }
