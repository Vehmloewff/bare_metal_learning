use crate::internal::run_op;
use agincourt_runtime_ops::{ByteListExt, MakeBlobError, Op, ReadError, ResourceHandle, SafeOption, Size, WriteError};
use rutils::OptionExt;

pub async fn read(resource: ResourceHandle, resource_offset: u64, bytes: &mut [u8]) -> Result<(usize, bool), ReadError> {
	let response = run_op(Op::Read {
		resource,
		resource_offset,
		bytes: bytes.as_byte_list(),
		bytes_read: Size::zero(),
		is_eof: false,
		error: SafeOption::None,
	})
	.await;

	match response {
		Op::Read { bytes_read, error, is_eof, .. } => {
			error.native().err()?;

			Ok((bytes_read.as_usize(), is_eof))
		}
		_ => panic!("runtime gave an invalid response"),
	}
}

pub async fn write(resource: ResourceHandle, resource_offset: u64, bytes: &[u8], is_eof: bool) -> Result<usize, WriteError> {
	let response = run_op(Op::Write {
		create: false,
		resource,
		resource_offset,
		bytes_written: Size::zero(),
		error: SafeOption::None,
		is_eof,
		bytes: bytes.as_byte_list(),
	})
	.await;

	match response {
		Op::Write { bytes_written, error, .. } => {
			error.native().err()?;
			Ok(bytes_written.as_usize())
		}
		_ => panic!("runtime gave an invalid response"),
	}
}

pub async fn make_blob() -> Result<ResourceHandle, MakeBlobError> {
	let response = run_op(Op::StartResource {
		resource: SafeOption::None,
		error: SafeOption::None,
	})
	.await;

	match response {
		Op::StartResource { resource, error } => {
			error.native().err()?;

			match resource {
				SafeOption::Some(resource) => Ok(resource),
				SafeOption::None => panic!("runtime did not give either a resource or an error"),
			}
		}
		_ => panic!("invalid response from runtime to MakeBlob op"),
	}
}
