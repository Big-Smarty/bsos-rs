#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod serial;
mod vga_buffer;

use bootloader_api::{entry_point, BootInfo};
use kernel::{exit_qemu, QemuExitCode};

static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    serial_println!("it works");
    exit_qemu(QemuExitCode::Success);
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
