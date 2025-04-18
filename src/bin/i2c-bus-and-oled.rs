use embedded_graphics::Drawable;
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::Peripherals,
    units::KiloHertz,
};
use shared_bus::BusManagerSimple;
use ssd1306::mode::DisplayConfig;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;
    let config = I2cConfig::new().baudrate(KiloHertz::from(1).into());
    // 方式 1
    let i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
    // 方式 2
    // let i2c_driver = I2cDriver::new(peripherals.i2c0, peripherals.pins.gpio10, peripherals.pins.gpio8, &config)?;
    let bus = BusManagerSimple::new(i2c_driver);
    let mut sht = shtcx::shtc3(bus.acquire_i2c());
    let interface = ssd1306::I2CDisplayInterface::new(bus.acquire_i2c());
    let mut display = ssd1306::Ssd1306::new(
        interface,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();
    loop {
        sht.start_measurement(shtcx::PowerMode::NormalMode).unwrap();
        FreeRtos::delay_ms(100);
        let measurement = sht.get_measurement_result().unwrap();
        let text_style = embedded_graphics::mono_font::MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
            .text_color(embedded_graphics::pixelcolor::BinaryColor::On)
            .build();

        let temperature = measurement.temperature.as_degrees_celsius();
        let humidity = measurement.humidity.as_percent();
        let temperature_str = format!("Temperature: {:.2} °C", temperature);
        let humidity_str = format!("Humidity: {:.2} %", humidity);
        embedded_graphics::text::Text::with_baseline(
            &temperature_str,
            embedded_graphics::prelude::Point::zero(),
            text_style,
            embedded_graphics::text::Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        embedded_graphics::text::Text::with_baseline(
            &humidity_str,
            embedded_graphics::prelude::Point::new(0, 16),
            text_style,
            embedded_graphics::text::Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
