mod bytes;
mod database;
mod event;
mod identity;
mod size;

pub use bytes::*;
pub use database::*;
pub use event::*;
pub use identity::*;
pub use size::*;

#[repr(C)]
#[derive(Debug)]
pub enum SafeOption<T> {
	Some(T),
	None,
}

impl<T> SafeOption<T> {
	pub fn from_native(option: Option<T>) -> SafeOption<T> {
		match option {
			None => SafeOption::None,
			Some(val) => SafeOption::Some(val),
		}
	}

	pub fn native(self) -> Option<T> {
		match self {
			SafeOption::None => Option::None,
			SafeOption::Some(val) => Option::Some(val),
		}
	}
}

#[repr(C)]
#[derive(Debug)]
pub enum Op<'a> {
	/// Read from resource, such as a file, socket, etc.
	Read {
		/// The resource to read
		resource: ResourceHandle,
		/// Start reading at this offset.
		resource_offset: u64,
		/// Read bytes into this list. Caller should allocate a bunch of empty space
		bytes: ByteList<'a>,
		/// The number of bytes read. Should have the initial value of 0, set by the caller, but then be set by the runtime
		bytes_read: Size,
		/// Should be set to initial value of `false` by caller and then set to `true` by the application if this read represents the last of the data
		is_eof: bool,
		/// Should be set to initial value of `None` by caller, but be set to a `Some(Error)` variant by the runtime if an error did occur
		error: SafeOption<ReadError>,
	},
	/// Write to a resource, such as a file, socket, etc.
	Write {
		/// The resource to write
		resource: ResourceHandle,
		/// If allowed, and the resource doesn't already exist, create it
		create: bool,
		/// The offset to start writing at
		resource_offset: u64,
		/// The bytes to write, set by the caller. Bytes must be copied out of list by the runtime and the list cannot be mutate in any way
		bytes: ByteList<'a>,
		/// The number of bytes written
		bytes_written: Size,
		/// Should be set to `true` by the caller if this write represents the last of the data if all bytes are written
		is_eof: bool,
		/// Should be set to initial value of `None` by caller, but be set to a `Some(Error)` variant by the runtime if an error did occur
		error: SafeOption<WriteError>,
	},
	/// An op to allocate a yet-unknown. Reads and writes do not need to resolve until runtime has internally assigned the resource to a task.
	///
	/// This can be used to, for example, start writing bytes to a request body before `Opp::Fetch` is called.
	StartResource {
		/// Initialized as None by the caller and set by the runtime
		resource: SafeOption<ResourceHandle>,
		/// Should be set to initial value of `None` by caller, but be set to a `Some(Error)` variant by the runtime if an error did occur
		error: SafeOption<MakeBlobError>,
	},
	/// An op to make an http(s) request.
	Fetch {
		/// The url to make the request to
		url: ByteList<'a>,
		/// The method of the request
		method: FetchMethod,
		/// The headers to send with the request, newline delimited
		request_headers: ByteList<'a>,
		/// The number of bytes that the client will write to the request body, then, once op returns, the number of bytes the server will
		/// write back to the client
		content_length: u64,
		/// The body to send along with the request. Request is sent as soon as op is received, but data is fed into the request body as
		/// fast as it is written here.
		///
		/// An error will be given if the resource handle hits EOF before `content_length` bytes have been sent.
		///
		/// Caller can only set this to `SafeOption::None` if content_length is 0.
		request_body_handle: SafeOption<ResourceHandle>,
		/// Initialized to None by the caller and set to a resource by the runtime. Resource handle to reads the bytes written by the server
		response_body_handle: SafeOption<ResourceHandle>,
		/// Initialized to None by the caller and set to a resource by the runtime. Resource handle to read the newline delimited headers
		/// set by the server during the response
		response_body_headers: SafeOption<ResourceHandle>,
		/// The status code that the server responded with
		status_code: SafeOption<u32>,
		/// The status text accompanying the response sent by the server
		status_text: ByteList<'a>,
		/// Should be set to initial value of `None` by caller, but be set to a `Some(Error)` variant by the runtime if an error did occur
		error: SafeOption<FetchError>,
	},
	/// Grab the next event for consumption
	GrabEvent {
		/// Should be set to `None` by the caller and replaced with the event by the runtime
		event: SafeOption<Event>,
	},
	/// Open a database
	OpenDatabase {
		/// The name to of the database to open
		name: ByteList<'a>,
		/// Set to `None` by the caller, and replaced with the resource handle of the database by the runtime
		resource: SafeOption<ResourceHandle>,
		/// Set to `None` by the caller, and replaced with an error by the runtime if one occurs
		error: SafeOption<OpenDatabaseError>,
	},
	OpenApplicationChannel {
		/// The application identity to open a channel to
		identity: Identity,
		/// Set to `None` by the caller, and replaced with the resource handle of the application channel by the runtime
		resource: SafeOption<ResourceHandle>,
		/// Set to `None` by the caller, and replaced with an error by the runtime if one occurs
		error: SafeOption<OpenApplicationChannelError>,
	},
}

#[repr(C)]
#[derive(Debug)]
pub enum FetchMethod {
	Get,
	Post,
	Put,
	Patch,
	Delete,
	Options,
}

#[repr(C)]
#[derive(Debug)]
pub enum ReadError {
	ResourceMissing,
	NotReadable,
}

#[repr(C)]
#[derive(Debug)]
pub enum WriteError {
	ResourceMissing,
}

#[repr(C)]
#[derive(Debug)]
pub enum MakeBlobError {
	TooManyResources,
}

#[repr(C)]
#[derive(Debug)]
pub enum FetchError {
	Unreachable,
}

#[repr(C)]
#[derive(Debug)]
pub enum OpenDatabaseError {
	NotExists,
}

#[repr(C)]
#[derive(Debug)]
pub enum OpenApplicationChannelError {
	NotExists,
	PermissionDenied,
}
