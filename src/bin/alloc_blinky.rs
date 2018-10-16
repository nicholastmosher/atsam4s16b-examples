#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;

use cortex_m_rt::{entry, heap_start};
use cortex_m::asm;
use atsam4s16b_embedded_hal as hal;

use core::alloc::Layout;
use self::alloc::vec;
use alloc_cortex_m::CortexMHeap;

use self::hal::prelude::*;
use self::hal::atsam4s16b;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let start = heap_start() as usize;
    let size = 1024;
    unsafe { ALLOCATOR.init(start, size) }

    let delay_seconds = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let dp = atsam4s16b::Peripherals::take().unwrap();
    let pioa = dp.PIOA.split();

    let mut oer = pioa.oer;
    let mut led = pioa.pa22.into_output(&mut oer);

    loop {
        for time in delay_seconds.iter() {
            for _ in 0..(time * 100_000) {
                led.set_high();
            }
            for _ in 0..(time * 100_000) {
                led.set_low();
            }
        }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop { }
}
