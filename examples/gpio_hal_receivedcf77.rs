#![no_main]
#![no_std]

use panic_halt as _;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use dcf77::{DCF77Time, SimpleDCF77Decoder};
use microbit::{
    hal::{
        self,
        gpio::{p0::P0_16, Floating, Input},
        prelude::*,
        uart::{Baudrate, Uart},
    },
    pac::{self, interrupt},
};

static DCF: Mutex<RefCell<Option<SimpleDCF77Decoder>>> = Mutex::new(RefCell::new(None));
static DCFPIN: Mutex<RefCell<Option<P0_16<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<pac::RTC0>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Uart<pac::UART0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            // TODO: check if there are safe wrappers now
            p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

            while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

            /* Split GPIO pins */
            let gpio = hal::gpio::p0::Parts::new(p.GPIO);

            /* Configure DCF77 receiver GPIO as input */
            let pin = gpio.p0_16.into_floating_input();

            /* Initialise serial port on the micro:bit */
            let mut serial = microbit::serial_port!(gpio, p.UART0, Baudrate::BAUD115200);

            let _ = serial.write_str("\n\rWelcome to the DCF77 decoder demo.\n\r");
            let _ = serial.write_str("If you are within reach of a DCF77 radio clock signal and have a DCF77 receiver connected\n\r");
            let _ = serial.write_str("you should see a stream of 59 bits appear in a line, followed by thd decoded date and time.\n\r");
            let _ =
                serial.write_str("If not, please check your hardware, location and reception.\n\r");
            // TODO: check for safe wrappers
            p.RTC0.prescaler.write(|w| unsafe { w.bits(327) });
            p.RTC0.evtenset.write(|w| w.tick().set_bit());
            p.RTC0.intenset.write(|w| w.tick().set_bit());
            p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *TX.borrow(cs).borrow_mut() = Some(serial);
            *DCFPIN.borrow(cs).borrow_mut() = Some(pin);
            *DCF.borrow(cs).borrow_mut() = Some(SimpleDCF77Decoder::new());

            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC0);
            }
            pac::NVIC::unpend(pac::Interrupt::RTC0);
        });
    }

    loop {
        continue;
    }
}

// Define an interrupt handler, i.e. function to call when interrupt occurs. Here if our timer
// trips, we'll process data from the DC77 module
#[interrupt]
fn RTC0() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut tx), &mut Some(ref mut pin), &mut Some(ref mut dcf)) = (
            RTC.borrow(cs).borrow().as_ref(),
            TX.borrow(cs).borrow_mut().deref_mut(),
            DCFPIN.borrow(cs).borrow_mut().deref_mut(),
            DCF.borrow(cs).borrow_mut().deref_mut(),
        ) {
            dcf.read_bit(pin.is_low().unwrap());
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
