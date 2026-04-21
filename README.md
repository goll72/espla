espla
=====

A _barebones_, _ad hoc_ logic analyzer for the ESP32, using a 128x64
OLED display driven by the SSD1306 display controller over I2C to
display two channels, which correspond to GPIO pins 33 and 32, respectively.

Some parameters are configurable but changing them requires editing the
source code, such as channel count, channel height, sampling period, etc.

## Characteristics

While the ESP32 runs at 40MHz, the I2C link can only go up to 400kHz,
and the display itself is going to have a much lower refresh rate, so
don't expect being able to view signals at frequencies over a few
dozens of kHz using this.

> [!NOTE]
>
> The ESP32's GPIO pins can handle up to `3.6V`, but ideally you shouldn't
> load them with anything over `3.3V`. According to the datasheet, voltage
> levels higher than `0.75VCC` (~`2.5V`) are guaranteed to be read as logic
> level high, while voltage levels lower than `0.25VCC` are guaranteed to be
> read as low. If you want to analyze signals with a higher voltage than the
> maximum allowed (e.g. `5V`), a voltage divider circuit may be used.
