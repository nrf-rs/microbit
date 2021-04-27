#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

#[cfg(feature = "v1")]
#[entry]
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

            for _ in 0..50_000 {
                cortex_m::asm::nop();
            }
        }
    };

    loop {
        continue;
    }
}

#[cfg(feature = "v2")]
#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.P0.pin_cnf[28].write(|w| w.dir().output());
        p.P0.pin_cnf[21].write(|w| w.dir().output());

        p.P0.out.write(|w| unsafe { w.bits(1 << 21) });

        let mut count: u8 = 0;
        loop {
            count += 1;

            if count & 1 == 1 {
                p.P0.out.write(|w| unsafe { w.bits(1 << 21) });
            } else {
                p.P0.out.write(|w| unsafe { w.bits(0) });
            }

            for _ in 0..50_000 {
                cortex_m::asm::nop();
            }
        }
    };

    loop {
        continue;
    }
}
