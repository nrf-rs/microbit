//! Implementation of [`DisplayTimer`] for the nrf51 `TIMER`s.
//!
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer

use tiny_led_matrix::DisplayTimer;

/// One of the three nrf51 TIMER peripherals.
pub trait Nrf51Timer<'a> {
    type Wrapper: DisplayTimer;

    /// Returns a wrapper for the peripheral, implementing DisplayTimer.
    fn as_display_timer(&'a mut self) -> Self::Wrapper;
}

// Implements Nrf51Timer for TIMER0, TIMER1, or TIMER2.
//
// The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks).
//
// Uses CC0 for the primary cycle and CC1 for the secondary alarm. Uses the
// CC0_CLEAR shortcut to implement the primary cycle.
//
// The initialise_cycle() implementation assumes the timer is in the state it
// would have after system reset.
//
// check_primary() and check_secondary() take care of clearing the timer's
// event registers.
macro_rules! nrf51_timer {
    ( $timer:ident, $stimer:expr ) => {
        #[allow(non_snake_case)]
        pub mod $timer {
            use crate::hal::nrf51;
            use tiny_led_matrix::DisplayTimer;
            use crate::display::microbit_timer::Nrf51Timer;
            #[doc = "Wrapper for `"]
            #[doc = $stimer]
            #[doc = "` for passing to the display code."]
            ///
            /// This implements the [`DisplayTimer`] trait.
            ///
            /// [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
            pub struct MicrobitTimer <'a> (pub &'a mut nrf51::$timer);

            // Checks whether the event for a CC register has been generated,
            // then clears the event register.
            fn check_cc(timer: &mut nrf51::$timer, index: usize) -> bool {
                let event_reg = &timer.events_compare[index];
                let fired = event_reg.read().bits() != 0;
                if fired {event_reg.write(|w| unsafe {w.bits(0)} )}
                return fired;
            }

            impl DisplayTimer for MicrobitTimer <'_> {

                fn initialise_cycle(&mut self, ticks: u16) {
                    let timer = &self.0;
                    timer.prescaler.write(|w| unsafe { w.bits(8) });
                    timer.cc[0].write(|w| unsafe { w.bits(ticks as u32) });
                    timer.bitmode.write(|w| w.bitmode()._32bit());
                    timer.shorts.write(|w| w.compare0_clear().enabled());
                    timer.intenset.write(|w| w.compare0().set());
                    timer.tasks_start.write(|w| unsafe { w.bits(1) });
                }

                fn enable_secondary(&mut self) {
                    self.0.intenset.write(|w| w.compare1().set());
                }

                fn disable_secondary(&mut self) {
                    self.0.intenclr.write(|w| w.compare1().clear());
                }

                fn program_secondary(&mut self, ticks: u16) {
                    self.0.cc[1].write(|w| unsafe { w.bits(ticks as u32) });
                }

                fn check_primary(&mut self) -> bool {
                    return check_cc(&mut self.0, 0);
                }

                fn check_secondary(&mut self) -> bool {
                    return check_cc(&mut self.0, 1);
                }

            }

            impl<'a> Nrf51Timer<'a> for nrf51::$timer {
                type Wrapper = MicrobitTimer<'a>;

                fn as_display_timer(&mut self) -> MicrobitTimer {
                    MicrobitTimer(self)
                }
            }
        }

    };
    ( $timer:ident ) => {
        nrf51_timer!($timer, stringify!($timer));
    }
}


nrf51_timer!(TIMER0);
nrf51_timer!(TIMER1);
nrf51_timer!(TIMER2);
