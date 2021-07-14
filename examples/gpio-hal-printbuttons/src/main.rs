#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::gpiote::Gpiote,
    pac::{self, interrupt},
};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let gpiote = Gpiote::new(board.GPIOTE);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    cortex_m::interrupt::free(move |cs| {
        /* Enable external GPIO interrupts */
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);

        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        defmt::info!("Welcome to the buttons demo. Press buttons A and/or B for some action.");
    });

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
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let buttonapressed = gpiote.channel0().is_event_triggered();
            let buttonbpressed = gpiote.channel1().is_event_triggered();

            /* Print buttons to the serial console */
            defmt::info!(
                "Button pressed {:?}",
                match (buttonapressed, buttonbpressed) {
                    (false, false) => "",
                    (true, false) => "A",
                    (false, true) => "B",
                    (true, true) => "A + B",
                }
            );

            /* Clear events */
            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    });
}
