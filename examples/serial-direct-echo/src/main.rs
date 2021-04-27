#![no_main]
#![no_std]

use panic_halt as _;

use core::str;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Configure RX and TX pins accordingly */
        p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
        p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

        let uart0 = p.UART0;
        /* Tell UART which pins to use for sending and receiving */
        uart0.pseltxd.write(|w| unsafe { w.bits(24) });
        uart0.pselrxd.write(|w| unsafe { w.bits(25) });

        /* Set a typical baud rate of 115200 */
        uart0.baudrate.write(|w| w.baudrate().baud115200());

        /* Enable UART function */
        uart0.enable.write(|w| w.enable().enabled());

        /* Print a nice hello message */
        let _ = write_uart0(&uart0, "Please type characters to echo:\r\n");

        /* Fire up receiving task */
        uart0.tasks_startrx.write(|w| unsafe { w.bits(1) });

        /* Endless loop */
        loop {
            /* Busy wait for reception of data */
            while uart0.events_rxdrdy.read().bits() == 0 {}

            /* We're going to pick up the data soon, let's signal the buffer is already waiting for
             * more data */
            uart0.events_rxdrdy.write(|w| unsafe { w.bits(0) });

            /* Read one 8bit value */
            let c = uart0.rxd.read().bits() as u8;

            /* What comes in must go out, we don't care what it is */
            let _ = write_uart0(&uart0, unsafe { str::from_utf8_unchecked(&[c; 1]) });
        }
    }

    loop {
        continue;
    }
}

fn write_uart0(uart0: &microbit::pac::UART0, s: &str) -> core::fmt::Result {
    /* Start UART sender */
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });

    for c in s.as_bytes() {
        /* Write the current character to the output register */
        uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

        /* Wait until the UART is clear to send */
        while uart0.events_txdrdy.read().bits() == 0 {}

        /* And then reset it for the next round */
        uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
    }

    /* Stop UART sender */
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}
