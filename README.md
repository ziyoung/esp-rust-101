# esp32 rust demo

## 事例

### ADC

[src/bin/adc-led.rs](src/bin/adc-led.rs)

测试下来，使用 `oneshot` 模式，采样率 1000Hz，精度 12bit，采样值 0-4095，衰减值为 DB11，最大电压为 2.5。
在 DB11 衰减下，相当于把 2.5v 分成了 4095 份。

> 如果在 gpio 上，不接任何线，也可以测量出一定的电压。
> 将 gpio 针脚接到 GND 上，电压为 0。

```
I (11418) adc_led: ADC raw value: 19
I (11418) adc_led: ADC value: 5

==== 移除 GND 后 ====

I (12418) adc_led: ADC raw value: 1502
I (12418) adc_led: ADC value: 854
```
