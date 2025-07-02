#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::time;

#[cfg(feature = "axstd")]
use axstd::println;

#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    println!("Hello, world!");
    loop {
        let va = 0xffff00001000800c as *mut u16;
        unsafe {
            println!("rr={:#x}", *va);
            *va = 1 << 8;
            axstd::thread::sleep(axstd::time::Duration::from_secs(1));
        }
    }
}
