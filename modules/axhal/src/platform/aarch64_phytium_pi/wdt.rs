use tock_registers::interfaces::ReadWriteable;
use tock_registers::interfaces::Readable;
use tock_registers::interfaces::Writeable;
use tock_registers::register_bitfields;
use tock_registers::register_structs;
use tock_registers::registers::ReadOnly;
use tock_registers::registers::ReadWrite;

register_structs! {
    pub WdtRegisters {
        (0x0000 => wrr:ReadWrite<u32, DWORD::Register>),
        (0x0004 => _resv:[u8;0xfc8]),
        (0x0fcc => iidr:ReadWrite<u32, DWORD::Register>),
        (0x0fd0 => _resv4:[u8;48]),
        (0x1000 =>wcs:ReadWrite<u32,CONTROL::Register>),
        (0x1004 => _resv2:[u8;0x4]),
        (0x1008 =>wor:ReadWrite<u32,DWORD::Register>),
        (0x100c => _resv3:[u8;0xc]),
        (0x1018 => @END),
    }
}

register_bitfields![u32, DWORD[DATA OFFSET(0) NUMBITS(32)],
CONTROL[WDR_EN OFFSET(0) NUMBITS(1), WS0  OFFSET(1) NUMBITS(1), WS1 OFFSET(2) NUMBITS(1)]
];

impl WdtRegisters {
    pub fn new(base: usize) -> &'static mut Self {
        unsafe { &mut *(base as *mut WdtRegisters) }
    }

    pub fn init(&mut self, timeout: u32) {
        debug!("get before wor {:#x}", self.wor.get());
        debug!("get before wcs {:#x}", self.wcs.get());
        self.wor.set(timeout);
        self.wcs.modify(CONTROL::WDR_EN.val(1));

        debug!("get after wcs {:#x}", self.wcs.get());
        debug!("get after wrr {:#x}", self.wrr.get());
        debug!("get after iidr {:#x}", self.iidr.get());
        debug!("get after wor {:#x}", self.wor.get());
    }
}
use crate::mem::PhysAddr;
use crate::mem::phys_to_virt;
use kspin::SpinNoIrq;

const BASE0: PhysAddr = pa!(0x000_2804_0000);
const BASE1: PhysAddr = pa!(0x000_2804_2000);

pub fn init() {
    #[cfg(feature = "irq")]
    {
        info!(
            "enable irq {}",
            crate::irq::register_handler_common(196, timeout_wdt0)
        );
        info!(
            "enable irq {}",
            crate::irq::register_handler_common(197, timeout_wdt1)
        );
    }
    // wdt 0
    let wdt0 = WdtRegisters::new(phys_to_virt(BASE0).into());
    wdt0.init(0x3000000);
    // wdt 1
    let wdt1 = WdtRegisters::new(phys_to_virt(BASE1).into());
    wdt1.init(0x3000000);
}

fn timeout_wdt0() {
    let wdt0 = WdtRegisters::new(phys_to_virt(BASE0).into());
    debug!("get wdt0 wcs {:x}", wdt0.wcs.get());
}

fn timeout_wdt1() {
    let wdt1 = WdtRegisters::new(phys_to_virt(BASE1).into());
    debug!("get wdt1 wcs {:x}", wdt1.wcs.get());
}
