//! Implementation of [`DisplayTimer`] for the nRF51 `TIMER`s.
//!
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer

use tiny_led_matrix::DisplayTimer;

use crate::hal::timer::Instance;

/// A TIMER peripheral programmed to manage the display.
///
/// `MicrobitDisplayTimer` instances implement the [`DisplayTimer`] trait.
///
/// The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks).
/// The primary cycle takes 6ms.
///
/// Uses CC0 for the primary cycle and CC1 for the secondary alarm. Uses the
/// CC0_CLEAR shortcut to implement the primary cycle.
///
/// [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
pub struct MicrobitDisplayTimer<T: Instance>(T);

impl<T: Instance> MicrobitDisplayTimer<T> {
    /// Returns a new `MicrobitDisplayTimer` wrapping the passed TIMER.
    ///
    /// Takes ownership of the TIMER peripheral.
    pub fn new(timer: T) -> MicrobitDisplayTimer<T> {
        MicrobitDisplayTimer(timer)
    }

    /// Gives the underlying `nrf51::TIMER`*n* instance back.
    pub fn free(self) -> T {
        self.0
    }
}

impl<T: Instance> DisplayTimer for MicrobitDisplayTimer<T> {
    fn initialise_cycle(&mut self, ticks: u16) {
        let timer0 = T::regs();
        // stop and reset timer
        timer0.tasks_stop.write(|w| unsafe { w.bits(1) });
        timer0.tasks_clear.write(|w| unsafe { w.bits(1) });

        // set as 16 bits
        timer0.bitmode.write(|w| w.bitmode()._16bit());

        // set frequency to 62500Hz
        timer0.prescaler.write(|w| unsafe { w.bits(8) });

        // set compare register
        timer0.cc[0].write(|w| unsafe { w.bits(ticks.into()) });

        // enable auto clear
        timer0.shorts.write(|w| w.compare0_clear().enabled());

        // enable compare interrupt
        timer0.intenset.write(|w| w.compare0().set());

        // start
        timer0.tasks_start.write(|w| unsafe { w.bits(1) });
        // maybe?
        // timer0.tasks_start.write(|w| w.tasks_start().set_bit());
    }

    fn enable_secondary(&mut self) {
        let timer0 = T::regs();
        timer0.intenset.write(|w| w.compare1().set());
    }

    fn disable_secondary(&mut self) {
        let timer0 = T::regs();
        timer0.intenclr.write(|w| w.compare1().set_bit());
    }

    fn program_secondary(&mut self, ticks: u16) {
        let timer0 = T::regs();
        #[cfg(feature = "v1")]
        timer0.cc[1].write(|w| unsafe { w.bits(ticks.into()) });
        #[cfg(feature = "v2")]
        timer0.cc[1].write(|w| unsafe { w.cc().bits(ticks.into()) });
    }

    fn check_primary(&mut self) -> bool {
        let timer0 = T::regs();
        // poll compare event
        let reg = &timer0.events_compare[0];
        let fired = reg.read().bits() != 0;
        if fired {
            reg.reset();
        }
        fired
    }

    fn check_secondary(&mut self) -> bool {
        let timer0 = T::regs();
        // poll compare event
        let reg = &timer0.events_compare[1];
        let fired = reg.read().bits() != 0;
        if fired {
            reg.reset();
        }
        fired
    }
}
