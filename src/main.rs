#![no_std]
#![no_main]

extern crate mllib_sys;

use core::panic::PanicInfo;
use mllib_sys::{ML_Init, ML_SplashScreen, ML_Refresh};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
	unsafe{
		ML_Init();
		ML_SplashScreen();
		
		loop {
			ML_Refresh();
		}
	}
	
	0	

}
