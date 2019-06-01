#![no_std]
#![feature(start)]
#![no_main]

#[no_mangle]
fn main() {
	0;
}

use core::panic::PanicInfo;

#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
