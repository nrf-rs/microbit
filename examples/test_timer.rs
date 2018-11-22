#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_halt;

use core::time::Duration;
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::timer_counter::{Timer, TimerCounter, Generic, Hfticks, Lfticks};
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
TIMER1 is " "
TIMER0 is 4000ms
RTC1 is 7.5s
RTC0 is 8.0s

@500
LED is On
TIMER2 is " "
TIMER2 -> LED
TIMER2 -> TIMER1@+100: clear
TIMER1 is 800ms

@+800
LED is Off
TIMER1 is " "
TIMER1 -> LED

@2000

@2097
TIMER2 is 500ms

@+500
LED is On
TIMER2 is " "
TIMER2 -> LED
TIMER1 is 800ms

@+800
LED is Off
TIMER1 is " "
TIMER1 -> LED

@4000
TIMER2 is 500ms
TIMER1 is 800ms
TIMER0 is " "
TIMER0 -> TIMER2 : clear
TIMER0 -> TIMER1 : clear

@4500
LED is On
TIMER2 is 500ms
TIMER2 -> LED

@4800
TIMER1 is "stop"

@5000
LED is Off
TIMER2 is 500ms
TIMER2 -> LED

@5500
LED is On
TIMER2 is 500ms

@+20
LED is Off
TIMER1 -> LED : compare
LED -> TIMER1

@6000
LED is On
TIMER2 is 500ms

@6320
TIMER1 -> LED
LED -> TIMER1 : compare \n not set

@6500
LED is Off
TIMER2 is 500ms

@7500
LED is On
RTC1 is " "
RTC1 -> LED

@8000
LED is Off
RTC0 is " "
RTC0 -> LED

TIMER2@0 <-> @+2097 : Overflow
TIMER1@500 <-> @+2097 : Overflow

LED@4999 <-> @+500
LED@5500 <-> @+500: 500ms
LED@6000 <-> @+500
TIMER1@5520 <-> @+800: 800ms

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
        let mut timer0 = Timer::<Generic, TIMER0>::new(p.TIMER0, 4);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut timer1 = Timer::<Generic, TIMER1>::new(p.TIMER1, 9);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut timer2 = Timer::<Generic, TIMER2>::new(p.TIMER2, 9);

        // 24bits @ 32.768kHz = 512 seconds
        let mut rtc0 = Timer::<Generic, RTC0>::new(p.RTC0, 0);
        // 24bits @ 32.768kHz = 512 seconds
        let mut rtc1 = Timer::<Generic, RTC1>::new(p.RTC1, 0);

        let compare0 = Hfticks::from(Duration::from_millis(4_000))
            .checked_mul(timer0.frequency())
            .expect("ticks to compare value overflow");
        timer0.set_compare_start(0, compare0).unwrap();
        
        let compare1 = Hfticks::from(Duration::from_millis(800))
            .checked_mul(timer1.frequency())
            .expect("ticks to compare value overflow");
        timer1.set_compare_start(0, compare1).unwrap();
        
        let compare2 = Hfticks::from(Duration::from_millis(500))
            .checked_mul(timer2.frequency())
            .expect("ticks to compare value overflow");
        timer2.set_compare_start(0, compare2).unwrap();
        
        let compare0 = Lfticks::from(Duration::from_millis(8_000))
            .checked_mul(rtc0.frequency())
            .expect("ticks to compare value overflow");
        rtc0.set_compare_start(0, compare0).unwrap();
        
        let compare0 = Lfticks::from(Duration::from_millis(7_500))
            .checked_mul(rtc1.frequency())
            .expect("ticks to compare value overflow");
        rtc1.set_compare_start(0, compare0).unwrap();

        // @500
        block!(timer2.nb_wait(0));
        pin.set_high();
        timer1.task_clear();

        // @800
        block!(timer1.nb_wait(0));
        pin.set_low();

        // @2097
        // Counter rolls over

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_high();

        // @+800
        block!(timer1.nb_wait(0));
        pin.set_low();

        // @4000
        block!(timer0.nb_wait(0));
        // Setup to test stop and clear on interrupt
        timer1.set_compare_int_stop(0);
        timer2.set_compare_int_clear(0);
        timer1.task_clear();
        timer2.task_clear();

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_high();

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_low();
        // timer1 interrupt and stops

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_high();
        
        // @+Δ
        // timer1 is waiting with the compare event set
        // this clears the event, but not the counter
        block!(timer1.nb_wait(0));
        pin.set_low();

        // @+Δ
        // timer1 is stopped, compare event does not fire again
        assert!(timer1.nb_wait(0) == Err(nb::Error::WouldBlock));

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_high();

        // @+500
        block!(timer2.nb_wait(0));
        pin.set_low();

        // @8000
        block!(rtc0.nb_wait(0));
        pin.set_high();

        // @8500
        block!(rtc1.nb_wait(0));
        pin.set_low();

    }
    
    panic!("FIN");
}
