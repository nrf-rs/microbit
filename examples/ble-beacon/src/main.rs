#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use microbit::hal::{clocks::Clocks, timer::Timer};
use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, CompanyId, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;

#[entry]
fn main() -> ! {
    static mut BLE_TX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
    static mut BLE_RX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
    if let Some(p) = microbit::Peripherals::take() {
        // On reset, the internal high frequency clock is already used, but we
        // also need to switch to the external HF oscillator. This is needed
        // for Bluetooth to work.
        let _clocks = Clocks::new(p.CLOCK).enable_ext_hfosc();

        // Determine device address
        let device_address = get_device_address();

        // Rubble currently requires an RX buffer even though the radio is only used as a TX-only beacon.
        let mut radio = BleRadio::new(p.RADIO, &p.FICR, BLE_TX_BUF, BLE_RX_BUF);

        let mut timer = Timer::new(p.TIMER0);

        loop {
            // Broadcast local name
            let local_name = "Rusty microbit";
            defmt::info!("Local name: {}", local_name);
            let beacon = Beacon::new(
                device_address,
                &[AdStructure::CompleteLocalName(local_name)],
            )
            .unwrap();
            beacon.broadcast(&mut radio);
            timer.delay_ms(100_u16);

            // Broadcast data
            let data = "Hello world";
            defmt::info!("Data: {}", data);
            let beacon = Beacon::new(
                device_address,
                &[AdStructure::ManufacturerSpecificData {
                    company_identifier: CompanyId::from_raw(0xffff),
                    payload: data.as_bytes(),
                }],
            )
            .unwrap();
            beacon.broadcast(&mut radio);
            timer.delay_ms(500_u16);
        }
    }

    loop {}
}
