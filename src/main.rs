#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal;
use panic_halt;
use stm32f484::{delay::Delay, pac, prelude::*};
use stm32g4::stm32g484;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32g484::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());
    while (true) {}
}
