use super::gpio::{
    DisplayPins, BTN_A, BTN_B, EDGE00, EDGE01, EDGE02, EDGE08, EDGE12, EDGE16, SCL, SDA, UART_RX,
    UART_TX,
};
use crate::{
    hal::{
        gpio::{p0, Disconnected, Level},
        twi, uart,
    },
    pac,
};

/// Provides access to the microbit
#[allow(non_snake_case)]
pub struct Board {
    /// GPIO pins that are not otherwise used
    pub pins: Pins,

    /// Unused GPIO pins on edge connector
    pub edge: Edge,

    /// display pins
    pub display_pins: DisplayPins,

    /// buttons
    pub buttons: Buttons,

    /// I2C shared internal and external bus pins
    pub i2c: I2CPins,

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

    /// nRF51 peripheral: ADC
    pub ADC: pac::ADC,

    /// nRF51 peripheral: CLOCK
    pub CLOCK: pac::CLOCK,

    /// nRF51 peripheral: FICR
    pub FICR: pac::FICR,

    /// nRF51 peripheral: GPIOTE
    pub GPIOTE: pac::GPIOTE,

    /// nRF51 preipheral: PPI
    pub PPI: pac::PPI,

    /// nRF51 peripheral: RADIO
    pub RADIO: pac::RADIO,

    /// nRF51 peripheral: RNG
    pub RNG: pac::RNG,

    /// nRF51 peripheral: RTC0
    pub RTC0: pac::RTC0,

    /// nRF51 peripheral: TEMP <br>
    /// Can be used with [`Temp::new()`](`crate::hal::temp::Temp::new()`)
    pub TEMP: pac::TEMP,

    /// nRF51 peripheral: TIMER0
    pub TIMER0: pac::TIMER0,

    /// nRF51 peripheral: TIMER1
    pub TIMER1: pac::TIMER1,

    /// nRF51 peripheral: TIMER2
    pub TIMER2: pac::TIMER2,

    /// nRF51 peripheral: TWI0
    pub TWI0: pac::TWI0,

    /// nrf51 peripheral: UART0
    pub UART0: pac::UART0,

    /// nrf51 peripheral: POWER
    pub POWER: pac::POWER,

    /// nrf51 peripheral: SPI0
    pub SPI0: pac::SPI0,

    /// nrf51 peripheral: SPI1
    pub SPI1: pac::SPI1,

    /// nrf51 peripheral: TWI1
    pub TWI1: pac::TWI1,

    /// nrf51 peripheral: SPIS1
    pub SPIS1: pac::SPIS1,

    /// nrf51 peripheral: ECB
    pub ECB: pac::ECB,

    /// nrf51 peripheral: AAR
    pub AAR: pac::AAR,

    /// nrf51 peripheral: CCM
    pub CCM: pac::CCM,

    /// nrf51 peripheral: WDT
    pub WDT: pac::WDT,

    /// nrf51 peripheral: RTC1
    pub RTC1: pac::RTC1,

    /// nrf51 peripheral: QDEC
    pub QDEC: pac::QDEC,

    /// nrf51 peripheral: LPCOMP
    pub LPCOMP: pac::LPCOMP,

    /// nrf51 peripheral: SWI
    pub SWI: pac::SWI,

    /// nrf51 peripheral: NVMC
    pub NVMC: pac::NVMC,

    /// nrf51 peripheral: UICR
    pub UICR: pac::UICR,
}

impl Board {
    /// Take the peripherals safely
    ///
    /// This method will return an instance of the board the first time it is
    /// called. It will return only `None` on subsequent calls.
    /// This function can also return `None` if one of the the peripherals was
    /// already taken.
    pub fn take() -> Option<Self> {
        Some(Self::new(
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        ))
    }

    /// Fallback method in the case peripherals and core peripherals were taken
    /// elsewhere already.
    ///
    /// This method will take the peripherals and core peripherals and
    /// return an instance of the board.
    ///
    /// An exemplary usecase is shown in the rtic display example.
    pub fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        let p0parts = p0::Parts::new(p.GPIO);
        Self {
            pins: Pins {
                //p0_01: p0parts.p0_01,
                //p0_02: p0parts.p0_02,
                //p0_03: p0parts.p0_03,
                //p0_16: p0parts.p0_16,
                //p0_18: p0parts.p0_18,
                p0_19: p0parts.p0_19,
                //p0_20: p0parts.p0_20,
                p0_21: p0parts.p0_21,
                p0_22: p0parts.p0_22,
                p0_23: p0parts.p0_23,
                p0_27: p0parts.p0_27,
                p0_28: p0parts.p0_28,
                p0_29: p0parts.p0_29,
            },
            edge: Edge {
                e00: p0parts.p0_03,
                e01: p0parts.p0_02,
                e02: p0parts.p0_01,
                e08: p0parts.p0_18,
                e12: p0parts.p0_20,
                e16: p0parts.p0_16,
            },
            display_pins: DisplayPins {
                row1: p0parts.p0_13.into_push_pull_output(Level::Low),
                row2: p0parts.p0_14.into_push_pull_output(Level::Low),
                row3: p0parts.p0_15.into_push_pull_output(Level::Low),
                col1: p0parts.p0_04.into_push_pull_output(Level::High),
                col2: p0parts.p0_05.into_push_pull_output(Level::High),
                col3: p0parts.p0_06.into_push_pull_output(Level::High),
                col4: p0parts.p0_07.into_push_pull_output(Level::High),
                col5: p0parts.p0_08.into_push_pull_output(Level::High),
                col6: p0parts.p0_09.into_push_pull_output(Level::High),
                col7: p0parts.p0_10.into_push_pull_output(Level::High),
                col8: p0parts.p0_11.into_push_pull_output(Level::High),
                col9: p0parts.p0_12.into_push_pull_output(Level::High),
            },
            buttons: Buttons {
                button_a: p0parts.p0_17.into_floating_input(),
                button_b: p0parts.p0_26.into_floating_input(),
            },
            i2c: I2CPins {
                scl: p0parts.p0_00.into_floating_input(),
                sda: p0parts.p0_30.into_floating_input(),
            },
            uart: UartPins {
                tx: p0parts.p0_24.into_push_pull_output(Level::Low),
                rx: p0parts.p0_25.into_floating_input(),
            },

            // Core peripherals
            CBP: cp.CBP,
            CPUID: cp.CPUID,
            DCB: cp.DCB,
            DWT: cp.DWT,
            FPB: cp.FPB,
            ITM: cp.ITM,
            MPU: cp.MPU,
            NVIC: cp.NVIC,
            SCB: cp.SCB,
            SYST: cp.SYST,
            TPIU: cp.TPIU,

            // nRF51 peripherals
            ADC: p.ADC,
            CLOCK: p.CLOCK,
            FICR: p.FICR,
            GPIOTE: p.GPIOTE,
            PPI: p.PPI,
            RADIO: p.RADIO,
            RNG: p.RNG,
            RTC0: p.RTC0,
            TEMP: p.TEMP,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            TWI0: p.TWI0,
            UART0: p.UART0,
            POWER: p.POWER,
            SPI0: p.SPI0,
            SPI1: p.SPI1,
            TWI1: p.TWI1,
            SPIS1: p.SPIS1,
            ECB: p.ECB,
            AAR: p.AAR,
            CCM: p.CCM,
            WDT: p.WDT,
            RTC1: p.RTC1,
            QDEC: p.QDEC,
            LPCOMP: p.LPCOMP,
            SWI: p.SWI,
            NVMC: p.NVMC,
            UICR: p.UICR,
        }
    }
}

/// Unused GPIO pins
#[allow(missing_docs)]
pub struct Pins {
    // pub p0_00: p0::P0_00<Disconnected>, // SCL
    // pub p0_01: p0::P0_01<Disconnected>, // PAD2, EDGE02
    // pub p0_02: p0::P0_02<Disconnected>, // PAD1, EDGE01
    // pub p0_03: p0::P0_03<Disconnected>, // PAD0, EDGE00
    // pub p0_04: p0::P0_04<Disconnected>, // LEDs
    // pub p0_05: p0::P0_05<Disconnected>, // LEDs
    // pub p0_06: p0::P0_06<Disconnected>, // LEDs
    // pub p0_07: p0::P0_07<Disconnected>, // LEDs
    // pub p0_08: p0::P0_08<Disconnected>, // LEDs
    // pub p0_09: p0::P0_09<Disconnected>, // LEDs
    // pub p0_10: p0::P0_10<Disconnected>, // LEDs
    // pub p0_11: p0::P0_11<Disconnected>, // LEDs
    // pub p0_12: p0::P0_12<Disconnected>, // LEDs
    // pub p0_13: p0::P0_13<Disconnected>, // LEDs
    // pub p0_14: p0::P0_14<Disconnected>, // LEDs
    // pub p0_15: p0::P0_15<Disconnected>, // LEDs
    // pub p0_16: p0::P0_16<Disconnected>, // EDGE16
    // pub p0_17: p0::P0_17<Disconnected>, // BTN_A
    // pub p0_18: p0::P0_18<Disconnected>, // EDGE08
    pub p0_19: p0::P0_19<Disconnected>,
    // pub p0_20: p0::P0_20<Disconnected>, // EDGE12
    pub p0_21: p0::P0_21<Disconnected>,
    pub p0_22: p0::P0_22<Disconnected>,
    pub p0_23: p0::P0_23<Disconnected>,
    // pub p0_24: p0::P0_24<Disconnected>, // UART TX
    // pub p0_25: p0::P0_25<Disconnected>, // UART RX
    // pub p0_26: p0::P0_26<Disconnected>, // BTN_B
    pub p0_27: p0::P0_27<Disconnected>,
    pub p0_28: p0::P0_28<Disconnected>,
    pub p0_29: p0::P0_29<Disconnected>,
    // pub p0_30: p0::P0_30<Disconnected>, // SDA
}

/// Unused edge connector pins
#[allow(missing_docs)]
pub struct Edge {
    // pub e03: COL1,
    pub e00: EDGE00<Disconnected>, // <- big pad 1
    // pub e04: COL2,
    // pub e05: BTN_A,
    // pub e06: COL9,
    // pub e07: COL8,
    pub e01: EDGE01<Disconnected>, // <- big pad 2
    pub e08: EDGE08<Disconnected>,
    // pub e09: COL7,
    // pub e10: COL3,
    // pub e11: BTN_B,
    pub e12: EDGE12<Disconnected>,
    pub e02: EDGE02<Disconnected>, // <- big pad 3
    //pub e13<MODE>: SCK<MODE>,
    //pub e14<MODE>: MISO<MODE>,
    //pub e15<MODE>: MOSI<MODE>,
    pub e16: EDGE16<Disconnected>,
    // +V
    // +V
    // +V
    // pub e19: SCL,
    // pub e20: SDA,
    // GND
    // GND
    // GND
}

/// Board buttons
pub struct Buttons {
    /// Left hand side button
    pub button_a: BTN_A,
    /// Right hand side button
    pub button_b: BTN_B,
}

/// I2C shared internal and external bus pins
pub struct I2CPins {
    /// I2C control pin
    pub scl: SCL,
    /// I2C data pin
    pub sda: SDA,
}

impl From<I2CPins> for twi::Pins {
    fn from(pins: I2CPins) -> Self {
        Self {
            scl: pins.scl.degrade(),
            sda: pins.sda.degrade(),
        }
    }
}

/// UART to debugger pins
pub struct UartPins {
    tx: UART_TX,
    rx: UART_RX,
}

impl From<UartPins> for uart::Pins {
    fn from(pins: UartPins) -> Self {
        Self {
            rxd: pins.rx.degrade(),
            txd: pins.tx.degrade(),
            cts: None,
            rts: None,
        }
    }
}
