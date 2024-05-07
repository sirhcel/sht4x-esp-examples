use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;
use sht4x::{Precision, Sht4x};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let config = I2cConfig::new()
        .baudrate(100.kHz().into())
        .scl_enable_pullup(true)
        .sda_enable_pullup(true);
    let i2c = I2cDriver::new(peripherals.i2c0, pins.gpio19, pins.gpio18, &config)?;
    let mut delay = Ets;
    let mut sensor = Sht4x::new(i2c);

    log::info!("Hello, world!");

    log::info!("sensor serial: {:?}", &sensor.serial_number(&mut delay));

    loop {
        let result = sensor.measure(Precision::High, &mut delay);
        log::info!("measurement: {:?}", result);

        thread::sleep(Duration::from_millis(2500));
    }
}
