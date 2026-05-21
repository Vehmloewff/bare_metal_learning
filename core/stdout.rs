pub struct StdoutResource;

#[async_trait]
impl Resource for StdoutResource {
	async fn read(&mut self, bytes: &mut [u8]) -> ResourceResult {
		ResourceResult::PermissionDenied
	}

	async fn write(&mut self, bytes: &[u8]) -> ResourceResult {
		let bytes_written = match stdout().write(bytes).await {
			Ok(n) => n,
			Err(_) => return ResourceResult::IoError,
		};

		ResourceResult::Ok(bytes_written)
	}

	fn will_have_more_data(&self) -> bool {
		false
	}

	fn signal_eof(&mut self) {}
}
