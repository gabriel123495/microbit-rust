#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm::nop;
use cortex_m_rt::entry;

use embedded_hal::digital::OutputPin;

use nrf52833_hal as hal;
use hal::gpio::Level;
use hal::pac;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let mut col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);

    loop {
        row1.set_high().unwrap();
        col1.set_low().unwrap();

        for _ in 0..200_000 {
            nop();
        }

        row1.set_low().unwrap();
        col1.set_high().unwrap();

        for _ in 0..200_000 {
            nop();
        }
    }
}
