#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::{ptr::NonNull, time};

#[cfg(feature = "axstd")]
use axstd::println;

use axhal::misc::PwmCtrl;
use axhal::misc::Tacho;
use axhal::misc::phys_to_virt;
use axstd::thread;
use memory_addr::pa;
#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    println!("Hello, world!");
    let mut pwm = init_pwm();
    init_pad();
    let tacho = init_tacho();
    let mut i = 0;
    let d = [50, 60, 70, 80, 90, 100];
    loop {
        pwm.change_duty(d[i]);
        thread::sleep(time::Duration::from_secs(2));
        if let Some(u) = tacho.get_result() {
            println!("current duty is {} , fan speed is {u}", d[i]);
        }
        i = (i + 1) % d.len();
    }
}

fn usize_to_va(addr: usize) -> NonNull<u8> {
    let p = pa!(addr);
    let va = phys_to_virt(p).as_usize() as *mut u8;
    NonNull::new(va).unwrap()
}

fn init_pwm() -> PwmCtrl {
    let mut pwm = PwmCtrl::new(usize_to_va(PWM1));
    pwm.init();
    pwm
}

fn init_tacho() -> Tacho {
    let mut tacho = Tacho::new(usize_to_va(TACHO2));
    tacho.init();
    tacho
}

fn init_pad() {
    unsafe {
        let mut ag59_reg0: NonNull<u32> = unsafe { usize_to_va(PAD).add(0x5c).cast() };

        let cfg = ag59_reg0.read_volatile();
        let new_cfg = (cfg & !0b111) | 0b001;
        ag59_reg0.write_volatile(new_cfg);
    }
}

const PWM1: usize = 0x000_2804_A000 + 0x400;
const TACHO2: usize = 0x000_2805_6000;
const PAD: usize = 0x000_32B3_0000;
