#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_abort;

#[macro_use]
extern crate microbit;

extern crate dcf77;

use dcf77::{DCF77Time, SimpleDCF77Decoder};

use microbit::cortex_m;
use microbit::hal::gpio::gpio::PIN16;
use microbit::hal::gpio::{Floating, Input};
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static DCF: Mutex<RefCell<Option<SimpleDCF77Decoder>>> = Mutex::new(RefCell::new(None));
static DCFPIN: Mutex<RefCell<Option<PIN16<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<microbit::RTC0>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<microbit::UART0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut cp)) = (microbit::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

            while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

            /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure DCF77 receiver GPIO as input */
            let pin = gpio.pin16.into_floating_input();

            /* Configure RX and TX pins accordingly */
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            /* Set up serial port using the prepared pins */
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

            let _ = tx.write_str("\n\rWelcome to the DCF77 decoder demo.\n\r");
            let _ = tx.write_str("If you are within reach of a DCF77 radio clock signal and have a DCF77 receiver connected\n\r");
            let _ = tx.write_str("you should see a stream of 59 bits appear in a line, followed by thd decoded date and time.\n\r");
            let _ = tx.write_str("If not, please check your hardware, location and reception.\n\r");
            p.RTC0.prescaler.write(|w| unsafe { w.bits(327) });
            p.RTC0.evtenset.write(|w| w.tick().set_bit());
            p.RTC0.intenset.write(|w| w.tick().set_bit());
            p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *TX.borrow(cs).borrow_mut() = Some(tx);
            *DCFPIN.borrow(cs).borrow_mut() = Some(pin);
            *DCF.borrow(cs).borrow_mut() = Some(SimpleDCF77Decoder::new());

            cp.NVIC.enable(microbit::Interrupt::RTC0);
            microbit::NVIC::unpend(microbit::Interrupt::RTC0);
        });
    }

    loop {}
}

/* Define an exception, i.e. function to call when exception occurs. Here if our SysTick timer
 * trips the hello_world function will be called */
interrupt!(RTC0, printrng);

fn printrng() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut tx), &mut Some(ref mut pin), &mut Some(ref mut dcf)) = (
            RTC.borrow(cs).borrow().as_ref(),
            TX.borrow(cs).borrow_mut().deref_mut(),
            DCFPIN.borrow(cs).borrow_mut().deref_mut(),
            DCF.borrow(cs).borrow_mut().deref_mut(),
        ) {
            dcf.read_bit(pin.is_low());
            if dcf.bit_faulty() {
                let _ = tx.write_str("F");
            } else if dcf.bit_complete() {
                let bit = dcf.latest_bit();
                let _ = tx.write_str(if bit { "1" } else { "0" });
            }

            if dcf.end_of_cycle() {
                let raw_data = dcf.raw_data();

                let dcftime = DCF77Time(raw_data);
                if let Ok((year, month, day, _)) = dcftime.date() {
                    let _ = write!(
                        tx,
                        "\n\r{:4}-{:02}-{:02} ",
                        year as usize, month as usize, day as usize
                    );
                } else {
                    let _ = tx.write_str("\n\rXXXX:XX:XX");
                }

                if let Ok(hours) = dcftime.hours() {
                    let _ = write!(tx, "{:02}", hours as usize);
                } else {
                    let _ = tx.write_str("XX");
                }

                let _ = tx.write_str(":");

                if let Ok(minutes) = dcftime.minutes() {
                    let _ = write!(tx, "{:02}", minutes as usize);
                } else {
                    let _ = tx.write_str("XX");
                }

                let _ = tx.write_str(":");

                let _ = write!(tx, "{:02}\n\r", dcf.seconds() as usize);
            }
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}
