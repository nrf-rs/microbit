use super::gpio::{
    DisplayPins, MicrophonePins, EDGE00, EDGE01, EDGE02, EDGE08, EDGE09, EDGE12, EDGE16, INT_SCL,
    INT_SDA, SCL, SDA, UART_RX, UART_TX,
};
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Pull},
    peripherals::{
        self, GPIOTE_CH0, GPIOTE_CH1, GPIOTE_CH2, GPIOTE_CH3, GPIOTE_CH4, GPIOTE_CH5, GPIOTE_CH6,
        GPIOTE_CH7, P0_00, P0_01, P0_07, P0_13, P0_17, P0_18, P0_25, P0_27, P0_29, P1_01, P1_03,
        P1_04, P1_06, P1_07, P1_09, PPI_CH0, PPI_CH1, PPI_CH10, PPI_CH11, PPI_CH12, PPI_CH13,
        PPI_CH14, PPI_CH15, PPI_CH16, PPI_CH17, PPI_CH18, PPI_CH19, PPI_CH2, PPI_CH3, PPI_CH4,
        PPI_CH5, PPI_CH6, PPI_CH7, PPI_CH8, PPI_CH9, PWM0, PWM1, PWM2, PWM3, RNG, RTC0, RTC1, RTC2,
        SAADC, TEMP, TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TWISPI0, TWISPI1, UARTE0, UARTE1,
    },
    twim, uarte,
};

/// Provides access to the microbit
#[allow(non_snake_case)]
#[allow(missing_docs)]
pub struct Board {
    /// GPIO pins that are not otherwise used
    pub pins: Pins,

    /// Unused GPIO pins on edge connector
    pub edge: Edge,

    /// display pins
    pub display_pins: DisplayPins,

    /// buttons
    pub buttons: Buttons,

    /// speaker
    pub speaker_pin: P0_00,

    /// microphone pins
    pub microphone_pins: MicrophonePins,

    /// I2C internal bus pins
    pub i2c_internal: I2CInternalPins,

    /// I2C external bus pins
    pub i2c_external: I2CExternalPins,

    /// UART to debugger pins
    pub uart: UartPins,

    /// nRF52 peripheral: GPIOTE
    pub GPIOTE_CH0: GPIOTE_CH0,
    pub GPIOTE_CH1: GPIOTE_CH1,
    pub GPIOTE_CH2: GPIOTE_CH2,
    pub GPIOTE_CH3: GPIOTE_CH3,
    pub GPIOTE_CH4: GPIOTE_CH4,
    pub GPIOTE_CH5: GPIOTE_CH5,
    pub GPIOTE_CH6: GPIOTE_CH6,
    pub GPIOTE_CH7: GPIOTE_CH7,

    /// nRF52 preipheral: PPI
    pub PPI_CH0: PPI_CH0,
    pub PPI_CH1: PPI_CH1,
    pub PPI_CH2: PPI_CH2,
    pub PPI_CH3: PPI_CH3,
    pub PPI_CH4: PPI_CH4,
    pub PPI_CH5: PPI_CH5,
    pub PPI_CH6: PPI_CH6,
    pub PPI_CH7: PPI_CH7,
    pub PPI_CH8: PPI_CH8,
    pub PPI_CH9: PPI_CH9,
    pub PPI_CH10: PPI_CH10,
    pub PPI_CH11: PPI_CH11,
    pub PPI_CH12: PPI_CH12,
    pub PPI_CH13: PPI_CH13,
    pub PPI_CH14: PPI_CH14,
    pub PPI_CH15: PPI_CH15,
    pub PPI_CH16: PPI_CH16,
    pub PPI_CH17: PPI_CH17,
    pub PPI_CH18: PPI_CH18,
    pub PPI_CH19: PPI_CH19,

    /// nRF52 peripheral: PWM0
    pub PWM0: PWM0,

    /// nRF52 peripheral: PWM1
    pub PWM1: PWM1,

    /// nRF52 peripheral: PWM2
    pub PWM2: PWM2,

    /// nRF52 peripheral: PWM3
    pub PWM3: PWM3,

    /// nRF52 peripheral: RNG
    pub RNG: RNG,

    /// nRF52 peripheral: RTC0
    pub RTC0: RTC0,

    /// nRF52 peripheral: RTC1
    pub RTC1: RTC1,

    /// nRF52 peripheral: RTC2
    pub RTC2: RTC2,

    /// nRF52 peripheral: TEMP <br>
    /// Can be used with [`Temp::new()`](`crate::hal::temp::Temp::new()`)
    pub TEMP: TEMP,

    /// nRF52 peripheral: TIMER0
    pub TIMER0: TIMER0,

    /// nRF52 peripheral: TIMER1
    pub TIMER1: TIMER1,

    /// nRF52 peripheral: TIMER2
    pub TIMER2: TIMER2,

    /// nRF52 peripheral: TIMER3
    pub TIMER3: TIMER3,

    /// nRF52 peripheral: TIMER4
    pub TIMER4: TIMER4,

    /// nRF52 peripheral: TWISPI0
    pub TWISPI0: TWISPI0,

    /// nRF52 peripheral: TWISPI1
    pub TWISPI1: TWISPI1,

    /// nRF52 peripheral: UARTE0
    pub UARTE0: UARTE0,

    /// nRF52 peripheral: UARTE1
    pub UARTE1: UARTE1,

    /// nRF52 peripheral: SAADC
    pub ADC: SAADC,
}

impl Default for Board {
    fn default() -> Board {
        Board::new(Default::default())
    }
}

impl Board {
    /// Take the peripherals safely
    ///
    /// This method will return an instance of the board the first time it is
    /// called. It will panic on subsequent calls.
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);
        Self {
            pins: Pins {
                p0_01: p.P0_01,
                //p0_02: p0parts.p0_02,
                //p0_03: p0parts.p0_03,
                //p0_04: p0parts.p0_04,
                p0_07: p.P0_07,
                //p0_09: p0parts.p0_09,
                //p0_10: p0parts.p0_10,
                //p0_12: p0parts.p0_12,
                p0_13: p.P0_13,
                p0_17: p.P0_17,
                p0_18: p.P0_18,
                p0_25: p.P0_25,
                p0_27: p.P0_27,
                p0_29: p.P0_29,
                p1_01: p.P1_01,
                //p1_02: p.P1_02,
                p1_03: p.P1_03,
                p1_04: p.P1_04,
                p1_06: p.P1_06,
                p1_07: p.P1_07,
                p1_09: p.P1_09,
            },
            edge: Edge {
                e00: p.P0_02,
                e01: p.P0_03,
                e02: p.P0_04,
                e08: p.P0_10,
                e09: p.P0_09,
                e12: p.P0_12,
                e16: p.P1_02,
            },
            display_pins: DisplayPins {
                col1: p.P0_28,
                col2: p.P0_11,
                col3: p.P0_31,
                col4: p.P1_05,
                col5: p.P0_30,
                row1: p.P0_21,
                row2: p.P0_22,
                row3: p.P0_15,
                row4: p.P0_24,
                row5: p.P0_19,
            },
            buttons: Buttons {
                button_a: Input::new(p.P0_14, Pull::None),
                button_b: Input::new(p.P0_23, Pull::None),
            },
            speaker_pin: p.P0_00,
            microphone_pins: MicrophonePins {
                mic_in: p.P0_05,
                mic_run: p.P0_20,
            },
            i2c_internal: I2CInternalPins {
                scl: p.P0_08,
                sda: p.P0_16,
            },
            i2c_external: I2CExternalPins {
                scl: p.P0_26,
                sda: p.P1_00,
            },
            uart: UartPins {
                tx: p.P0_06,
                rx: p.P1_08,
            },

            // nRF52 peripherals
            GPIOTE_CH0: p.GPIOTE_CH0,
            GPIOTE_CH1: p.GPIOTE_CH1,
            GPIOTE_CH2: p.GPIOTE_CH2,
            GPIOTE_CH3: p.GPIOTE_CH3,
            GPIOTE_CH4: p.GPIOTE_CH4,
            GPIOTE_CH5: p.GPIOTE_CH5,
            GPIOTE_CH6: p.GPIOTE_CH6,
            GPIOTE_CH7: p.GPIOTE_CH7,
            PPI_CH0: p.PPI_CH0,
            PPI_CH1: p.PPI_CH1,
            PPI_CH2: p.PPI_CH2,
            PPI_CH3: p.PPI_CH3,
            PPI_CH4: p.PPI_CH4,
            PPI_CH5: p.PPI_CH5,
            PPI_CH6: p.PPI_CH6,
            PPI_CH7: p.PPI_CH7,
            PPI_CH8: p.PPI_CH8,
            PPI_CH9: p.PPI_CH9,
            PPI_CH10: p.PPI_CH10,
            PPI_CH11: p.PPI_CH11,
            PPI_CH12: p.PPI_CH12,
            PPI_CH13: p.PPI_CH13,
            PPI_CH14: p.PPI_CH14,
            PPI_CH15: p.PPI_CH15,
            PPI_CH16: p.PPI_CH16,
            PPI_CH17: p.PPI_CH17,
            PPI_CH18: p.PPI_CH18,
            PPI_CH19: p.PPI_CH19,
            PWM0: p.PWM0,
            PWM1: p.PWM1,
            PWM2: p.PWM2,
            PWM3: p.PWM3,
            RNG: p.RNG,
            RTC0: p.RTC0,
            RTC1: p.RTC1,
            RTC2: p.RTC2,
            TEMP: p.TEMP,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            TIMER3: p.TIMER3,
            TIMER4: p.TIMER4,
            TWISPI0: p.TWISPI0,
            TWISPI1: p.TWISPI1,
            UARTE0: p.UARTE0,
            UARTE1: p.UARTE1,
            ADC: p.SAADC,
        }
    }
}

/// Unused GPIO pins
#[allow(missing_docs)]
pub struct Pins {
    // pub p0_00: p0::P0_00<Disconnected>, // Speaker
    pub p0_01: P0_01,
    // pub p0_02: p0::P0_02<Disconnected>, // PAD0, EDGE00
    // pub p0_03: p0::P0_03<Disconnected>, // PAD1, EDGE01
    // pub p0_04: p0::P0_04<Disconnected>, // PAD2, EDGE02
    // pub p0_05: p0::P0_05<Disconnected>, // Microphone IN
    // pub p0_06: p0::P0_06<Disconnected>, // UART RX
    pub p0_07: P0_07,
    // pub p0_08: p0::P0_08<Disconnected>, // INT_SCL
    // pub p0_09: p0::P0_09<Disconnected>, // EDGE09
    // pub p0_10: p0::P0_10<Disconnected>, // EDGE08
    // pub p0_11: p0::P0_11<Disconnected>, // LEDs
    // pub p0_12: p0::P0_12<Disconnected>, // EDGE12
    pub p0_13: P0_13,
    // pub p0_14: p0::P0_14<Disconnected>, // BTN_A
    // pub p0_15: p0::P0_15<Disconnected>, // LEDs
    // pub p0_16: p0::P0_16<Disconnected>, // INT_SDA
    pub p0_17: P0_17,
    pub p0_18: P0_18,
    // pub p0_19: p0::P0_19<Disconnected>, // LEDs
    // pub p0_20: p0::P0_20<Disconnected>, // Microphone RUN
    // pub p0_21: p0::P0_21<Disconnected>, // LEDs
    // pub p0_22: p0::P0_22<Disconnected>, // LEDs
    // pub p0_23: p0::P0_23<Disconnected>, // BTN_B
    // pub p0_24: p0::P0_24<Disconnected>, // LEDs
    pub p0_25: P0_25,
    // pub p0_26: p0::P0_26<Disconnected>, // SCL
    pub p0_27: P0_27,
    // pub p0_28: p0::P0_28<Disconnected>, // LEDs
    pub p0_29: P0_29,
    // pub p0_30: p0::P0_30<Disconnected>, // LEDs
    // pub p0_31: p0::P0_31<Disconnected>, // LEDs
    // pub p1_00: p1::P1_00<Disconnected>, // SDA
    pub p1_01: P1_01,
    // pub p1_02: p1::P1_02<Disconnected>, // EDGE16
    pub p1_03: P1_03,
    pub p1_04: P1_04,
    // pub p1_05: p1::P1_05<Disconnected>, // LEDs
    pub p1_06: P1_06,
    pub p1_07: P1_07,
    // pub p1_08: p1::P1_08<Disconnected>, // UART TX
    pub p1_09: P1_09,
}

/// Unused edge connector pins
#[allow(missing_docs)]
pub struct Edge {
    /* edge connector */
    // pub e03: COL3,
    pub e00: EDGE00, // <- big pad 1
    // pub e04: COL1,
    // pub e05: BTN_A,
    // pub e06: COL4,
    // pub e07: COL2,
    pub e01: EDGE01, // <- big pad 2
    pub e08: EDGE08,
    pub e09: EDGE09,
    // pub e10: COL5,
    // pub e11: BTN_B,
    pub e12: EDGE12,
    pub e02: EDGE02, // <- big pad 3
    //pub e13<MODE>: SCK<MODE>,
    //pub e14<MODE>: MISO<MODE>,
    //pub e15<MODE>: MOSI<MODE>,
    pub e16: EDGE16,
    // +V
    // +V
    // +V
    // pub e19: SCL,
    // pub e20: SDA,
    // GND
    // GND
    // GND
}

/// Buttons
pub struct Buttons {
    /// Left hand button
    pub button_a: Input<'static>,
    /// Right hand button
    pub button_b: Input<'static>,
}

/// I2C internal bus pins
pub struct I2CInternalPins {
    scl: INT_SCL,
    sda: INT_SDA,
}

impl I2CInternalPins {
    /// Create a new uarte instance for the UART pins
    pub fn create(self, p: TWISPI1, config: twim::Config) -> twim::Twim<'static, TWISPI1> {
        bind_interrupts!(struct Irqs {
            SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 => twim::InterruptHandler<TWISPI1>;
        });
        twim::Twim::new(p, Irqs, self.scl, self.sda, config)
    }
}

/// I2C external bus pins
pub struct I2CExternalPins {
    scl: SCL,
    sda: SDA,
}

impl I2CExternalPins {
    /// Create a new uarte instance for the UART pins
    pub fn create(self, p: TWISPI0, config: twim::Config) -> twim::Twim<'static, TWISPI0> {
        bind_interrupts!(struct Irqs {
            SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<TWISPI0>;
        });
        twim::Twim::new(p, Irqs, self.scl, self.sda, config)
    }
}

/// UART to debugger pins
pub struct UartPins {
    tx: UART_TX,
    rx: UART_RX,
}

impl UartPins {
    /// Create a new uarte instance for the UART pins
    pub fn create(self, p: UARTE0, config: uarte::Config) -> uarte::Uarte<'static, UARTE0> {
        bind_interrupts!(struct Irqs {
            UARTE0_UART0 => uarte::InterruptHandler<peripherals::UARTE0>;
        });

        return uarte::Uarte::new(p, Irqs, self.rx, self.tx, config);
    }
}
