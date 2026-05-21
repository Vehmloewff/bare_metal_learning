use agincourt_bindings::{application, read, write};
use agincourt_runtime_ops::ResourceHandle;

application!();

async fn main() {
	read(ResourceHandle::log(), 0, &mut [0, 0, 0, 0, 0, 0, 0, 0]).await.unwrap_err();
	write(ResourceHandle::log(), 0, b"Hello, World!\n", false).await.unwrap();
}
