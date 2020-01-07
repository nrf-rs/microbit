#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use microbit::hal::i2c;
use microbit::hal::i2c::I2c;
use microbit::hal::nrf51::{interrupt, GPIOTE, UART0};
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::TWI1;

use crate::cortex_m::interrupt::Mutex;
use crate::cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use mag3110::{DataRate, Mag3110, Oversampling};

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static GPIO: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<UART0>>>> = Mutex::new(RefCell::new(None));
static MAG3110: Mutex<RefCell<Option<Mag3110<I2c<TWI1>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut cp)) = (microbit::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            /* Enable external GPIO interrupts */
            cp.NVIC.enable(microbit::Interrupt::GPIOTE);
            microbit::NVIC::unpend(microbit::Interrupt::GPIOTE);

            /* Set up pin 29 to act as external interrupt from the magnetometer */
            p.GPIOTE.config[0]
                .write(|w| unsafe { w.mode().event().psel().bits(29).polarity().lo_to_hi() });
            p.GPIOTE.intenset.write(|w| w.in0().set_bit());
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });
            *GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);

            /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure RX and TX pins accordingly */
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            /* Set up serial port using the prepared pins */
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

            let _ = write!(&mut tx, "\n\rWelcome to the magnetometer reader!\n\r");
            *TX.borrow(cs).borrow_mut() = Some(tx);

            /* Configure SCL and SDA pins accordingly */
            let scl = gpio.pin0.into_open_drain_input().downgrade();
            let sda = gpio.pin30.into_open_drain_input().downgrade();

            /* Set up I2C */
            let i2c = i2c::I2c::i2c1(p.TWI1, sda, scl);

            /* Set up MAG3110 magnetometer on the I2C bus */
            let mut mag3110 = Mag3110::new(i2c).ok().unwrap();

            /* Slow reading down a bit */
            let _ = mag3110.set_sampling_mode(DataRate::HZ20, Oversampling::OV128);

            /* Read a value so we know we can be sure to receive interrupts */
            let _ = mag3110.mag().ok().unwrap();
            *MAG3110.borrow(cs).borrow_mut() = Some(mag3110);
        });
    }

    loop {
        continue;
    }
}

// Define an interrupt handler, i.e. function to call when interrupt occurs. Here if we receive an
// internal interrupt from the magnetometer, we'll print out the readings
#[interrupt]
fn GPIOTE() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(gpiote), &mut Some(ref mut mag3110), &mut Some(ref mut tx)) = (
            GPIO.borrow(cs).borrow().as_ref(),
            MAG3110.borrow(cs).borrow_mut().deref_mut(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let (x, y, z) = mag3110.mag().ok().unwrap();
            let temp = mag3110.temp().ok().unwrap();

            /* Print read values on the serial console */
            let _ = write!(tx, "x: {}, y: {}, z: {}, temp: {}\n\r", x, y, z, temp);

            /* Clear event */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        }
    });

    loop {
        continue;
    }
}
