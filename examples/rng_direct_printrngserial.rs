#![no_main]
#![no_std]

use panic_halt;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use microbit::hal::nrf51::{interrupt, RNG, RTC0, UART0};

use core::cell::RefCell;
use core::fmt::Write;
use cortex_m_rt::entry;

static RNG: Mutex<RefCell<Option<RNG>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<RTC0>>> = Mutex::new(RefCell::new(None));
static UART: Mutex<RefCell<Option<UART0>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

        /* And then set it back to 0 again, just because ?!? */
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
        p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

        p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
        p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

        p.UART0.baudrate.write(|w| w.baudrate().baud115200());
        p.UART0.enable.write(|w| w.enable().enabled());

        let _ = write!(
            UART0Buffer(&p.UART0),
            "\n\rWelcome to the random number printer!\n\r"
        );

        p.RTC0.prescaler.write(|w| unsafe { w.bits(1) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        /* Enable error correction for better values */
        p.RNG.config.write(|w| w.dercen().enabled());

        /* Enable random number generation */
        p.RNG.tasks_start.write(|w| unsafe { w.bits(1) });

        cortex_m::interrupt::free(move |cs| {
            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *RNG.borrow(cs).borrow_mut() = Some(p.RNG);
            *UART.borrow(cs).borrow_mut() = Some(p.UART0);
        });

        if let Some(mut p) = Peripherals::take() {
            p.NVIC.enable(microbit::Interrupt::RTC0);
            microbit::NVIC::unpend(microbit::Interrupt::RTC0);
        }
    }

    loop {
        continue;
    }
}

// Define an exception, i.e. function to call when exception occurs. Here if our timer
// trips, we'll print some random number to the serial port
#[interrupt]
fn RTC0() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
            let count = if let Some(rng) = RNG.borrow(cs).borrow().as_ref() {
                /* Let's wait until we have a new random value */
                while rng.events_valrdy.read().bits() == 0 {}

                let num = rng.value.read().bits();

                /* Clear event for next random number value */
                rng.events_valrdy.write(|w| unsafe { w.bits(0) });

                num
            } else {
                0
            };

            if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
                let _ = write!(UART0Buffer(uart), "{}\n\r", count);
            }
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}

pub struct UART0Buffer<'a>(pub &'a UART0);

impl<'a> core::fmt::Write for UART0Buffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let uart0 = self.0;
        uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
        for c in s.as_bytes() {
            /* Write the current character to the output register */
            uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

            /* Wait until the UART is clear to send */
            while uart0.events_txdrdy.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
        }
        uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
        Ok(())
    }
}
