#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp_println::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "Rust" fn _setup_interrupts() {
    // nothing
}

use riscv_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    println!("Hello world! PF 2024 :-)");

    loop {}
}
