const std = @import("std");

const fw = @cImport({
    @cInclude("fwwasm.h");
});

const NUMBER_OF_LEDS: i32 = 7;

const Color = struct {
    red: u8,
    green: u8,
    blue: u8,
};

const RED = Color{ .red = 0xFF, .green = 0x00, .blue = 0x00 };
const ORANGE = Color{ .red = 0xFF, .green = 0x7F, .blue = 0x00 };
const YELLOW = Color{ .red = 0xFF, .green = 0xFF, .blue = 0x00 };
const GREEN = Color{ .red = 0x00, .green = 0xFF, .blue = 0x00 };
const LIGHT_GREEN = Color{ .red = 0xCA, .green = 0xFF, .blue = 0x00 };
const BLUE = Color{ .red = 0x00, .green = 0x00, .blue = 0xBF };
const LIGHT_BLUE = Color{ .red = 0x9B, .green = 0xF6, .blue = 0xFF };
const INDIGO = Color{ .red = 0x4B, .green = 0x00, .blue = 0x82 };
const VIOLET = Color{ .red = 0x94, .green = 0x00, .blue = 0xD3 };
const PINK = Color{ .red = 0xFF, .green = 0xC6, .blue = 0xFF };
const GRAY = Color{ .red = 0x30, .green = 0x30, .blue = 0x30 };
const WHITE = Color{ .red = 0xFF, .green = 0xFF, .blue = 0xFF };

const PanelInfo = struct {
    index: u8,
    event_type: fw.FWGuiEventType,
    color: Color,
    text: []const u8,
    sub_fname: []const u8,
};

const Panels = [_]PanelInfo{
    PanelInfo{ .index = 1, .event_type = fw.FWGUI_EVENT_GRAY_BUTTON, .color = GRAY, .text = "GRAY", .sub_fname = "/radio/white.sub" },
    PanelInfo{ .index = 2, .event_type = fw.FWGUI_EVENT_YELLOW_BUTTON, .color = YELLOW, .text = "YELLOW", .sub_fname = "/radio/yellow.sub" },
    PanelInfo{ .index = 3, .event_type = fw.FWGUI_EVENT_GREEN_BUTTON, .color = GREEN, .text = "GREEN", .sub_fname = "/radio/green.sub" },
    PanelInfo{ .index = 4, .event_type = fw.FWGUI_EVENT_BLUE_BUTTON, .color = BLUE, .text = "BLUE", .sub_fname = "/radio/blue.sub" },
    PanelInfo{ .index = 5, .event_type = fw.FWGUI_EVENT_RED_BUTTON, .color = RED, .text = "RED", .sub_fname = "/radio/red.sub" },
};

const Buttons = [_]fw.FWGuiEventType{
    fw.FWGUI_EVENT_GRAY_BUTTON,
    fw.FWGUI_EVENT_YELLOW_BUTTON,
    fw.FWGUI_EVENT_GREEN_BUTTON,
    fw.FWGUI_EVENT_BLUE_BUTTON,
    fw.FWGUI_EVENT_RED_BUTTON,
};

pub fn setup_panels() void {
    // Create a panel to display the image
    fw.addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    fw.addControlPictureFromFile(0, 0, 0, 0, @ptrCast("pip_boy.fwi"), 1);
    // Show the Panel
    fw.showPanel(0);

    for (Panels) |panel| {
        fw.addPanel(panel.index, 1, 0, 0, 0, panel.color.red, panel.color.green, panel.color.blue, 1);
        fw.addControlText(panel.index, 1, 10, 50, 2, 0, 0, 0, 0, @ptrCast(panel.text));
    }
}

pub fn show_rainbow_leds(max_loops: u32) void {
    const Colors = [_]Color{
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
        GRAY,
        WHITE,
    };
    var color_choice: u32 = 0;
    for (0..max_loops) |_| {
        for (0..NUMBER_OF_LEDS) |led_index| {
            // Pick the color and set it
            const color: Color = Colors[color_choice];
            fw.setBoardLED(@intCast(led_index), color.red, color.green, color.blue, 300, fw.ledpulsefade);
            // Make sure we don't overflow the array
            if (color_choice >= Colors.len - 1) {
                color_choice = 0;
            } else {
                color_choice += 1;
            }
            fw.waitms(50);
        }
    }
}

pub fn process_events() void {
    fw.printInt(@ptrCast("\nListening to events...\n"), fw.printColorNormal, fw.printUInt32, 0);
    var red_count: u8 = 0;
    while (true) {
        fw.waitms(33);
        if (fw.hasEvent() == 0) {
            continue;
        }
        var event_data = [_]u8{0} ** fw.FW_GET_EVENT_DATA_MAX;
        const last_event = fw.getEventData(&event_data);
        fw.printInt(@ptrCast("\nLast event: %d\n"), fw.printColorNormal, fw.printUInt32, last_event);
        // We only want to process button presses
        var found: bool = false;
        for (Buttons) |button| {
            if (button == last_event) {
                found = true;
                break;
            }
        }
        if (!found) {
            continue;
        }
        for (Panels) |panel| {
            if (panel.event_type == last_event) {
                fw.printInt(@ptrCast("\nShowing Panel %d...\n"), fw.printColorNormal, fw.printUInt32, panel.index);
                fw.showPanel(panel.index);
                // If we match the event type, show the panel, flash leds and transmit the radio
                for (0..NUMBER_OF_LEDS) |led_index| {
                    fw.setBoardLED(@intCast(led_index), panel.color.red, panel.color.green, panel.color.blue, 250, fw.ledpulse);
                }
                _ = fw.RadioTxSubFile(@intCast(1), @ptrCast(panel.sub_fname));
                // Wait for the radio to stop transmitting
                while (fw.RadioSubFileIsTransmitting() != 0) {
                    fw.waitms(33);
                }
                // show the main panel
                fw.showPanel(0);
                break;
            }
        }
        // we need an exit condition
        if (last_event == fw.FWGUI_EVENT_RED_BUTTON) {
            fw.printInt("\nLast event was red button: %d\n", fw.printColorNormal, fw.printUInt32, red_count);
            red_count += 1;
            if (red_count >= 3) {
                _ = fw.RadioTxSubFile(1, @ptrCast("/radio/off.sub"));
                // Wait for the radio to stop transmitting
                while (fw.RadioSubFileIsTransmitting() != 0) {
                    fw.waitms(33);
                }
                fw.exitToMainAppMenu();
                break;
            }
        } else {
            red_count = 0;
        }
    }
}

pub export fn main() void {
    setup_panels();

    show_rainbow_leds(5);

    process_events();
}
