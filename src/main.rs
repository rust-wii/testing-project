#![feature(start)]

use mllib_sys::{ML_Init, ML_SplashScreen, ML_Refresh};

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(5);
    vec.push(7);

    unsafe {
        ML_Init();
        ML_SplashScreen();

        loop {
            ML_Refresh();
        }
    }
}
