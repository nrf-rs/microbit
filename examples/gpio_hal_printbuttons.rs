#![no_main]
#![no_std]

use panic_halt as _;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{
    hal::{
        self,
        uart::{Baudrate, Uart},
    },
    pac::{self, interrupt},
};

static GPIO: Mutex<RefCell<Option<pac::GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Uart<pac::UART0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            /* Enable external GPIO interrupts */
            unsafe {
                pac::NVIC::unmask(pac::Interrupt::GPIOTE);
            }
            pac::NVIC::unpend(pac::Interrupt::GPIOTE);

            /* Split GPIO pins */
            let gpio = hal::gpio::p0::Parts::new(p.GPIO);

            /* Configure button GPIOs as inputs */
            let _ = gpio.p0_26.into_floating_input();
            let _ = gpio.p0_17.into_floating_input();

            // TODO: check for safe wrappers
            /* Set up GPIO 17 (button A) to generate an interrupt when pulled down */
            p.GPIOTE.config[0]
                .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in0().set_bit());
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

            /* Set up GPIO 26 (button B) to generate an interrupt when pulled down */
            p.GPIOTE.config[1]
                .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in1().set_bit());
            p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

            *GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);

            /* Initialise serial port on the micro:bit */
            let mut serial = microbit::serial_port!(gpio, p.UART0, Baudrate::BAUD115200);

            let _ = write!(
                serial,
                "\n\rWelcome to the buttons demo. Press buttons A and/or B for some action.\n\r",
            );
            *TX.borrow(cs).borrow_mut() = Some(serial);
        });
    }

    loop {
        continue;
    }
}

// Define an interrupt, i.e. function to call when exception occurs. Here if we receive an
// interrupt from a button press, the function will be called
#[interrupt]
fn GPIOTE() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(gpiote), &mut Some(ref mut tx)) = (
            GPIO.borrow(cs).borrow().as_ref(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let buttonapressed = gpiote.events_in[0].read().bits() != 0;
            let buttonbpressed = gpiote.events_in[1].read().bits() != 0;

            /* Print buttons to the serial console */
            let _ = write!(
                tx,
                "Button pressed {}\n\r",
                match (buttonapressed, buttonbpressed) {
                    (false, false) => "",
                    (true, false) => "A",
                    (false, true) => "B",
                    (true, true) => "A + B",
                }
            );

            /* Clear events */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
        }
    });
}
