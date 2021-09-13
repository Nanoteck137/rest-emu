#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{ AtomicI32, Ordering };

static TEST: AtomicI32 = AtomicI32::new(123);

#[no_mangle]
fn _start(a: i32) -> i32 {
    let value = TEST.fetch_add(1, Ordering::SeqCst);

    a + value
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
