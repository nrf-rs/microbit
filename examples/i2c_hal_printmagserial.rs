#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_abort;

#[macro_use]
extern crate microbit;

use microbit::cortex_m;
use microbit::hal::i2c;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m_rt::entry;

static RTC: Mutex<RefCell<Option<microbit::RTC0>>> = Mutex::new(RefCell::new(None));
static I2C: Mutex<RefCell<Option<i2c::I2c<microbit::TWI1>>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<microbit::UART0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        p.RTC0.prescaler.write(|w| unsafe { w.bits(4095) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        cortex_m::interrupt::free(move |cs| {
            /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure RX and TX pins accordingly */
            let scl = gpio.pin0.into_open_drain_input().downgrade();
            let sda = gpio.pin30.into_open_drain_input().downgrade();

            let mut i2c = i2c::I2c::i2c1(p.TWI1, sda, scl);

            /* Configure magnetometer for automatic updates */
            let _ = i2c.write(0xE, &[0x10, 0x1]);
            let _ = i2c.write(0xE, &[0x11, 0x7f]);

            /* Initialise serial port on the micro:bit */
            //let (mut tx, _) = microbit::serial_port(gpio, p.UART0, BAUD115200);

            /* Configure RX and TX pins accordingly */
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            /* Set up serial port using the prepared pins */
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

            let _ = write!(&mut tx, "\n\rWelcome to the magnetometer reader!\n\r");

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *I2C.borrow(cs).borrow_mut() = Some(i2c);
            *TX.borrow(cs).borrow_mut() = Some(tx);
        });

        if let Some(mut p) = Peripherals::take() {
            p.NVIC.enable(microbit::Interrupt::RTC0);
            microbit::NVIC::unpend(microbit::Interrupt::RTC0);
        }
    }

    loop {}
}

/* Define an exception, i.e. function to call when exception occurs. Here if our SysTick timer
 * trips the printmag function will be called */
interrupt!(RTC0, printmag);

fn printmag() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut i2c), &mut Some(ref mut tx)) = (
            RTC.borrow(cs).borrow().as_ref(),
            I2C.borrow(cs).borrow_mut().deref_mut(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let mut data: [u8; 6] = [0; 6];

            if i2c.write_read(0xE, &[0x1], &mut data).is_ok() {
                /* Join and translate 2s complement values */
                let (x, y, z) = (
                    (u16::from(data[0]) << 8 | u16::from(data[1])) as i16,
                    (u16::from(data[2]) << 8 | u16::from(data[3])) as i16,
                    (u16::from(data[4]) << 8 | u16::from(data[5])) as i16,
                );

                /* Print read values on the serial console */
                let _ = write!(tx, "x: {}, y: {}, z: {}\n\r", x, y, z);
            }

            /* Clear timer event */
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}
