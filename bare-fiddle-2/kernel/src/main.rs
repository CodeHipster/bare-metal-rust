#![no_std]
#![no_main]

mod lib;

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

use core::fmt::Write;
use lib::serial;

static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    writeln!(serial(), "Entered kernel with boot info: {boot_info:?}").unwrap();
    // let vga_buffer = 0xfd000000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
