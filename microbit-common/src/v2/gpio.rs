#![allow(clippy::upper_case_acronyms, missing_docs)]
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::peripherals::{
    P0_00, P0_01, P0_02, P0_03, P0_04, P0_05, P0_06, P0_08, P0_09, P0_10, P0_11, P0_12, P0_13,
    P0_14, P0_15, P0_16, P0_17, P0_19, P0_20, P0_21, P0_22, P0_23, P0_24, P0_26, P0_28, P0_30,
    P0_31, P1_00, P1_02, P1_05, P1_08,
};

/* GPIO pads */
pub type PAD0 = P0_02;
pub type PAD1 = P0_03;
pub type PAD2 = P0_04;

/* LED display */
pub const NUM_COLS: usize = 5;
pub type COL1 = P0_28;
pub type COL2 = P0_11;
pub type COL3 = P0_31;
pub type COL4 = P1_05;
pub type COL5 = P0_30;

pub const NUM_ROWS: usize = 5;
pub type ROW1 = P0_21;
pub type ROW2 = P0_22;
pub type ROW3 = P0_15;
pub type ROW4 = P0_24;
pub type ROW5 = P0_19;

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
    pub mic_in: P0_05,
    pub mic_run: P0_20,
}

type LED = Output<'static>;

impl DisplayPins {
    pub fn degrade(self) -> ([LED; NUM_COLS], [LED; NUM_ROWS]) {
        (
            [
                Output::new(self.col1, Level::High, OutputDrive::Standard),
                Output::new(self.col2, Level::High, OutputDrive::Standard),
                Output::new(self.col3, Level::High, OutputDrive::Standard),
                Output::new(self.col4, Level::High, OutputDrive::Standard),
                Output::new(self.col5, Level::High, OutputDrive::Standard),
            ],
            [
                Output::new(self.row1, Level::Low, OutputDrive::Standard),
                Output::new(self.row2, Level::Low, OutputDrive::Standard),
                Output::new(self.row3, Level::Low, OutputDrive::Standard),
                Output::new(self.row4, Level::Low, OutputDrive::Standard),
                Output::new(self.row5, Level::Low, OutputDrive::Standard),
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
pub type BTN_A = P0_14;
pub type BTN_B = P0_23;

/* spi */
pub type MOSI = P0_13;
pub type MISO = P0_01;
pub type SCK = P0_17;

/* i2c - internal */
pub type INT_SCL = P0_08;
pub type INT_SDA = P0_16;

/* i2c - external */
pub type SCL = P0_26;
pub type SDA = P1_00;

/* uart */
pub type UART_TX = P0_06;
pub type UART_RX = P1_08;

/* speaker */
pub type SPEAKER = P0_00;

/* edge connector */
pub type EDGE03 = COL3;
pub type EDGE00 = PAD0; // <- big pad 1
pub type EDGE04 = COL1;
pub type EDGE05 = BTN_A;
pub type EDGE06 = COL4;
pub type EDGE07 = COL2;
pub type EDGE01 = PAD1; // <- big pad 2
pub type EDGE08 = P0_10;
pub type EDGE09 = P0_09;
pub type EDGE10 = COL5;
pub type EDGE11 = BTN_B;
pub type EDGE12 = P0_12;
pub type EDGE02 = PAD2; // <- big pad 3
pub type EDGE13 = SCK;
pub type EDGE14 = MISO;
pub type EDGE15 = MOSI;
pub type EDGE16 = P1_02;
// EDGE18 -> +V
// EDGE19 -> +V
// EDGE20 -> +V
pub type EDGE19 = SCL;
pub type EDGE20 = SDA;
// EDGE23 -> GND
// EDGE24 -> GND
// EDGE25 -> GND
