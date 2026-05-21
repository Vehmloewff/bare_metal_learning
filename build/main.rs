use bootloader::BiosBoot;
use std::{path::PathBuf, process::Command};

fn main() {
	// // read env variables that were set in build script
	// let uefi_path = env!("UEFI_PATH");
	// let bios_path = env!("BIOS_PATH");

	// // choose whether to start the UEFI or BIOS image
	// let uefi = true;

	// let mut cmd = std::process::Command::new("qemu-system-x86_64");
	// if uefi {
	// 	cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
	// 	cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
	// } else {
	// 	cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
	// }
	// let mut child = cmd.spawn().unwrap();
	// child.wait().unwrap();

	Command::new("cargo")
		.arg("build")
		.arg("--release")
		.arg("--target=x86_64-unknown-none")
		.arg("--package=agincourt_kernel")
		.spawn()
		.unwrap()
		.wait()
		.unwrap();

	let kernel = PathBuf::from("target/x86_64-unknown-none/release/agincourt_kernel");
	let bios_path = PathBuf::from("target/agincourt.img");

	BiosBoot::new(&kernel).create_disk_image(&bios_path).unwrap()
}
