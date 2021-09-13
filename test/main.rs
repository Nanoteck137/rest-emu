#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn _start(a: i32, b: i32) -> i32 {
    a + b
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
