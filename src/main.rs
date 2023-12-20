#![no_std]
#![no_main]

use esp_println::println;
// use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {
    }
}

// #[entry]
fn main() -> ! {
    // let peripherals = Peripherals::take();
    // let system = peripherals.SYSTEM.split();

    // let clocks = ClockControl::max(system.clock_control).freeze();
    // let mut delay = Delay::new(&clocks);

    println!("Hello world!");
    loop {
        println!("Loop...");
        // delay.delay_ms(500u32);
    }
}
