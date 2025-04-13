// use esp_idf_svc::hal::adc::{AdcContConfig, AdcContDriver, AdcMeasurement, Attenuated};
// use esp_idf_svc::hal::peripherals::Peripherals;

// fn main() -> anyhow::Result<()> {
//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();

//     let peripherals = Peripherals::take()?;
//     let config = AdcContConfig::default();

//     let adc_1_channel_0 = Attenuated::db11(peripherals.pins.gpio3);
//     let mut adc = AdcContDriver::new(peripherals.adc1, &config, adc_1_channel_0)?;

//     adc.start()?;

//     let mut samples = [AdcMeasurement::default(); 100];

//     loop {
//         match adc.read(&mut samples, 1000) {
//             Ok(num_read) => {
//                 log::info!("Read {} measurement.", num_read);
//                 let sum: u64 = samples[..num_read].iter().map(|x| x.data() as u64).sum();
//                 let avg = (sum / (num_read as u64)) as u16; // 如果需要 u16 类型的结果
//                 log::info!("Average measurement: {}", avg);
//             },
//             Err(e) => {
//                 log::error!("Error reading measurement: {}", e);
//             }
//         }

//         log::info!("Waiting for next measurement...");
//         std::thread::sleep(std::time::Duration::from_millis(2000));
//     }
// }

use std::thread;
use std::time::Duration;

use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_svc::hal::prelude::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: DB_11,
        ..Default::default()
    };
    let mut adc_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio3, &config)?;

    loop {
        log::info!("Reading ADC value...");
        thread::sleep(Duration::from_millis(1000));
        log::info!("ADC raw value: {}", adc.read_raw(&mut adc_pin)?);
        log::info!("ADC value: {}", adc.read(&mut adc_pin)?);
        log::info!("Waiting for next measurement...");
    }
}
