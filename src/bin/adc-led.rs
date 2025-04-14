use esp_idf_svc::hal::{
    adc::oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
    prelude::Peripherals,
    units::Hertz,
};

// 据 AI 回答，oneshot 模式比 continuous 模式更稳定而且准确
// 在 wokwi 上测试，continuous 模式下，根本测量不出结果，实际测量时，发现该模式下的测量结果波动极大
// 而 oneshot 模式下，测量结果稳定且准确
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("ADC-LED oneshot and PWM example");

    let peripherals = Peripherals::take().unwrap();

    // ADC oneshot mode driver
    let adc_driver = AdcDriver::new(peripherals.adc1)?;
    let adc_channel_config = AdcChannelConfig {
        attenuation: esp_idf_svc::hal::adc::attenuation::DB_11,
        ..Default::default()
    };
    let mut adc_channel_driver =
        AdcChannelDriver::new(adc_driver, peripherals.pins.gpio3, &adc_channel_config)?;

    // LEDC driver
    let timer_config = TimerConfig {
        frequency: Hertz(5000),
        resolution: Resolution::Bits12,
        ..Default::default()
    };
    let ledc_timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;
    let mut ledc_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        ledc_timer_driver,
        peripherals.pins.gpio6,
    )?;
    loop {
        let value = adc_channel_driver.read_raw()?;
        log::info!("ADC raw value: {}", value);
        set_brightness(&mut ledc_driver, value)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn set_brightness(ledc_driver: &mut LedcDriver, value: u16) -> anyhow::Result<()> {
    let max_duty = ledc_driver.get_max_duty();
    // 注意：先进行乘法再进行除法，否则结果为 0（非浮点数运算）
    let duty = value as u32 * max_duty / (2u32.pow(12) - 1);
    log::info!("led current duty: {}", duty);
    ledc_driver.set_duty(duty)?;
    Ok(())
}
