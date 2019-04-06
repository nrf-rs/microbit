//! A complete working example.
//!
//! This requires `cortex-m-rtfm` v0.4.1.
//!
//! It uses `TIMER1` to drive the display, and `RTC0` to update a simple
//! animated image.
//!
//! A version of this code which doesn't use `cortex-m-rtfm` is available as
//! `examples/led_nonblocking.rs`.
//!
//! ```
//! #![no_main]
//! #![no_std]
//!
//! #[allow(unused)]
//! use panic_halt;
//!
//! use microbit::display::image::GreyscaleImage;
//! use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};
//! use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
//! use microbit::hal::nrf51;
//! use rtfm::app;
//!
//! fn heart_image(inner_brightness: u8) -> GreyscaleImage {
//!     let b = inner_brightness;
//!     GreyscaleImage::new(&[
//!         [0, 7, 0, 7, 0],
//!         [7, b, 7, b, 7],
//!         [7, b, b, b, 7],
//!         [0, 7, b, 7, 0],
//!         [0, 0, 7, 0, 0],
//!     ])
//! }
//!
//! #[app(device = microbit::hal::nrf51)]
//! const APP: () = {
//!     static mut GPIO: nrf51::GPIO = ();
//!     static mut DISPLAY_TIMER: MicrobitDisplayTimer<nrf51::TIMER1> = ();
//!     static mut ANIM_TIMER: LoResTimer<nrf51::RTC0> = ();
//!     static mut DISPLAY: Display<MicrobitFrame> = ();
//!
//!     #[init]
//!     fn init() -> init::LateResources {
//!         let mut p: nrf51::Peripherals = device;
//!
//!         // Starting the low-frequency clock (needed for RTC to work)
//!         p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
//!         while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
//!         p.CLOCK.events_lfclkstarted.reset();
//!
//!         let mut rtc0 = LoResTimer::new(p.RTC0);
//!         // 16Hz; 62.5ms period
//!         rtc0.set_frequency(FREQ_16HZ);
//!         rtc0.enable_tick_event();
//!         rtc0.enable_tick_interrupt();
//!         rtc0.start();
//!
//!         let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
//!         display::initialise_display(&mut timer, &mut p.GPIO);
//!
//!         init::LateResources {
//!             GPIO: p.GPIO,
//!             DISPLAY_TIMER: timer,
//!             ANIM_TIMER: rtc0,
//!             DISPLAY: Display::new(),
//!         }
//!     }
//!
//!     #[interrupt(priority = 2,
//!                 resources = [DISPLAY_TIMER, GPIO, DISPLAY])]
//!     fn TIMER1() {
//!         display::handle_display_event(
//!             &mut resources.DISPLAY,
//!             resources.DISPLAY_TIMER,
//!             resources.GPIO,
//!         );
//!     }
//!
//!     #[interrupt(priority = 1,
//!                 resources = [ANIM_TIMER, DISPLAY])]
//!     fn RTC0() {
//!         static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
//!         static mut STEP: u8 = 0;
//!
//!         &resources.ANIM_TIMER.clear_tick_event();
//!
//!         let inner_brightness = match *STEP {
//!             0..=8 => 9 - *STEP,
//!             9..=12 => 0,
//!             _ => unreachable!(),
//!         };
//!
//!         FRAME.set(&mut heart_image(inner_brightness));
//!         resources.DISPLAY.lock(|display| {
//!             display.set_frame(FRAME);
//!         });
//!
//!         *STEP += 1;
//!         if *STEP == 13 {
//!             *STEP = 0
//!         };
//!     }
//! };
//! ```
