use super::gpio::{DisplayPins, BTN_A, BTN_B, INT_SCL, INT_SDA, SCL, SDA, UART_RX, UART_TX};
use crate::{
    hal::{
        gpio::{p0, p1, Disconnected, Level},
        twim, twis, uarte,
    },
    pac,
};

/// Provides access to the micrbobit
#[allow(non_snake_case)]
pub struct Board {
    /// GPIO pins that are not otherwise used
    pub pins: Pins,

    /// display pins
    pub display_pins: DisplayPins,

    /// buttons
    pub buttons: Buttons,

    /// speaker
    pub speaker_pin: p0::P0_00<Disconnected>,

    /// I2C internal bus pins
    pub i2c_internal: I2CInternalPins,

    /// I2C external bus pins
    pub i2c_external: I2CExternalPins,

    /// UART to debugger pins
    pub uart: UartPins,

    /// Core peripheral: Cache and branch predictor maintenance operations
    pub CBP: pac::CBP,

    /// Core peripheral: CPUID
    pub CPUID: pac::CPUID,

    /// Core peripheral: Debug Control Block
    pub DCB: pac::DCB,

    /// Core peripheral: Data Watchpoint and Trace unit
    pub DWT: pac::DWT,

    /// Core peripheral: Flash Patch and Breakpoint unit
    pub FPB: pac::FPB,

    /// Core peripheral: Floating Point Unit
    pub FPU: pac::FPU,

    /// Core peripheral: Instrumentation Trace Macrocell
    pub ITM: pac::ITM,

    /// Core peripheral: Memory Protection Unit
    pub MPU: pac::MPU,

    /// Core peripheral: Nested Vector Interrupt Controller
    pub NVIC: pac::NVIC,

    /// Core peripheral: System Control Block
    pub SCB: pac::SCB,

    /// Core peripheral: SysTick Timer
    pub SYST: pac::SYST,

    /// Core peripheral: Trace Port Interface Unit
    pub TPIU: pac::TPIU,

    /// nRF52 peripheral: CLOCK
    pub CLOCK: pac::CLOCK,

    /// nRF52 peripheral: FICR
    pub FICR: pac::FICR,

    /// nRF52 peripheral: GPIOTE
    pub GPIOTE: pac::GPIOTE,

    /// nRF52 peripheral: PWM0
    pub PWM0: pac::PWM0,

    /// nRF52 peripheral: PWM1
    pub PWM1: pac::PWM1,

    /// nRF52 peripheral: PWM2
    pub PWM2: pac::PWM2,

    /// nRF52 peripheral: PWM3
    pub PWM3: pac::PWM3,

    /// nRF52 peripheral: RADIO
    pub RADIO: pac::RADIO,

    /// nRF52 peripheral: RNG
    pub RNG: pac::RNG,

    /// nRF52 peripheral: RTC0
    pub RTC0: pac::RTC0,

    /// nRF52 peripheral: TEMP <br>
    /// Can be used with [`Temp::new()`](`crate::hal::temp::Temp::new()`)
    pub TEMP: pac::TEMP,

    /// nRF52 peripheral: TIMER0
    pub TIMER0: pac::TIMER0,

    /// nRF52 peripheral: TIMER1
    pub TIMER1: pac::TIMER1,

    /// nRF52 peripheral: TIMER2
    pub TIMER2: pac::TIMER2,

    /// nRF52 peripheral: TIMER3
    pub TIMER3: pac::TIMER3,

    /// nRF52 peripheral: TIMER4
    pub TIMER4: pac::TIMER4,

    /// nRF52 peripheral: TWIM0
    pub TWIM0: pac::TWIM0,

    /// nRF52 peripheral: TWIS0
    pub TWIS0: pac::TWIS0,

    /// nRF52 peripheral: UARTE0
    pub UARTE0: pac::UARTE0,

    /// nRF52 peripheral: UARTE1
    pub UARTE1: pac::UARTE1,
}

impl Board {
    /// Take the peripherals safely
    ///
    /// This method will return an instance of the board the first time it is
    /// called. It will return only `None` on subsequent calls.
    pub fn take() -> Option<Self> {
        Some(Self::new(
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        ))
    }

    fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        let p0parts = p0::Parts::new(p.P0);
        let p1parts = p1::Parts::new(p.P1);
        Self {
            pins: Pins {
                p0_01: p0parts.p0_01,
                p0_02: p0parts.p0_02,
                p0_03: p0parts.p0_03,
                p0_04: p0parts.p0_04,
                p0_05: p0parts.p0_05,
                p0_07: p0parts.p0_07,
                p0_09: p0parts.p0_09,
                p0_10: p0parts.p0_10,
                p0_12: p0parts.p0_12,
                p0_13: p0parts.p0_13,
                p0_17: p0parts.p0_17,
                p0_18: p0parts.p0_18,
                p0_20: p0parts.p0_20,
                p0_25: p0parts.p0_25,
                p0_27: p0parts.p0_27,
                p0_29: p0parts.p0_29,
                p1_01: p1parts.p1_01,
                p1_02: p1parts.p1_02,
                p1_03: p1parts.p1_03,
                p1_04: p1parts.p1_04,
                p1_06: p1parts.p1_06,
                p1_07: p1parts.p1_07,
                p1_09: p1parts.p1_09,
            },
            display_pins: DisplayPins {
                col1: p0parts.p0_28.into_push_pull_output(Level::High),
                col2: p0parts.p0_11.into_push_pull_output(Level::High),
                col3: p0parts.p0_31.into_push_pull_output(Level::High),
                col4: p1parts.p1_05.into_push_pull_output(Level::High),
                col5: p0parts.p0_30.into_push_pull_output(Level::High),
                row1: p0parts.p0_21.into_push_pull_output(Level::Low),
                row2: p0parts.p0_22.into_push_pull_output(Level::Low),
                row3: p0parts.p0_15.into_push_pull_output(Level::Low),
                row4: p0parts.p0_24.into_push_pull_output(Level::Low),
                row5: p0parts.p0_19.into_push_pull_output(Level::Low),
            },
            buttons: Buttons {
                button_a: p0parts.p0_14.into_floating_input(),
                button_b: p0parts.p0_23.into_floating_input(),
            },
            speaker_pin: p0parts.p0_00,
            i2c_internal: I2CInternalPins {
                scl: p0parts.p0_08.into_floating_input(),
                sda: p0parts.p0_16.into_floating_input(),
            },
            i2c_external: I2CExternalPins {
                scl: p0parts.p0_26.into_floating_input(),
                sda: p1parts.p1_00.into_floating_input(),
            },
            uart: UartPins {
                tx: p0parts.p0_06.into_push_pull_output(Level::High),
                rx: p1parts.p1_08.into_floating_input(),
            },

            // Core peripherals
            CBP: cp.CBP,
            CPUID: cp.CPUID,
            DCB: cp.DCB,
            DWT: cp.DWT,
            FPB: cp.FPB,
            FPU: cp.FPU,
            ITM: cp.ITM,
            MPU: cp.MPU,
            NVIC: cp.NVIC,
            SCB: cp.SCB,
            SYST: cp.SYST,
            TPIU: cp.TPIU,

            // nRF52 peripherals
            CLOCK: p.CLOCK,
            FICR: p.FICR,
            GPIOTE: p.GPIOTE,
            PWM0: p.PWM0,
            PWM1: p.PWM1,
            PWM2: p.PWM2,
            PWM3: p.PWM3,
            RADIO: p.RADIO,
            RNG: p.RNG,
            RTC0: p.RTC0,
            TEMP: p.TEMP,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            TIMER3: p.TIMER3,
            TIMER4: p.TIMER4,
            TWIM0: p.TWIM0,
            TWIS0: p.TWIS0,
            UARTE0: p.UARTE0,
            UARTE1: p.UARTE1,
        }
    }
}

/// Unused GPIO pins
#[allow(missing_docs)]
pub struct Pins {
    // pub p0_00: p0::P0_00<Disconnected>, // Speaker
    pub p0_01: p0::P0_01<Disconnected>,
    pub p0_02: p0::P0_02<Disconnected>,
    pub p0_03: p0::P0_03<Disconnected>,
    pub p0_04: p0::P0_04<Disconnected>,
    pub p0_05: p0::P0_05<Disconnected>,
    // pub p0_06: p0::P0_06<Disconnected>, // UART RX
    pub p0_07: p0::P0_07<Disconnected>,
    // pub p0_08: p0::P0_08<Disconnected>, // INT_SCL
    pub p0_09: p0::P0_09<Disconnected>,
    pub p0_10: p0::P0_10<Disconnected>,
    // pub p0_11: p0::P0_11<Disconnected>, // LEDs
    pub p0_12: p0::P0_12<Disconnected>,
    pub p0_13: p0::P0_13<Disconnected>,
    // pub p0_14: p0::P0_14<Disconnected>, // BTN_A
    // pub p0_15: p0::P0_15<Disconnected>, // LEDs
    // pub p0_16: p0::P0_16<Disconnected>, // INT_SDA
    pub p0_17: p0::P0_17<Disconnected>,
    pub p0_18: p0::P0_18<Disconnected>,
    // pub p0_19: p0::P0_19<Disconnected>, // LEDs
    pub p0_20: p0::P0_20<Disconnected>,
    // pub p0_21: p0::P0_21<Disconnected>, // LEDs
    // pub p0_22: p0::P0_22<Disconnected>, // LEDs
    // pub p0_23: p0::P0_23<Disconnected>, // BTN_B
    // pub p0_24: p0::P0_24<Disconnected>, // LEDs
    pub p0_25: p0::P0_25<Disconnected>,
    // pub p0_26: p0::P0_26<Disconnected>, // SCL
    pub p0_27: p0::P0_27<Disconnected>,
    // pub p0_28: p0::P0_28<Disconnected>, // LEDs
    pub p0_29: p0::P0_29<Disconnected>,
    // pub p0_30: p0::P0_30<Disconnected>, // LEDs
    // pub p0_31: p0::P0_31<Disconnected>, // LEDs
    // pub p1_00: p1::P1_00<Disconnected>, // SDA
    pub p1_01: p1::P1_01<Disconnected>,
    pub p1_02: p1::P1_02<Disconnected>,
    pub p1_03: p1::P1_03<Disconnected>,
    pub p1_04: p1::P1_04<Disconnected>,
    // pub p1_05: p1::P1_05<Disconnected>, // LEDs
    pub p1_06: p1::P1_06<Disconnected>,
    pub p1_07: p1::P1_07<Disconnected>,
    // pub p1_08: p1::P1_08<Disconnected>, // UART TX
    pub p1_09: p1::P1_09<Disconnected>,
}

/// Buttons
pub struct Buttons {
    /// Left hand button
    pub button_a: BTN_A,
    /// Right hand button
    pub button_b: BTN_B,
}

/// I2C internal bus pins
pub struct I2CInternalPins {
    scl: INT_SCL,
    sda: INT_SDA,
}

impl Into<twim::Pins> for I2CInternalPins {
    fn into(self) -> twim::Pins {
        twim::Pins {
            scl: self.scl.degrade(),
            sda: self.sda.degrade(),
        }
    }
}

impl Into<twis::Pins> for I2CInternalPins {
    fn into(self) -> twis::Pins {
        twis::Pins {
            scl: self.scl.degrade(),
            sda: self.sda.degrade(),
        }
    }
}

/// I2C external bus pins
pub struct I2CExternalPins {
    scl: SCL,
    sda: SDA,
}

impl Into<twim::Pins> for I2CExternalPins {
    fn into(self) -> twim::Pins {
        twim::Pins {
            scl: self.scl.degrade(),
            sda: self.sda.degrade(),
        }
    }
}

impl Into<twis::Pins> for I2CExternalPins {
    fn into(self) -> twis::Pins {
        twis::Pins {
            scl: self.scl.degrade(),
            sda: self.sda.degrade(),
        }
    }
}

/// UART to debugger pins
pub struct UartPins {
    tx: UART_TX,
    rx: UART_RX,
}

impl Into<uarte::Pins> for UartPins {
    fn into(self) -> uarte::Pins {
        uarte::Pins {
            txd: self.tx.degrade(),
            rxd: self.rx.degrade(),
            cts: None,
            rts: None,
        }
    }
}
