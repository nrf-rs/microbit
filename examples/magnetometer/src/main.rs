#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelMode, AccelOutputDataRate, Lsm303agr,
};
use microbit::hal::Timer;

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::{twi0::frequency::FREQUENCY_A, TWI0},
};
#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::{twim0::frequency::FREQUENCY_A, TWIM0},
};

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    if let Ok(id) = sensor.accelerometer_id() {
        if !id.is_correct() {
            defmt::panic!("Accelerometer had unexpected ID {:#x}", id.raw());
        }
    } else {
        defmt::panic!("Error getting accelerometer ID");
    }
    sensor.init().unwrap();

    defmt::info!("normal mode");
    sensor
        .set_accel_mode_and_odr(&mut timer, AccelMode::Normal, AccelOutputDataRate::Hz50)
        .unwrap();
    timer.delay_ms(1000_u32);
    get_data(&mut sensor);

    defmt::info!("low power mode");
    sensor
        .set_accel_mode_and_odr(&mut timer, AccelMode::LowPower, AccelOutputDataRate::Hz50)
        .unwrap();
    timer.delay_ms(1000_u32);
    get_data(&mut sensor);

    defmt::info!("high resolution mode");
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();
    timer.delay_ms(1000_u32);
    get_data(&mut sensor);

    loop {
        timer.delay_ms(100_u32);
        get_data(&mut sensor);
    }
}

#[cfg(feature = "v1")]
type Sensor = Lsm303agr<I2cInterface<twi::Twi<TWI0>>, MagOneShot>;

#[cfg(feature = "v2")]
type Sensor = Lsm303agr<I2cInterface<twim::Twim<TWIM0>>, MagOneShot>;

fn get_data(sensor: &mut Sensor) {
    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            defmt::info!("x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
            return;
        }
    }
}
