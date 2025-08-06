#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
use core::{ffi::c_int, panic::PanicInfo};

pub mod colors;

use fwwasm_ffi::{LEDManagerLEDMode, exitToMainAppMenu, printInt, printOutColor, printOutDataType};

// TODO
// Display background fwi.
// Play sound on start.
// Open data file and display text stuff.
// Make button play audio.
// Have other buttons do stuff?

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            printInt(
                c"\nPanic: %d\n".as_ptr(),
                printOutColor::printColorNormal,
                printOutDataType::printUInt32,
                0,
            );
            exitToMainAppMenu();
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn main() {
    flash_leds(colors::LIGHT_BLUE, 1000);

    //setup_panels();

    play_audio_file();

    read_file();

    loop {
        process_events();
        return;
    }
}

unsafe fn flash_leds(color: colors::Color, duration_ms: i32) {
    // Flash the LEDs
    for i in 0..7 {
        fwwasm_ffi::setBoardLED(
            i,
            color.red / 20,
            color.green / 20,
            color.blue / 20,
            duration_ms,
            LEDManagerLEDMode::ledflash,
        );
    }
}

unsafe fn read_file() {
    let badge_file_handle: i32 = fwwasm_ffi::openFile(c"/build_a_badge.txt".as_ptr(), 0);
    if badge_file_handle == 0 {
        printInt(
            c"\nFile open failed\n".as_ptr(),
            printOutColor::printColorRed,
            printOutDataType::printUInt32,
            0,
        );
        exitToMainAppMenu();
        return;
    }
    let mut data_bytes = [0u8; 1024];
    let mut read_bytes = 0;
    let result = fwwasm_ffi::readFile(badge_file_handle, data_bytes.as_mut_ptr(), &mut read_bytes);
    fwwasm_ffi::printInt(
        c"\nFile Opened: %d\n".as_ptr(),
        printOutColor::printColorNormal,
        printOutDataType::printUInt32,
        read_bytes as i32,
    );

    fwwasm_ffi::printInt(
        c"read %d of data from file!\n".as_ptr(),
        printOutColor::printColorRed,
        printOutDataType::printUInt32,
        read_bytes as i32,
    );
}

unsafe fn play_audio_file() {
    fwwasm_ffi::playSoundFromFile(c"build_a_badge.wav".as_ptr());
}
unsafe fn setup_panels() {
    // Setup the main panel that shows pip boy
    fwwasm_ffi::addPanel(0, 1, 0, 0, 0, 0, 0, 0, 0);
    fwwasm_ffi::addControlPictureFromFile(0, 0, 0, 0, c"build_a_badge.fwi".as_ptr(), 1);
    fwwasm_ffi::addControlText(
        0,
        1,
        90,
        180,
        1,
        64,
        colors::WHITE.red,
        colors::WHITE.green,
        colors::WHITE.blue,
        c"Press a Button".as_ptr(),
    );
    fwwasm_ffi::showPanel(0);
    // Setup the rest of the panels
    // for panel in PANELS.iter() {
    //     fwwasm_ffi::addPanel(
    //         panel.index,
    //         1,
    //         0,
    //         0,
    //         0,
    //         panel.color.red,
    //         panel.color.green,
    //         panel.color.blue,
    //         0,
    //     );
    //     addControlText(panel.index, 1, 10, 50, 2, 0, 0, 0, 0, panel.text);
    // }
}

const BUTTONS: [u32; 5] = [
    fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON.0,
    fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON.0,
    fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON.0,
    fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON.0,
    fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_RED_BUTTON.0,
];

unsafe fn process_events() {
    loop {
        fwwasm_ffi::waitms(33);
        if fwwasm_ffi::hasEvent() == 0 {
            continue;
        }
        let mut event_data = [0; fwwasm_ffi::FW_GET_EVENT_DATA_MAX as usize];
        let last_event = fwwasm_ffi::getEventData(event_data.as_mut_ptr());
        printInt(
            c"\nLast event: %d\n".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            last_event as i32,
        );
        // We only want to process button presses
        if !BUTTONS.contains(&(last_event as u32)) {
            continue;
        }
        if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON.0 == last_event as u32 {
            flash_leds(colors::WHITE, 1000);
        } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON.0 == last_event as u32 {
            flash_leds(colors::YELLOW, 1000);
        } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON.0 == last_event as u32 {
            flash_leds(colors::GREEN, 1000);
        } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON.0 == last_event as u32 {
            flash_leds(colors::BLUE, 1000);
        } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_RED_BUTTON.0 == last_event as u32 {
            flash_leds(colors::RED, 1000);
            fwwasm_ffi::exitToMainAppMenu();
            return;
        }
    }
}
