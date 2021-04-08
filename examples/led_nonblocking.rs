#![no_main]
#![no_std]

use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use microbit::display::image::GreyscaleImage;
use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};
use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
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
static ANIM_TIMER: Mutex<RefCell<Option<LoResTimer<RTC0>>>> = Mutex::new(RefCell::new(None));
static DISPLAY_TIMER: Mutex<RefCell<Option<MicrobitDisplayTimer<TIMER1>>>> =
    Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display<MicrobitFrame>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        cortex_m::interrupt::free(move |cs| {
            let mut rtc0 = LoResTimer::new(p.RTC0);
            // 62.5ms period
            rtc0.set_frequency(FREQ_16HZ);
            rtc0.enable_tick_event();
            rtc0.enable_tick_interrupt();
            rtc0.start();

            let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
            let mut gpio = p.GPIO;
            display::initialise_display(&mut timer, &mut gpio);
            *GPIO.borrow(cs).borrow_mut() = Some(gpio);
            *ANIM_TIMER.borrow(cs).borrow_mut() = Some(rtc0);
            *DISPLAY_TIMER.borrow(cs).borrow_mut() = Some(timer);
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
        if let Some(timer) = DISPLAY_TIMER.borrow(cs).borrow_mut().as_mut() {
            if let Some(gpio) = GPIO.borrow(cs).borrow_mut().as_mut() {
                if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    display::handle_display_event(d, timer, gpio);
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
        if let Some(rtc) = ANIM_TIMER.borrow(cs).borrow_mut().as_mut() {
            rtc.clear_tick_event();
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
