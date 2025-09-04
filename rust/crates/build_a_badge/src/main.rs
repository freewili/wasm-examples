#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
use core::{ffi::c_int, panic::PanicInfo};

pub mod colors;

use fwwasm_ffi::{LEDManagerLEDMode, exitToMainAppMenu, printInt, printOutColor, printOutDataType};

// Control IDs
const PANEL_CONTROL: i32 = 0;
const BADGE_IMAGE_CONTROL: i32 = 0;
const BADGE_NAME_TEXT_CONTROL: i32 = 1;

// Musical note frequencies in Hz
const NOTE_E2 :f32 = 82.41;
const NOTE_A2 :f32 = 110.0;
const NOTE_E3 :f32 = 164.81;
const NOTE_CS3 :f32 = 138.59;
const NOTE_A3 :f32 = 220.0;
const NOTE_C4 :f32 = 261.63;
const NOTE_E4 :f32 = 329.63;
const NOTE_G4 :f32 = 392.0;

const NOTE_PLAY_DURATION :f32 = 0.5;

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
    setup_panels();

    // Load the badge name and led show mode from the text file that was created by build a badge app.
    if load_badge_values_from_text_file() == false
    {
        printInt(
            c"\nFailed to setup app!".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            0,
        );        
        exitToMainAppMenu();
        return;
    }

    // Main event loop
    loop 
    {
        process_events();
        return;
    }
}

//Opens the build_a_badge.txt file and reads the badge name and led show mode from it.
//The first line is the badge name, the second line is the led show mode (as an integer).
unsafe fn load_badge_values_from_text_file() -> bool{

    // Open the file for reading.
    let badge_file_handle: i32 = fwwasm_ffi::openFile(c"/build_a_badge.txt".as_ptr(), 0);
    if badge_file_handle == 0 {
        printInt(
            c"\nFailed to open build_a_badge.txt!\n".as_ptr(),
            printOutColor::printColorRed,
            printOutDataType::printUInt32,
            0,
        );
        return false;
    }

    // Read the badge name from the first line of the file.
    let mut line = [0i8; 255];
    let actual_read = fwwasm_ffi::readFileLine(badge_file_handle, line.as_mut_ptr(), 255);
    if actual_read == 0
    {        
        fwwasm_ffi::printInt(
                c"Failed to get badge name!".as_ptr(),
                printOutColor::printColorRed,
                printOutDataType::printUInt32, // Use the correct enum for string if available
                0,
            );  
        fwwasm_ffi::closeFile(badge_file_handle);         
        return false;
    }

    // Set badge name text control to the name we read from the file.
    fwwasm_ffi::setControlValueText(PANEL_CONTROL, BADGE_NAME_TEXT_CONTROL, line.as_ptr());           
    fwwasm_ffi::printInt(
            line.as_ptr(),
            printOutColor::printColorRed,
            printOutDataType::printUInt32,
            0,
        );

    // Read the led show mode from the second line of the file.
    let nextline = fwwasm_ffi::readFileLine(badge_file_handle, line.as_mut_ptr(), 255);
    if nextline == 0
    {
        fwwasm_ffi::printInt(
                c"Failed to get badge led show!".as_ptr(),
                printOutColor::printColorRed,
                printOutDataType::printUInt32, // Use the correct enum for string if available
                0,
            );  
        fwwasm_ffi::closeFile(badge_file_handle);         
        return false;
    } 

    // Convert the i8 slice to a u8 slice for UTF-8 parsing.
    let u8_slice = unsafe {
        core::slice::from_raw_parts(line.as_ptr() as *const u8, nextline as usize)
    };

    // Parse the led show mode as an integer.
    let ledshowindex = core::str::from_utf8(u8_slice).unwrap_or("<invalid utf8>");
    if let Ok(parsed_int) = ledshowindex.trim().parse::<i32>() 
    {
        fwwasm_ffi::printInt(
                c"Led Show selected %d".as_ptr(),
                printOutColor::printColorRed,
                printOutDataType::printUInt32, // Use the correct enum for string if available
                parsed_int,
            );

        // Set the LED show mode that the user selected.
        fwwasm_ffi::setLEDShowMode(parsed_int);
    } 
    else 
    {
        //Ahhhh! Couldn't parse the int for the LED show!
        fwwasm_ffi::printInt(
        c"Failed to parse LED show mode !".as_ptr(),
        printOutColor::printColorRed,
        printOutDataType::printUInt32, // Use the correct enum for string if available
        0);
    }

    // Close the file now that we're done with it.
    let close_res = fwwasm_ffi::closeFile(badge_file_handle);
    if close_res == 0
    {
        fwwasm_ffi::printInt(
        c"Failed Close Badge File!".as_ptr(),
        printOutColor::printColorRed,
        printOutDataType::printUInt32, // Use the correct enum for string if available
        0);
    }

    return true;
}

unsafe fn setup_panels() {
    // Setup the main panel for wasm app
    fwwasm_ffi::addPanel(0, 1, 0, 0, 0, 0, 0, 0, 0);

    // Add picture control for the badge image that was loaded from build a badge app
    fwwasm_ffi::addControlPictureFromFile(PANEL_CONTROL, BADGE_IMAGE_CONTROL, 0, 0, c"build_a_badge.fwi".as_ptr(), 1);

    // Add text control for the badge name that was loaded from build a badge app
    fwwasm_ffi::addControlText(
        PANEL_CONTROL,
        BADGE_NAME_TEXT_CONTROL,
        90,
        180,
        1,
        64,
        colors::WHITE.red,
        colors::WHITE.green,
        colors::WHITE.blue,
        c"".as_ptr(),
    );

    // Show that panel!
    fwwasm_ffi::showPanel(0);
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
            c"Last event: %d".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            last_event as i32,
        );

        // We only want to process button presses
        if !BUTTONS.contains(&(last_event as u32)) {
            continue;
        }

        // Check if button was pressed or released
        let pressed = event_data[0] == 0;

        // Do stuff based on which button was pressed
        if pressed
        {
            if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON.0 == last_event as u32 {
                fwwasm_ffi::playSoundFromFrequencyAndDuration(NOTE_E3, NOTE_PLAY_DURATION, 0.2, 1);
            } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON.0 == last_event as u32 {
                fwwasm_ffi::playSoundFromFrequencyAndDuration(NOTE_CS3, NOTE_PLAY_DURATION, 0.2, 1);
            } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON.0 == last_event as u32 {
                fwwasm_ffi::playSoundFromFrequencyAndDuration(NOTE_A3, NOTE_PLAY_DURATION, 0.2, 1);
            } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON.0 == last_event as u32 {
                fwwasm_ffi::playSoundFromFrequencyAndDuration(NOTE_C4, NOTE_PLAY_DURATION, 0.2, 1);
            } else if fwwasm_ffi::FWGuiEventType::FWGUI_EVENT_RED_BUTTON.0 == last_event as u32 {
                fwwasm_ffi::exitToMainAppMenu();
                break;
            }            
        }
    }
}