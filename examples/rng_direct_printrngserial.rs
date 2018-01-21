#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
use cortex_m::peripheral::Peripherals;
use cortex_m::interrupt::Mutex;

#[macro_use]
extern crate microbit;
use microbit::peripherals::uart;

use core::cell::RefCell;
use core::fmt::Write;


static RNG: Mutex<RefCell<Option<microbit::RNG>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<microbit::RTC0>>> = Mutex::new(RefCell::new(None));
static UART: Mutex<RefCell<Option<microbit::UART0>>> = Mutex::new(RefCell::new(None));


fn main() {
    if let Some(p) = microbit::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

        /* And den set it back to 0 again, just because ?!? */
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
        p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

        p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
        p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

        p.UART0.baudrate.write(|w| w.baudrate().baud115200());
        p.UART0.enable.write(|w| w.enable().enabled());

        let _ = write!(
            uart::UART0Buffer(&p.UART0),
            "\n\rWelcome to the random number printer!\n\r"
        );

        p.RTC0.prescaler.write(|w| unsafe { w.bits(4095) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        p.RNG.tasks_start.write(|w| unsafe { w.bits(1) });

        cortex_m::interrupt::free(move |cs| {
            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *RNG.borrow(cs).borrow_mut() = Some(p.RNG);
            *UART.borrow(cs).borrow_mut() = Some(p.UART0);
        });

        if let Some(mut p) = Peripherals::take() {
            p.NVIC.enable(microbit::Interrupt::RTC0);
            p.NVIC.clear_pending(microbit::Interrupt::RTC0);
        }
    }
}


/* Define an exception, i.e. function to call when exception occurs. Here if our SysTick timer
 * trips the hello_world function will be called */
interrupt!(RTC0, printrng);


fn printrng() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
        let count = if let Some(rng) = RNG.borrow(cs).borrow().as_ref() {
            rng.value.read().bits()
        } else {
            0
        };

        if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
            let _ = write!(uart::UART0Buffer(uart), "{}\n\r", count);
        }
        rtc.events_tick.write(|w| unsafe { w.bits(0) });
    });
}
