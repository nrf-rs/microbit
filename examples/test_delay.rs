#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_halt;

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::delay::{Timer, Generic};
use microbit::nrf51::{TIMER0, TIMER1, TIMER2, RTC0, RTC1};

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {

        // Start the LFCLK
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        let mut gpio = p.GPIO.split();

        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();

        // 32bits @ 1MHz = ~72 minutes
        let mut delay_timer0 = Timer::<Generic, TIMER0>::new(p.TIMER0, 4).into_delay();
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay_timer1 = Timer::<Generic, TIMER1>::new(p.TIMER1, 9).into_delay();
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay_timer2 = Timer::<Generic, TIMER2>::new(p.TIMER2, 9).into_delay();

        // 24bits @ 32.768kHz = 512 seconds
        let mut delay_rtc3 = Timer::<Generic,  RTC0>::new(p.RTC0, 0).into_delay();
        // 24bits @ 32.768kHz = 512 seconds
        let mut delay_rtc4 = Timer::<Generic, RTC1>::new(p.RTC1, 0).into_delay();

        const LONG: u16 = 800;
        const SHORT: u16 = 400;

        for _ in 0..2 {    
            
            for _ in 0..2 {
                pin.set_high();
                delay_timer0.delay_ms(LONG);
                pin.set_low();
                delay_timer0.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay_timer1.delay_ms(LONG);
                pin.set_low();
                delay_timer1.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay_timer2.delay_ms(LONG);
                pin.set_low();
                delay_timer2.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay_rtc3.delay_ms(LONG);
                pin.set_low();
                delay_rtc3.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay_rtc4.delay_ms(LONG);
                pin.set_low();
                delay_rtc4.delay_ms(SHORT);
            }
        }
    }
    
    panic!("FIN");
}
