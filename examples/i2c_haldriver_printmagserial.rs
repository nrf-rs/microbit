#![no_main]
#![no_std]

use panic_halt as _;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

use microbit::{
    hal::{
        self, twi,
        uart::{Baudrate, Uart},
    },
    pac::{self, interrupt, twi0::frequency::FREQUENCY_A},
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use mag3110::{DataRate, Mag3110, Oversampling};

static GPIO: Mutex<RefCell<Option<pac::GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Uart<pac::UART0>>>> = Mutex::new(RefCell::new(None));
static MAG3110: Mutex<RefCell<Option<Mag3110<twi::Twi<pac::TWI1>>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            /* Enable external GPIO interrupts */
            unsafe {
                pac::NVIC::unmask(pac::Interrupt::GPIOTE);
            }
            pac::NVIC::unpend(pac::Interrupt::GPIOTE);

            /* Set up pin 29 to act as external interrupt from the magnetometer */
            p.GPIOTE.config[0]
                .write(|w| unsafe { w.mode().event().psel().bits(29).polarity().lo_to_hi() });
            p.GPIOTE.intenset.write(|w| w.in0().set_bit());
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });
            *GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);

            let gpio = hal::gpio::p0::Parts::new(p.GPIO);
            let mut serial = microbit::serial_port!(gpio, p.UART0, Baudrate::BAUD115200);

            let _ = write!(&mut serial, "\n\rWelcome to the magnetometer reader!\n\r");
            *TX.borrow(cs).borrow_mut() = Some(serial);

            /* Set up I2C */
            let twi_pins = twi::Pins {
                scl: gpio.p0_00.into_floating_input().degrade(),
                sda: gpio.p0_30.into_floating_input().degrade(),
            };
            let i2c = twi::Twi::new(p.TWI1, twi_pins, FREQUENCY_A::K250);

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
