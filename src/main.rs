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
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);
    let mut pb13 = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    pb13.set_high();
    loop {
        println!("Set High");
        for _i in 0..100_000 {
            cortex_m::asm::nop();
        }
        println!("Set Low");
        pb13.set_low();
        for _i in 0..100_000 {
            cortex_m::asm::nop();
        }
        pb13.set_high();
    }
}
