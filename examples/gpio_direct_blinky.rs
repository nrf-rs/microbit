#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

extern crate microbit;
extern crate panic_abort;
use microbit::*;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}
entry!(main);

fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.GPIO.pin_cnf[4].write(|w| w.dir().output());
        p.GPIO.pin_cnf[13].write(|w| w.dir().output());

        p.GPIO.out.write(|w| unsafe { w.bits(1 << 13) });

        let mut count: u8 = 0;
        loop {
            count += 1;

            if count & 1 == 1 {
                p.GPIO.out.write(|w| unsafe { w.bits(1 << 13) });
            } else {
                p.GPIO.out.write(|w| unsafe { w.bits(0) });
            }

            for _ in 0..1_000_000 {
                cortex_m::asm::nop();
            }
        }
    };

    loop {}
}
