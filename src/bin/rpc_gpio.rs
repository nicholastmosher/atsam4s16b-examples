#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;
extern crate capnp;

use cortex_m_rt::{entry, heap_start};
use cortex_m::asm;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

use atsam4s16b_embedded_hal as hal;
use self::hal::prelude::*;
use self::hal::atsam4s16b;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub mod embedded_capnp {
    include!(concat!(env!("OUT_DIR"), "/src/bin/gpio_capnp.rs"));
}

#[entry]
fn main() -> ! {
    let start = heap_start() as usize;
    let size = 1024;
    unsafe { ALLOCATOR.init(start, size) }

//    embedded_capnp::gpio::Builder::

    loop { }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop { }
}
