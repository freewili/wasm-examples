#![no_std]
#![no_main]

use core::ffi::c_int;
use core::panic::PanicInfo;

mod fwwasm;
use fwwasm::{setBoardLED, setled, waitms, LEDManagerLEDMode_ledpulsefade};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

const RED: Color = Color {
    red: 0xFF,
    green: 0x00,
    blue: 0x00,
};
const PINK: Color = Color {
    red: 0xFF,
    green: 0xC6,
    blue: 0xFF,
};
const ORANGE: Color = Color {
    red: 0xFF,
    green: 0x7F,
    blue: 0x00,
};
const YELLOW: Color = Color {
    red: 0xFF,
    green: 0xFF,
    blue: 0x00,
};
const GREEN: Color = Color {
    red: 0x00,
    green: 0xFF,
    blue: 0x00,
};
const LIGHT_GREEN: Color = Color {
    red: 0xCA,
    green: 0xFF,
    blue: 0xBF,
};
const BLUE: Color = Color {
    red: 0x00,
    green: 0x00,
    blue: 0xFF,
};
const LIGHT_BLUE: Color = Color {
    red: 0x9B,
    green: 0xF6,
    blue: 0xFF,
};
const INDIGO: Color = Color {
    red: 0x4B,
    green: 0x00,
    blue: 0x82,
};
const VIOLET: Color = Color {
    red: 0x94,
    green: 0x00,
    blue: 0xD3,
};

const MAX_LOOPS: u32 = 20;
const NUM_LEDS: u32 = 7;

#[no_mangle]
pub extern "C" fn main() -> c_int {
    let rainbow_colors = [
        RED,
        PINK,
        ORANGE,
        YELLOW,
        GREEN,
        LIGHT_GREEN,
        BLUE,
        LIGHT_BLUE,
        INDIGO,
        VIOLET,
    ];

    let mut color_choice = 0;
    for _ in 0..MAX_LOOPS {
        for led_index in 0..NUM_LEDS {
            let color = &rainbow_colors[color_choice];
            unsafe {
                setBoardLED(
                    led_index as i32,
                    color.red.into(),
                    color.green.into(),
                    color.blue.into(),
                    300,
                    LEDManagerLEDMode_ledpulsefade,
                )
            };
            if color_choice >= rainbow_colors.len() - 1 {
                color_choice = 0;
            } else {
                color_choice += 1;
            }

            unsafe { waitms(50) };
        }
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            setled(1);
            waitms(33);
            setled(0);
            waitms(33);
        };
    }
}
