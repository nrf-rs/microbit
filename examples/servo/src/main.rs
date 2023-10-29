#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use microbit::{
    board::Board,
    hal::{
        gpio::Level,
        gpiote::*,
        pac::{self, interrupt, TIMER0},
        ppi::{self, ConfigurablePpi, Ppi},
    },
};

static SERVO_TIMER: Mutex<RefCell<Option<TIMER0>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let gpiote = Gpiote::new(board.GPIOTE);
        // Servo output pins
        let servopin1 = board.edge.e01.into_push_pull_output(Level::Low).degrade(); // PAD2
        let servopin2 = board.edge.e02.into_push_pull_output(Level::Low).degrade(); // PAD3

        // Output channel for Servo 1
        gpiote
            .channel0()
            .output_pin(servopin1)
            .task_out_polarity(TaskOutPolarity::Toggle)
            .init_low();
        gpiote.channel0().task_out().write(|w| unsafe { w.bits(1) });
        // Output channel for Servo 2
        gpiote
            .channel1()
            .output_pin(servopin2)
            .task_out_polarity(TaskOutPolarity::Toggle)
            .init_low();
        gpiote.channel1().task_out().write(|w| unsafe { w.bits(1) });

        let ppi_channels = ppi::Parts::new(board.PPI);
        // Set both servo outputs high form Timer0 CC[0]
        // Set each servo output low from the respective Timer0 CC[1] and CC[2]
        // Each timer can run 3 Servos
        let mut ppi0 = ppi_channels.ppi0;
        ppi0.set_task_endpoint(gpiote.channel0().task_out());
        ppi0.set_event_endpoint(&board.TIMER0.events_compare[0]);
        ppi0.enable();
        let mut ppi1 = ppi_channels.ppi1;
        ppi1.set_task_endpoint(gpiote.channel0().task_out());
        ppi1.set_event_endpoint(&board.TIMER0.events_compare[1]);
        ppi1.enable();
        let mut ppi2 = ppi_channels.ppi2;
        ppi2.set_task_endpoint(gpiote.channel1().task_out());
        ppi2.set_event_endpoint(&board.TIMER0.events_compare[0]);
        ppi2.enable();
        let mut ppi3 = ppi_channels.ppi3;
        ppi3.set_task_endpoint(gpiote.channel1().task_out());
        ppi3.set_event_endpoint(&board.TIMER0.events_compare[2]);
        ppi3.enable();

        // The Timer PAC is used directly as the HAL does not give full access to all registers
        board.TIMER0.mode.write(|w| unsafe { w.bits(0) });
        board.TIMER0.bitmode.write(|w| unsafe { w.bits(0) });
        // CC[0] every 20 ms (50 Hz)
        board.TIMER0.cc[0].write(|w| unsafe { w.bits(20000) });
        board.TIMER0.shorts.write(|w| unsafe { w.bits(1) });
        // Servo duty cycle is from 0.5 ms to 2.5 ms with 1.5 ms for center position
        board.TIMER0.cc[1].write(|w| unsafe { w.bits(1500) });
        board.TIMER0.cc[2].write(|w| unsafe { w.bits(1500) });
        board.TIMER0.tasks_start.write(|w| unsafe { w.bits(1) });
        // Timer0 interrupt on CC[0]
        board.TIMER0.intenset.write(|w| unsafe { w.bits(1 << 16) });

        cortex_m::interrupt::free(move |cs| {
            *SERVO_TIMER.borrow(cs).borrow_mut() = Some(board.TIMER0);
        });
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER0);
        }

        loop {}
    }
    panic!("End");
}

#[interrupt]
fn TIMER0() {
    // Change Servo position at the start of the duty cycle. Then there is no race condition
    // between changing the duty cycle and a CC event.
    static mut SPEED: i32 = 1500;
    static mut DIRECTION: i32 = 1;
    match SPEED {
        i32::MIN..=500 => *DIRECTION = 1,
        2500..=i32::MAX => *DIRECTION = -1,
        _ => {}
    }
    *SPEED += *DIRECTION;
    cortex_m::interrupt::free(|cs| {
        //    if let Some(cc_value) = CC_VALUE.borrow(cs).borrow().as_ref() {
        if let Some(timer) = SERVO_TIMER.borrow(cs).borrow_mut().as_mut() {
            //timer.cc[1].write(|w|unsafe { w.bits(u32::try_from(*cc_value).unwrap_or(1500)) });
            let set_speed = u32::try_from(*SPEED).unwrap_or(1500);
            timer.cc[1].write(|w| unsafe { w.bits(set_speed) });
            timer.cc[2].write(|w| unsafe { w.bits(set_speed) });
            timer.events_compare[0].write(|w| unsafe { w.bits(0) });
        }
        //}
    });
}
