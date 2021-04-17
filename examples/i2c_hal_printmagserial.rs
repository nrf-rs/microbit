#![no_main]
#![no_std]

use panic_halt as _;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{
    hal::{
        self, twi,
        uart::{Baudrate, Uart},
    },
    pac::{self, interrupt, twi0::frequency::FREQUENCY_A},
};

static RTC: Mutex<RefCell<Option<pac::RTC0>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Uart<pac::UART0>>>> = Mutex::new(RefCell::new(None));
static I2C: Mutex<RefCell<Option<twi::Twi<pac::TWI1>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        // TODO: check if there are safe wrappers for this now
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        p.RTC0.prescaler.write(|w| unsafe { w.bits(4095) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        cortex_m::interrupt::free(move |cs| {
            /* Split GPIO pins */
            let gpio = hal::gpio::p0::Parts::new(p.GPIO);

            /* Set up I2C */
            let twi_pins = twi::Pins {
                scl: gpio.p0_00.into_floating_input().degrade(),
                sda: gpio.p0_30.into_floating_input().degrade(),
            };
            let mut i2c = twi::Twi::new(p.TWI1, twi_pins, FREQUENCY_A::K250);

            /* Configure magnetometer for automatic updates */
            let _ = i2c.write(0xE, &[0x10, 0x1]);
            let _ = i2c.write(0xE, &[0x11, 0x7f]);

            /* Initialise serial port on the micro:bit */
            let mut serial = microbit::serial_port!(gpio, p.UART0, Baudrate::BAUD115200);

            let _ = write!(&mut serial, "\n\rWelcome to the magnetometer reader!\n\r");

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *I2C.borrow(cs).borrow_mut() = Some(i2c);
            *TX.borrow(cs).borrow_mut() = Some(serial);
        });
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::RTC0);
        }
        pac::NVIC::unpend(pac::Interrupt::RTC0);
    }

    loop {
        continue;
    }
}

// Define an interrupt handler, i.e. function to call when interrupt occurs. Here if our
// timer trips, we'll print out the readings from the magnetometer
#[interrupt]
fn RTC0() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut i2c), &mut Some(ref mut tx)) = (
            RTC.borrow(cs).borrow().as_ref(),
            I2C.borrow(cs).borrow_mut().deref_mut(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let mut data: [u8; 6] = [0; 6];

            if i2c.write_then_read(0xE, &[0x1], &mut data).is_ok() {
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
