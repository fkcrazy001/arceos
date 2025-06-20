#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::time;

#[cfg(feature = "axstd")]
use axstd as std;

use std::println;
use std::thread::sleep;

use axconfig::{TASK_STACK_SIZE, plat::PHYS_VIRT_OFFSET};

use axhal::irq::{register_handler, set_enable};

/// magic number from dts
/// irq number 39 from teachers' doc
unsafe fn set_gpio_irq_enable() {
    let base_addr = (0x9030000 + PHYS_VIRT_OFFSET) as *mut u8;

    let pin = 3;
    let gpio_is = base_addr.add(0x404);
    // *gpio_is = 0;
    *gpio_is = *gpio_is & !(1 << pin);

    let gpio_iev = base_addr.add(0x40c);
    *gpio_iev = *gpio_iev & !(1 << pin);

    println!("GPIORIS={:#x}", *base_addr.add(0x414));

    let gpio_ie = base_addr.add(0x410);
    *gpio_ie = 0;
    *gpio_ie = *gpio_ie | (1 << pin);

    fn shut_down() {
        println!("shutdown function called");
        unsafe {
            let base_addr = (0x9030000 + PHYS_VIRT_OFFSET) as *mut u8;
            let pin = 3;
            // clear interrupt
            let gpio_ic = base_addr.add(0x41c);
            // *gpio_ic = *gpio_ic
            *gpio_ic = (1 << pin);
            core::arch::asm!(
                "mov w0, #0x18;
                hlt #0xf000"
            )
        }
    };
    register_handler(39, shut_down);
    set_enable(39, true);
    println!("set irq done");
}

#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    println!("Hello, world!");
    unsafe {
        set_gpio_irq_enable();
    }
    println!("loop started!");
    loop {
        sleep(time::Duration::from_millis(10));
    }
}
