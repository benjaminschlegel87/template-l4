#![no_main]
#![no_std]

use core::panic::PanicInfo;
use defmt::println;
use defmt_rtt as _;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::{self as _, rcc::RccExt};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = stm32l4xx_hal::pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb2);
    let mut pc15 = gpioc
        .pc15
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    pc15.set_high();
    loop {
        println!("Set High");
        for _i in 0..100_000 {
            cortex_m::asm::nop();
        }
        println!("Set Low");
        pc15.set_low();
        for _i in 0..100_000 {
            cortex_m::asm::nop();
        }
        pc15.set_high();
    }
}
