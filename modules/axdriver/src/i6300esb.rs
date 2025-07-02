use core::{
    ptr::{read_volatile, write_volatile},
    time,
};

use axdriver_base::BaseDriverOps;
use axdriver_pci::{Command, DeviceFunction, DeviceFunctionInfo, PciRoot};

use crate::{
    AxDeviceEnum,
    drivers::{AxWdtDevice, DriverProbe},
};

#[derive(Debug)]
pub struct I6300esb {
    base_va: usize,
    size: usize,
}

impl BaseDriverOps for I6300esb {
    fn device_name(&self) -> &str {
        "i6300esb"
    }
    fn device_type(&self) -> axdriver_base::DeviceType {
        axdriver_base::DeviceType::Char
    }
}

use axhal::mem::phys_to_virt;

impl DriverProbe for I6300esb {
    #[cfg(bus = "pci")]
    fn probe_pci(
        root: &mut PciRoot,
        bdf: DeviceFunction,
        dev_info: &DeviceFunctionInfo,
    ) -> Option<AxDeviceEnum> {
        // axdriver_net
        if dev_info.vendor_id == 0x8086 && dev_info.device_id == 0x25ab {
            info!("{}", bdf);
            info!("{:?}", root);
            match root.bar_info(bdf, 0).unwrap() {
                axdriver_pci::BarInfo::Memory {
                    address,
                    size,
                    address_type,
                    ..
                } => {
                    let dev = I6300esb {
                        base_va: phys_to_virt((address as usize).into()).into(),
                        size: size as usize,
                    };
                    info!("address_type={:?}, {:#x?}", address_type, dev);

                    info!("{:?}", root.get_status_command(bdf));

                    root.config_write_word(bdf, 0x60, 0x03);

                    let data = root.config_read_word(bdf, 0x68);
                    info!("wdt lock = {data}");

                    // write bar and wait for timeout
                    set_up_watch_dog(&dev);

                    root.config_write_word(bdf, 0x68, 0x2);
                    let data = root.config_read_word(bdf, 0x68);
                    info!("wdt lock = {data}");

                    let data = root.config_read_word(bdf, 0x60);
                    info!("wcr  = {data}");

                    // root.config_write_word(bdf, 0x60, 0x00);

                    return Some(AxDeviceEnum::Wdt(dev));
                }
                axdriver_pci::BarInfo::IO { .. } => {
                    error!("wdt: BAR0 is of I/O type");
                    return None;
                }
            }
        }
        None
    }
}

pub fn set_up_watch_dog(dev: &I6300esb) {
    let va = dev.base_va as *mut u32;
    let unlock_reg = move || unsafe {
        let wa = va.add(3) as *mut u16;
        info!("va={:?}", va.add(3));
        write_volatile(wa, 0x80);
        write_volatile(wa, 0x86);
    };
    info!("va={:?}", va);
    unsafe {
        // pre load value 1
        info!("p1 {}", read_volatile(va));
        info!("p2 {:x}", read_volatile(va.add(1)));
        info!("gis {:x}", read_volatile(va.add(2)));

        info!("rr {:x}", read_volatile(va.add(3)));
        // pre load value 2

        unlock_reg();
        write_volatile(va.add(3) as *mut u16, 3 << 8);

        unlock_reg();
        write_volatile(va, 1);
        unlock_reg();
        write_volatile(va.add(1), 1);
        // reload
        unlock_reg();
        write_volatile(va.add(3) as *mut u16, 1 << 8);
        // pre load value 1
        info!("p1 {}", read_volatile(va));
        info!("p2 {:x}", read_volatile(va.add(1)));
        info!("gis {:x}", read_volatile(va.add(2)));

        info!("rr {:x}", read_volatile(va.add(3)));
    }
    fn timeout() {
        error!("watch dog timeout!");
    }

    axhal::irq::register_handler(43, timeout);
    axhal::irq::register_handler(42, timeout);
    axhal::irq::register_handler(10, timeout);
    axhal::irq::register_handler(0x3c, timeout);
    debug!("done set watch dog");
}
