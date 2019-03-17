#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use microbit::display::image::GreyscaleImage;
use microbit::display::{self, Display, Frame, MicrobitFrame};
use microbit::hal::nrf51::{interrupt, GPIO, RTC0, TIMER1};

fn heart_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
}

// We use TIMER1 to drive the display, and RTC0 to update the animation.
// We set the TIMER1 interrupt to a higher priority than RTC0.

static GPIO: Mutex<RefCell<Option<GPIO>>> = Mutex::new(RefCell::new(None));
static RTC0: Mutex<RefCell<Option<RTC0>>> = Mutex::new(RefCell::new(None));
static TIMER1: Mutex<RefCell<Option<TIMER1>>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display<MicrobitFrame>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(mut p) = microbit::Peripherals::take() {
        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        // 16Hz; 62.5ms period
        p.RTC0.prescaler.write(|w| unsafe { w.bits(2047) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        display::initialise_display(&mut p.TIMER1, &mut p.GPIO);

        cortex_m::interrupt::free(move |cs| {
            *GPIO.borrow(cs).borrow_mut() = Some(p.GPIO);
            *RTC0.borrow(cs).borrow_mut() = Some(p.RTC0);
            *TIMER1.borrow(cs).borrow_mut() = Some(p.TIMER1);
            *DISPLAY.borrow(cs).borrow_mut() = Some(Display::new());
        });

        if let Some(mut cp) = Peripherals::take() {
            unsafe {
                cp.NVIC.set_priority(microbit::Interrupt::RTC0, 64);
                cp.NVIC.set_priority(microbit::Interrupt::TIMER1, 128);
            }
            cp.NVIC.enable(microbit::Interrupt::RTC0);
            cp.NVIC.enable(microbit::Interrupt::TIMER1);
            microbit::NVIC::unpend(microbit::Interrupt::RTC0);
            microbit::NVIC::unpend(microbit::Interrupt::TIMER1);
        }
    }

    loop {
        continue;
    }
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(timer1) = TIMER1.borrow(cs).borrow_mut().as_mut() {
            if let Some(gpio) = GPIO.borrow(cs).borrow_mut().as_mut() {
                if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    display::handle_display_event(d, timer1, gpio);
                }
            }
        }
    });
}

#[interrupt]
fn RTC0() {
    static mut STEP: u8 = 0;
    static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();

    cortex_m::interrupt::free(|cs| {
        if let Some(rtc0) = RTC0.borrow(cs).borrow().as_ref() {
            rtc0.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });

    let inner_brightness = match *STEP {
        0..=8 => 9 - *STEP,
        9..=12 => 0,
        _ => unreachable!(),
    };

    FRAME.set(&mut heart_image(inner_brightness));

    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.set_frame(&FRAME);
        }
    });

    *STEP += 1;
    if *STEP == 13 {
        *STEP = 0
    };
}
