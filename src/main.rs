#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use lazy_static::lazy_static;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Panic message");
    loop {}
}

#[panic_handler]
fn panic(_inf: &PanicInfo) -> ! {
    println!("{}", _inf);
    loop {}
}
