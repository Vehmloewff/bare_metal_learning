use crate::{Identity, ResourceHandle};

#[repr(C)]
#[derive(Debug)]
pub enum Event {
	UiRequested { javascript: ResourceHandle, channel: ResourceHandle },
	ApplicationChannelOpened { channel: ResourceHandle, identity: Identity },
}
