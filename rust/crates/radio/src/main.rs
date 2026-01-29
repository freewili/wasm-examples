#![no_std]
#![no_main]
use fwwasm_ffi::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
     loop {}
}

const NUMBER_OF_LEDS: u32 = 7;

#[derive(Clone, Copy)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Color {
    const fn new(red: i32, green: i32, blue: i32) -> Self {
        Self { red, green, blue }
    }
}

const RED: Color = Color::new(255, 0, 0);
const ORANGE: Color = Color::new(255, 127, 0);
const YELLOW: Color = Color::new(255, 255, 0);
const GREEN: Color = Color::new(0, 255, 0);
const LIGHT_GREEN: Color = Color::new(0, 255, 191);
const BLUE: Color = Color::new(0, 0, 255);
const LIGHT_BLUE: Color = Color::new(0, 191, 255);
const INDIGO: Color = Color::new(75, 0, 130);
const VIOLET: Color = Color::new(238, 130, 238);
const PINK: Color = Color::new(255, 192, 203);
const GRAY: Color = Color::new(0x30, 0x30, 0x30);
const WHITE: Color = Color::new(255, 255, 255);

// Main panel index - Start at 1 since 0 is reserved for the event log
const MAIN_PANEL_INDEX: i32 = 1;

struct PanelInfo {
    index: i32,
    event_type: FWGuiEventType,
    color: Color,
    text: *const core::ffi::c_char,
    sub_fname: *const core::ffi::c_char,
}

impl PanelInfo {
    const fn new(
        index: i32,
        event_type: FWGuiEventType,
        color: Color,
        text: *const core::ffi::c_char,
        sub_fname: *const core::ffi::c_char,
    ) -> Self {
        Self {
            index,
            event_type,
            color,
            text,
            sub_fname,
        }
    }
}

const PANELS: [PanelInfo; 5] = [
    PanelInfo::new(
        2,
        FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON,
        GRAY,
        c"GRAY".as_ptr(),
        c"/radio/white.sub".as_ptr(),
    ),
    PanelInfo::new(
        3,
        FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON,
        YELLOW,
        c"YELLOW".as_ptr(),
        c"/radio/yellow.sub".as_ptr(),
    ),
    PanelInfo::new(
        4,
        FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON,
        GREEN,
        c"GREEN".as_ptr(),
        c"/radio/green.sub".as_ptr(),
    ),
    PanelInfo::new(
        5,
        FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON,
        BLUE,
        c"BLUE".as_ptr(),
        c"/radio/blue.sub".as_ptr(),
    ),
    PanelInfo::new(
        6,
        FWGuiEventType::FWGUI_EVENT_RED_BUTTON,
        RED,
        c"RED".as_ptr(),
        c"/radio/red.sub".as_ptr(),
    ),
];

const BUTTONS: [u32; 5] = [
    FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON.0,
    FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON.0,
    FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON.0,
    FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON.0,
    FWGuiEventType::FWGUI_EVENT_RED_BUTTON.0,
];

fn setup_panels() {
    unsafe {
        // Setup the main panel that shows pip boy
        addPanel(MAIN_PANEL_INDEX, 1, 0, 0, 0, 0, 0, 0, 0);
        addControlPictureFromFile(MAIN_PANEL_INDEX, 0, 0, 0, c"pip_boy.fwi".as_ptr(), 1);
        addControlText(
            MAIN_PANEL_INDEX,
            1,
            90,
            180,
            1,
            64,
            WHITE.red,
            WHITE.green,
            WHITE.blue,
            c"Press a Button".as_ptr(),
        );
        showPanel(MAIN_PANEL_INDEX);
        // Setup the rest of the panels
        for panel in PANELS.iter() {
            addPanel(
                panel.index,
                1,
                0,
                0,
                0,
                panel.color.red,
                panel.color.green,
                panel.color.blue,
                0,
            );
            addControlText(panel.index, 1, 10, 50, 2, 0, 0, 0, 0, panel.text);
        }
    }
}

fn show_rainbow_leds(max_loops: u32) {
    unsafe {
        const COLORS: [Color; 10] = [
            RED,
            ORANGE,
            YELLOW,
            GREEN,
            LIGHT_GREEN,
            BLUE,
            LIGHT_BLUE,
            INDIGO,
            VIOLET,
            PINK,
        ];
        let mut color_choice = 0;
        // do the whole thing multiple times
        for _ in 0..max_loops {
            // set every LED one at a time
            for led in 0..NUMBER_OF_LEDS {
                let color = COLORS[color_choice];
                // set the LED
                setBoardLED(
                    led as i32,
                    color.red.into(),
                    color.green.into(),
                    color.blue.into(),
                    300,
                    LEDManagerLEDMode::ledpulsefade,
                );
                // next time, get a new color.  If we used all of the colors, start over
                color_choice = (color_choice + 1) % COLORS.len();
                // wait before setting the next LED
                waitms(50);
            }
        }
    };
}

fn process_events() {
    unsafe {
        printInt(
            c"\nListening to events...\n".as_ptr(),
            printOutColor::printColorNormal,
            printOutDataType::printUInt32,
            0 as i32,
        );
        let mut red_count = 0;
        loop {
            waitms(33);
            if hasEvent() == 0 {
                continue;
            }
            let mut event_data = [0; FW_GET_EVENT_DATA_MAX as usize];
            let last_event = getEventData(event_data.as_mut_ptr());
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
            for panel in PANELS.iter() {
                if panel.event_type.0 != last_event as u32 {
                    continue;
                }
                showPanel(panel.index);
                for led in 0..NUMBER_OF_LEDS {
                    setBoardLED(
                        led as i32,
                        panel.color.red.into(),
                        panel.color.green.into(),
                        panel.color.blue.into(),
                        300,
                        LEDManagerLEDMode::ledpulsefade,
                    );
                }
                let _ = RadioTxSubFile(1, panel.sub_fname);
                // Wait for the radio to stop transmitting
                while RadioSubFileIsTransmitting() != 0 {
                    waitms(33);
                }
                // show the main panel
                showPanel(MAIN_PANEL_INDEX);
                break;
            }
            // we need an exit condition
            if last_event as u32 == FWGuiEventType::FWGUI_EVENT_RED_BUTTON.0 {
                printInt(c"\nLast event was red button: %d\n".as_ptr(), printOutColor::printColorNormal,
                        printOutDataType::printUInt32, red_count);
                red_count += 1;
                if red_count >= 3 {
                    let _ = RadioTxSubFile(1, c"/radio/off.sub".as_ptr());
                    // Wait for the radio to stop transmitting
                    while RadioSubFileIsTransmitting() != 0 {
                        waitms(33);
                    }
                    exitToMainAppMenu();
                    break;
                }
            } else {
                red_count = 0;
            }
        }
    }
}

// GUI Auto Button Levels
// This is temporarily defined here until it is added to fwwasm-ffi
// #[allow(dead_code)]
// const GUI_AUTO_BUTTON_LEVEL_LONG_PRESS_HOME_ONLY: i32 = 0;
// #[allow(dead_code)]
// const GUI_AUTO_BUTTON_LEVEL_HOME_AND_STOP: i32 = 1;
// #[allow(dead_code)]
// const GUI_AUTO_BUTTON_LEVEL_HOME_ONLY_LONG_STOP: i32 = 2;
// #[allow(dead_code)]
// const GUI_AUTO_BUTTON_LEVEL_HOME_ONLY: i32 = 3;
// #[allow(dead_code)]
// const GUI_AUTO_BUTTON_LEVEL_NONE: i32 = 4;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    // Disable automatic handling of Blue and Red buttons
    unsafe { setCanDisplayReactToButtons(4); }
    
    setup_panels();

    show_rainbow_leds(5);

    process_events();
}
