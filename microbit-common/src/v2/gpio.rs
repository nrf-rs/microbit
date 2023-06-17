#![allow(clippy::upper_case_acronyms, missing_docs)]
use nrf52833_hal::gpio::{p0, p1, Floating, Input, OpenDrain, Output, Pin, PushPull};

/* GPIO pads */
pub type PAD0<MODE> = p0::P0_02<MODE>;
pub type PAD1<MODE> = p0::P0_03<MODE>;
pub type PAD2<MODE> = p0::P0_04<MODE>;

/* LED display */
pub const NUM_COLS: usize = 5;
pub type COL1 = p0::P0_28<Output<PushPull>>;
pub type COL2 = p0::P0_11<Output<PushPull>>;
pub type COL3 = p0::P0_31<Output<PushPull>>;
pub type COL4 = p1::P1_05<Output<PushPull>>;
pub type COL5 = p0::P0_30<Output<PushPull>>;

pub const NUM_ROWS: usize = 5;
pub type ROW1 = p0::P0_21<Output<PushPull>>;
pub type ROW2 = p0::P0_22<Output<PushPull>>;
pub type ROW3 = p0::P0_15<Output<PushPull>>;
pub type ROW4 = p0::P0_24<Output<PushPull>>;
pub type ROW5 = p0::P0_19<Output<PushPull>>;

/// GPIO pins connected to the LED matrix
///
/// Use the [display_pins] macro for easier construction.
pub struct DisplayPins {
    pub col1: COL1,
    pub col2: COL2,
    pub col3: COL3,
    pub col4: COL4,
    pub col5: COL5,
    pub row1: ROW1,
    pub row2: ROW2,
    pub row3: ROW3,
    pub row4: ROW4,
    pub row5: ROW5,
}

/// GPIO pins connected to the microphone
pub struct MicrophonePins {
    pub mic_in: p0::P0_05<Input<Floating>>,
    pub mic_run: p0::P0_20<Output<OpenDrain>>,
}

type LED = Pin<Output<PushPull>>;

impl DisplayPins {
    pub fn degrade(self) -> ([LED; NUM_COLS], [LED; NUM_ROWS]) {
        (
            [
                self.col1.degrade(),
                self.col2.degrade(),
                self.col3.degrade(),
                self.col4.degrade(),
                self.col5.degrade(),
            ],
            [
                self.row1.degrade(),
                self.row2.degrade(),
                self.row3.degrade(),
                self.row4.degrade(),
                self.row5.degrade(),
            ],
        )
    }
}

/// Create [DisplayPins] from a [GPIO Parts](crate::hal::gpio::p0::Parts)
///
/// # Example
///
/// ```no_run
/// # use microbit_common as microbit;
/// use microbit::{
///     display_pins,
///     pac,
///     hal::gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts},
/// };
///
/// // take the peripherals
/// let p = pac::Peripherals::take().unwrap();
/// // split off the P0 GPIO port
/// let p0parts = P0Parts::new(p.P0);
/// // split off the P1 GPIO port
/// let p1parts = P1Parts::new(p.P1);
///
/// let pins = display_pins!(p0parts, p1parts);
/// ```
#[macro_export]
macro_rules! display_pins {
    ( $p0parts:expr, $p1parts:expr ) => {{
        use microbit::{gpio::DisplayPins, hal::gpio::Level};

        DisplayPins {
            col1: $p0parts.p0_28.into_push_pull_output(Level::Low),
            col2: $p0parts.p0_11.into_push_pull_output(Level::Low),
            col3: $p0parts.p0_31.into_push_pull_output(Level::Low),
            col4: $p1parts.p1_05.into_push_pull_output(Level::Low),
            col5: $p0parts.p0_30.into_push_pull_output(Level::Low),
            row1: $p0parts.p0_21.into_push_pull_output(Level::Low),
            row2: $p0parts.p0_22.into_push_pull_output(Level::Low),
            row3: $p0parts.p0_15.into_push_pull_output(Level::Low),
            row4: $p0parts.p0_24.into_push_pull_output(Level::Low),
            row5: $p0parts.p0_19.into_push_pull_output(Level::Low),
        }
    }};
}

/* buttons */
pub type BTN_A = p0::P0_14<Input<Floating>>;
pub type BTN_B = p0::P0_23<Input<Floating>>;

/* spi */
pub type MOSI<MODE> = p0::P0_13<MODE>;
pub type MISO<MODE> = p0::P0_01<MODE>;
pub type SCK<MODE> = p0::P0_17<MODE>;

/* i2c - internal */
pub type INT_SCL = p0::P0_08<Input<Floating>>;
pub type INT_SDA = p0::P0_16<Input<Floating>>;

/* i2c - external */
pub type SCL = p0::P0_26<Input<Floating>>;
pub type SDA = p1::P1_00<Input<Floating>>;

/* uart */
pub type UART_TX = p0::P0_06<Output<PushPull>>;
pub type UART_RX = p1::P1_08<Input<Floating>>;

/* speaker */
pub type SPEAKER = p0::P0_00<Output<PushPull>>;

/* edge connector */
pub type EDGE03 = COL3;
pub type EDGE00<MODE> = PAD0<MODE>; // <- big pad 1
pub type EDGE04 = COL1;
pub type EDGE05 = BTN_A;
pub type EDGE06 = COL4;
pub type EDGE07 = COL2;
pub type EDGE01<MODE> = PAD1<MODE>; // <- big pad 2
pub type EDGE08<MODE> = p0::P0_10<MODE>;
pub type EDGE09<MODE> = p0::P0_09<MODE>;
pub type EDGE10 = COL5;
pub type EDGE11 = BTN_B;
pub type EDGE12<MODE> = p0::P0_12<MODE>;
pub type EDGE02<MODE> = PAD2<MODE>; // <- big pad 3
pub type EDGE13<MODE> = SCK<MODE>;
pub type EDGE14<MODE> = MISO<MODE>;
pub type EDGE15<MODE> = MOSI<MODE>;
pub type EDGE16<MODE> = p1::P1_02<MODE>;
// EDGE18 -> +V
// EDGE19 -> +V
// EDGE20 -> +V
pub type EDGE19 = SCL;
pub type EDGE20 = SDA;
// EDGE23 -> GND
// EDGE24 -> GND
// EDGE25 -> GND
