const COM1: u16 = 0x3F8; // Base port for COM1

#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn outb(port: u16, val: u8) {
	#[cfg(target_arch = "x86_64")]
	core::arch::asm!("out dx, al", in("dx") port, in("al") val);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn outb(_port: u16, _val: u8) {}

fn serial_write_char(c: u8) {
	unsafe { outb(COM1, c) }
}

fn serial_write(s: &str) {
	for b in s.bytes() {
		serial_write_char(b);
	}
}

/// Print a debug message to the serial port.
pub fn debug(message: &str) {
	serial_write(message);
	serial_write("\n");
}
