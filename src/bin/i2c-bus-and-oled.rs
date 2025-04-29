use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::{Point, Primitive, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
    Drawable,
};
use embedded_hal_bus::{i2c, util::AtomicCell};
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::Peripherals,
    units::KiloHertz,
};
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

    let i2c_cell = AtomicCell::new(i2c_driver);
    // 方式 2
    // let i2c_driver = I2cDriver::new(peripherals.i2c0, peripherals.pins.gpio10, peripherals.pins.gpio8, &config)?;
    let mut sht = shtcx::shtc3(i2c::AtomicDevice::new(&i2c_cell));
    let interface = ssd1306::I2CDisplayInterface::new(i2c::AtomicDevice::new(&i2c_cell));
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
        let text_style = u8g2_fonts::U8g2TextStyle::new(
            u8g2_fonts::fonts::u8g2_font_unifont_t_gb2312, // 选用这个，否则某些汉字无法显示
            BinaryColor::On,                               // Off 时为黑色，无法显示
        );
        let temperature = measurement.temperature.as_degrees_celsius();
        let temperature_str = format!("温度: {:.2} °C", temperature);
        Text::with_baseline(
            &temperature_str,
            Point::new(0, 4),
            text_style.clone(),
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        let humidity = measurement.humidity.as_percent();
        let humidity_str = format!("湿度: {:.2} %", humidity);
        Text::with_baseline(&humidity_str, Point::new(0, 23), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let ohm = &[0b00111000, 0b01000100, 0b01000100, 0b00101000, 0b11101110];
        // width 不一定是 8 的倍数，如果不是 8 的倍数时，相应位置的 u8 后几位就为 0
        let raw_image = ImageRaw::<BinaryColor>::new(ohm, 8);
        let image = Image::new(&raw_image, Point::new(0, 36));
        image.draw(&mut display).unwrap();

        Rectangle::new(Point::new(0, 42), Size::new(127, 22))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(BinaryColor::On)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
