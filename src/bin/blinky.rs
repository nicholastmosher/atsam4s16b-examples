#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use atsam4s16b_embedded_hal as hal;

use self::hal::prelude::*;
use self::hal::atsam4s16b;

#[entry]
fn main() -> ! {
    let dp = atsam4s16b::Peripherals::take().unwrap();
    let pioa = dp.PIOA.split();

    let mut oer = pioa.oer;
    let mut led = pioa.pa22.into_output(&mut oer);

    loop {
        for _ in 0..100_000 {
            led.set_high();
        }
        for _ in 0..100_000 {
            led.set_low();
        }
    }
}
