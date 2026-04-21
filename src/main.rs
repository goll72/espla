#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use esp_hal::Config;
use esp_hal::delay::Delay;

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::time::Rate;

use esp_hal::main;

use ssd1306::prelude::*;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

const CH1_LABEL: &'static str = "CH1";
const CH2_LABEL: &'static str = "CH2";

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let mut display = {
        let i2c_config = I2cConfig::default().with_frequency(Rate::from_khz(400));

        let i2c = I2c::new(peripherals.I2C0, i2c_config)
            .unwrap()
            .with_sda(peripherals.GPIO21)
            .with_scl(peripherals.GPIO22);

        let interface = I2CDisplayInterface::new(i2c);

        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode()
    };

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(embedded_graphics::pixelcolor::BinaryColor::On)
        .build();

    let ch1 = Input::new(
        peripherals.GPIO33,
        InputConfig::default().with_pull(Pull::None),
    );

    let ch2 = Input::new(
        peripherals.GPIO32,
        InputConfig::default().with_pull(Pull::None),
    );

    let delay = Delay::new();

    let mut pos = 0;

    const CH_PADDING: i32 = 12;
    const DISPLAY_HEIGHT: i32 = 64;
    const DISPLAY_WIDTH: i32 = 128;

    const INNER_CH_PADDING: i32 = 2;

    const N_CH: i32 = 2;

    const CH_HEIGHT: i32 =
        (DISPLAY_HEIGHT - (N_CH + 1) * CH_PADDING - 2 * N_CH * INNER_CH_PADDING) / N_CH;

    Rectangle::new(Point::new(0, 0), Size::new(128, 64))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(&mut display)
        .unwrap();

    Text::with_baseline(CH1_LABEL, Point::new(2, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline(
        CH2_LABEL,
        Point::new(2, 2 + (CH_HEIGHT + CH_PADDING + 2 * INNER_CH_PADDING) * 2),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();

    let mut p = [[0; 2]; N_CH as usize];
    let ch = [ch1, ch2];

    loop {
        for i in 0..N_CH {
            p[i as usize][pos % 2] = bool::from(ch[i as usize].level()).into();
        }

        for i in 0..N_CH {
            Line::new(
                Point::new(
                    pos as i32,
                    i * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING)
                        + CH_PADDING
                        + INNER_CH_PADDING
                        + CH_HEIGHT * (1 - p[i as usize][pos % 2]),
                ),
                Point::new(
                    pos as i32,
                    i * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING)
                        + CH_PADDING
                        + INNER_CH_PADDING
                        + CH_HEIGHT * (1 - p[i as usize][(pos + 1) % 2]),
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();
        }

        for i in 0..N_CH {
            Line::new(
                Point::new(
                    (pos + 1) as i32 % DISPLAY_WIDTH,
                    i * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING) + CH_PADDING
                        - INNER_CH_PADDING,
                ),
                Point::new(
                    (pos + 1) as i32 % DISPLAY_WIDTH,
                    (i + 1) * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING) + INNER_CH_PADDING,
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .draw(&mut display)
            .unwrap();

            Line::new(
                Point::new(
                    (pos + 2) as i32 % DISPLAY_WIDTH,
                    i * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING) + CH_PADDING
                        - INNER_CH_PADDING,
                ),
                Point::new(
                    (pos + 2) as i32 % DISPLAY_WIDTH,
                    (i + 1) * (CH_PADDING + CH_HEIGHT + 2 * INNER_CH_PADDING) + INNER_CH_PADDING,
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();
        }

        pos = (pos + 1) % DISPLAY_WIDTH as usize;

        if (pos % 2) == 0 {
            display.flush().unwrap();
        }

        delay.delay_millis(5);
    }
}
