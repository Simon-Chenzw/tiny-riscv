#![no_main]
#![no_std]

use core::arch::asm;

fn ebreak() -> ! {
    unsafe { asm!("ebreak", options(noreturn)) }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    ebreak()
}

mod peripherals {
    pub mod pin_io {
        unsafe extern "C" {
            #[link_name = "pin_io"]
            static mut PinIOval: u8;
        }

        pub fn read() -> u8 {
            unsafe { core::ptr::read_volatile(&raw mut PinIOval) }
        }

        pub fn write(value: u8) {
            unsafe { core::ptr::write_volatile(&raw mut PinIOval, value) }
        }
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    loop {
        let mut a = peripherals::pin_io::read();
        let b = peripherals::pin_io::read();
        let mut div = 0;
        while a > b {
            a -= b;
            div += 1;
        }
        peripherals::pin_io::write(div);
        peripherals::pin_io::write(a);
    }
}
