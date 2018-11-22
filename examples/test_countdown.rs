#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_halt;

use core::time::Duration;
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::timer::{Timer, Generic};
use microbit::hal::hal::timer::CountDown;
use microbit::nrf51::{TIMER0, TIMER1, TIMER2, RTC0, RTC1};
use microbit::nb::block;

/* 
@startuml

scale 500 as 60 pixels
skinparam monochrome reverse

robust "LED light" as LED
concise "Timer 2" as TIMER2
concise "Timer 1" as TIMER1
concise "Timer 0" as TIMER0
concise "RTC 1" as RTC1
concise "RTC 0" as RTC0

LED is Off

@0
TIMER2 is 500ms
TIMER1 is 1000ms
TIMER0 is 1500ms
RTC1 is 4000ms
RTC0 is 5000ms

@+500
LED is On
TIMER2 is " "
TIMER2 -> LED

@+500
LED is Off
TIMER2 is " "
TIMER1 is 1000ms
TIMER1 -> LED

@+500
LED is On
TIMER2 is " "
TIMER0 is 1500ms
TIMER0 -> LED

@+500
LED is Off
TIMER2 is 500ms
TIMER1 is " "
TIMER1 -> LED

@+500
LED is On
TIMER2 is " "
TIMER2 -> LED

@+500
LED is Off
TIMER2 is " "
TIMER1 is " "
TIMER0 is " "
TIMER0 -> LED

@4000
LED is On
RTC1 is " "
RTC1 -> LED

@5000
LED is Off
RTC0 is " "
RTC0 -> LED

@enduml
*/

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {

        // Start the LFCLK
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        let mut gpio = p.GPIO.split();
        
        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();

        // 32bits @ 1MHz = ~72 minutes
        let mut timer0 = Timer::<Generic, TIMER0>::new(p.TIMER0, 4).into_countdown();
        // 16bits @ 31.25kHz = ~2 seconds
        let mut timer1 = Timer::<Generic, TIMER1>::new(p.TIMER1, 9).into_countdown();
        // 16bits @ 31.25kHz = ~2 seconds
        let mut timer2 = Timer::<Generic, TIMER2>::new(p.TIMER2, 9).into_countdown();

        // 24bits @ 32.768kHz = 512 seconds
        let mut rtc0 = Timer::<Generic, RTC0>::new(p.RTC0, 0).into_countdown();
        // 24bits @ 32.768kHz = 512 seconds
        let mut rtc1 = Timer::<Generic, RTC1>::new(p.RTC1, 0).into_countdown();

        CountDown::start(&mut rtc0, Duration::from_millis(5_000));
        CountDown::start(&mut rtc1, Duration::from_millis(4_000));

        CountDown::start(&mut timer0, Duration::from_millis(1_500));
        CountDown::start(&mut timer1, Duration::from_millis(1_000));
        CountDown::start(&mut timer2, Duration::from_millis(500));

        // @+500
        block!(timer2.wait());
        pin.set_high();

        // @+500
        block!(timer1.wait());
        pin.set_low();

        // @+500
        block!(timer0.wait());
        pin.set_high();

        // @+500
        block!(timer1.wait());
        pin.set_low();
        CountDown::start(&mut timer2, Duration::from_millis(500));

        // @+500
        block!(timer2.wait());
        pin.set_high();

        // @+500
        block!(timer0.wait());
        pin.set_low();

        // @4000
        block!(rtc1.wait());
        pin.set_high();

        // @5000
        block!(rtc0.wait());
        pin.set_low();

    }
    
    panic!("FIN");
}
