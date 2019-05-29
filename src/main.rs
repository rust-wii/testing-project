#![no_std]
#![feature(lang_items, start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(name = "MLlib", kind = "static")]
extern "C" {
    fn ML_Init();
    fn ML_SplashScreen();
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
	unsafe {
		ML_Init();
		ML_SplashScreen();
	}
	0
}
