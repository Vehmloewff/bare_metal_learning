#![no_std]
#![no_main]
#![cfg(not(test))]
#![feature(alloc_error_handler, asm_experimental_arch)]

mod debug;

use core::panic::PanicInfo;
use debug::debug;

const CONFIG: bootloader_api::BootloaderConfig = {
	let mut config = bootloader_api::BootloaderConfig::new_default();
	config.kernel_stack_size = 100 * 1024; // 100 KiB
	config
};

#[cfg(target_arch = "x86_64")]
bootloader_api::entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
	debug("Hello, world!");
	debug("How are you?");

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	debug("we have panicked:");
	debug(info.message().as_str().unwrap_or("[no message]"));

	loop {}
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
	loop {}
}
