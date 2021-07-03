#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use cortex_m_rt::entry;
use microbit::{
    hal::{clocks::Clocks, gpio, prelude::OutputPin, pwm, time::Hertz},
    pac::{self, interrupt},
};

static RTC: Mutex<RefCell<Option<pac::RTC0>>> = Mutex::new(RefCell::new(None));
static SPEAKER: Mutex<RefCell<Option<pwm::Pwm<pac::PWM0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            let _clocks = Clocks::new(p.CLOCK)
                .enable_ext_hfosc()
                .set_lfclk_src_synth()
                .start_lfclk();

            defmt::info!("Speaker");

            p.RTC0.prescaler.write(|w| unsafe { w.bits(511) });
            p.RTC0.evtenset.write(|w| w.tick().set_bit());
            p.RTC0.intenset.write(|w| w.tick().set_bit());
            p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);

            let p0parts = gpio::p0::Parts::new(p.P0);

            let mut speaker_pin = p0parts.p0_00.into_push_pull_output(gpio::Level::Low);
            let _ = speaker_pin.set_low();

            let speaker = pwm::Pwm::new(p.PWM0);
            speaker
                .set_output_pin(pwm::Channel::C0, &speaker_pin.degrade())
                .set_prescaler(pwm::Prescaler::Div16)
                .set_period(Hertz(1000u32))
                .set_counter_mode(pwm::CounterMode::UpAndDown)
                .enable();
            speaker
                .set_seq_refresh(pwm::Seq::Seq0, 0)
                .set_seq_refresh(pwm::Seq::Seq1, 0)
                .set_seq_end_delay(pwm::Seq::Seq0, 0)
                .set_seq_end_delay(pwm::Seq::Seq1, 0);

            let max_duty = speaker.max_duty();
            speaker.set_duty_on_common(max_duty / 2);

            *SPEAKER.borrow(cs).borrow_mut() = Some(speaker);

            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC0);
            }
            pac::NVIC::unpend(pac::Interrupt::RTC0);
        });
    }

    loop {
        continue;
    }
}

// Define an exception, i.e. function to call when exception occurs. Here if our timer
// trips, we'll print some random number
#[interrupt]
fn RTC0() {
    static mut FREQUENCY: u32 = 1;
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(speaker) = SPEAKER.borrow(cs).borrow().as_ref() {
            speaker.set_period(Hertz(*FREQUENCY));
            defmt::info!("Speaker {}", *FREQUENCY);
            let max_duty = speaker.max_duty();
            speaker.set_duty_on_common(max_duty / 2);
        }
        if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
    *FREQUENCY += 1;
    if *FREQUENCY >= 1000 {
        *FREQUENCY = 1
    };
}
