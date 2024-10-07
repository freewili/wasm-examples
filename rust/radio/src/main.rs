#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/fwwasm.rs"));

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

const RED: Color = Color {
    red: 0x30,
    green: 0x00,
    blue: 0x00,
};

const YELLOW: Color = Color {
    red: 0x30,
    green: 0x30,
    blue: 0x00,
};
const GREEN: Color = Color {
    red: 0x00,
    green: 0x30,
    blue: 0x00,
};

const BLUE: Color = Color {
    red: 0x00,
    green: 0x00,
    blue: 0x30,
};

const GRAY: Color = Color {
    red: 0x30,
    green: 0x30,
    blue: 0x30,
};

const NUM_LEDS: u32 = 7;

fn display_image() {
    // Create a panel to display the image
    unsafe {
        // Setup the PipBoy image
        addPanel(0, 1, 0, 0, 0, 0, 0, 0, 0);
        addControlPictureFromFile(0, 0, 0, 0, c"pip_boy.fwi".as_ptr(), 1);
        addControlText(0, 1, 0, 0, 2, 64, 255, 255, 255, c"PIP BOY".as_ptr());
        showPanel(0);

        addPanel(
            1,
            1,
            0,
            0,
            0,
            GRAY.red as i32,
            GRAY.green as i32,
            GRAY.blue as i32,
            0,
        );
        addControlText(
            1,
            0,
            10,
            50,
            2,
            64,
            255,
            255,
            255,
            c"Transmitting\nRainbow...".as_ptr(),
        );
        addPanel(
            2,
            1,
            0,
            0,
            0,
            YELLOW.red as i32 * 5,
            YELLOW.green as i32 * 5,
            YELLOW.blue as i32 * 5,
            0,
        );
        addControlText(2, 0, 10, 50, 2, 64, 0, 0, 0, c"Transmitting!".as_ptr());
        addPanel(
            3,
            1,
            0,
            0,
            0,
            GREEN.red as i32 * 5,
            GREEN.green as i32 * 5,
            GREEN.blue as i32 * 5,
            0,
        );
        addControlText(3, 0, 10, 50, 2, 64, 0, 0, 0, c"Transmitting!".as_ptr());
        addPanel(
            4,
            1,
            0,
            0,
            0,
            BLUE.red as i32 * 5,
            BLUE.green as i32 * 5,
            BLUE.blue as i32 * 5,
            0,
        );
        addControlText(4, 0, 10, 50, 2, 64, 0, 0, 0, c"Transmitting!".as_ptr());
        addPanel(
            5,
            1,
            0,
            0,
            0,
            RED.red as i32 * 5,
            RED.green as i32 * 5,
            RED.blue as i32 * 5,
            0,
        );
        addControlText(5, 0, 10, 50, 2, 64, 0, 0, 0, c"Transmitting!".as_ptr());
        addControlText(5, 1, 10, 100, 1, 64, 0, 0, 0, c"Repeat 3".as_ptr());
        addControlText(5, 2, 10, 120, 1, 64, 0, 0, 0, c"times to".as_ptr());
        addControlText(5, 3, 10, 140, 1, 64, 0, 0, 0, c"exit!".as_ptr());
    };
}

#[no_mangle]
pub extern "C" fn main() -> () {
    println!("Hello, World!");

    display_image();

    unsafe {
        playSoundFromNameOrID(core::ptr::null(), 77);

        printInt(
            c"Hello, World %d!\n".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            0 as i32,
        );
    };

    let startup_color = Color {
        red: 0x10,
        green: 0x10,
        blue: 0x10,
    };
    // Set the startup LED color to a dim white.
    for led_index in 0..NUM_LEDS {
        unsafe {
            setBoardLED(
                led_index as i32,
                startup_color.red.into(),
                startup_color.green.into(),
                startup_color.blue.into(),
                300,
                LEDManagerLEDMode::ledsimplevalue,
            )
        };
    }

    unsafe {
        printInt(
            c"Listening to events...\n".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            0 as i32,
        );
    };
    let mut red_count = 0;
    loop {
        unsafe {
            waitms(33);
            if (hasEvent()) == 0 {
                continue;
            }

            let mut event_data = [0; FW_GET_EVENT_DATA_MAX as usize];
            let last_event = getEventData(event_data.as_mut_ptr());
            printInt(
                c"Event: %d...\n".as_ptr(),
                printOutColor::printColorNormal,
                printOutDataType::printUInt32,
                last_event as i32,
            );
            let (color, fname) = if last_event == FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON as i32 {
                //showPanel(1);
                (&GRAY, c"/radio/rainbow.sub")
            } else if last_event == FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON as i32 {
                (&YELLOW, c"/radio/yellow.sub")
            } else if last_event == FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON as i32 {
                (&GREEN, c"/radio/green.sub")
            } else if last_event == FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON as i32 {
                (&BLUE, c"/radio/blue.sub")
            } else if last_event == FWGuiEventType::FWGUI_EVENT_RED_BUTTON as i32 {
                (&RED, c"/radio/red.sub")
            } else {
                continue;
            };
            if last_event == FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON as i32 {
                showPanel(1);
            } else if last_event == FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON as i32 {
                showPanel(2);
            } else if last_event == FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON as i32 {
                showPanel(3);
            } else if last_event == FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON as i32 {
                showPanel(4);
            } else if last_event == FWGuiEventType::FWGUI_EVENT_RED_BUTTON as i32 {
                showPanel(5);
            } else {
                continue;
            };

            for led_index in 0..NUM_LEDS {
                setBoardLED(
                    led_index as i32,
                    color.red.into(),
                    color.green.into(),
                    color.blue.into(),
                    250,
                    LEDManagerLEDMode::ledflashfade,
                );
            }
            printInt(
                fname.as_ptr(),
                printOutColor::printColorNormal,
                printOutDataType::printUInt32,
                last_event as i32,
            );
            RadioSetTx(1);
            let res = RadioTxSubFile(1, fname.as_ptr() as *mut core::ffi::c_char);
            printInt(
                c"\nRadioTxSubFile: %d\n".as_ptr(),
                printOutColor::printColorNormal,
                printOutDataType::printUInt32,
                res as i32,
            );
            //waitms(1000);
            RadioSetIdle(1);
            for led_index in 0..NUM_LEDS {
                setBoardLED(
                    led_index as i32,
                    color.red.into(),
                    color.green.into(),
                    color.blue.into(),
                    100,
                    LEDManagerLEDMode::ledpulse,
                );
            }
            showPanel(0);

            if last_event == FWGuiEventType::FWGUI_EVENT_RED_BUTTON as i32 {
                red_count += 1;
                if red_count >= 3 {
                    RadioTxSubFile(1, c"/radio/off.sub".as_ptr() as *mut core::ffi::c_char);
                    break;
                }
            } else {
                red_count = 0;
            }
        };
    }

    unsafe {
        playSoundFromNameOrID(c"GoodJob".as_ptr(), -1);
        exitToMainAppMenu()
    };
}

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {
//         unsafe {
//             //setled(1);
//             waitms(33);
//             //setled(0);
//             waitms(33);
//         };
//     }
// }
