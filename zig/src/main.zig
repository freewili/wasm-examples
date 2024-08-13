// zig build-exe -target wasm32-wasi src/main.zig -fno-entry --export=main --stack 131056 -OReleaseSmall
// This doesn't seem to work correctly. for some reason the zig compiler isn't picking up __attribute__((import_module("wiliwasm"))) __attribute__((import_name(NAME)))
// const fw = @cImport({
//     @cInclude("fwwasm.h");
// });

extern "wiliwasm" fn setled(on: c_int) void;
extern "wiliwasm" fn waitms(ms: c_int) void;
extern "wiliwasm" fn i2cRead(address: c_int, register: c_int, bytes: *c_char, length: c_int) c_int;
extern "wiliwasm" fn i2cWrite(address: c_int, register: c_int, bytes: *c_char, length: c_int) c_int;
extern "wiliwasm" fn addPanel(iPanelIndex: c_int, iVisible: c_int, iVisible: c_int, iUseTile: c_int, iTileID: c_int, iR: u8, iG: u8, iB: u8, iShowMenu: c_int) void;
extern "wiliwasm" fn addControlPictureFromFile(iPanelIndex: c_int, iControlIndex: c_int, iX: c_int, iY: c_int, szPicture: [*c]const u8, iVisible: c_int) void;
extern "wiliwasm" fn showPanel(iPanel: c_int) void;
extern "wiliwasm" fn setBoardLED(iLEDIndex0_7: c_uint, iR: c_int, iG: c_int, iB: c_int, iDurationMs: c_int, iMode: c_int) void;

const LEDManagerLEDMode = enum(c_int) { ledsimplevalue, ledflash, ledpulse, ledflashfade, ledpulsefade };

// different color RGB values
const RED: i32 = 0xFF0000;
const PINK: i32 = 0xFFC6FF;
const ORANGE: i32 = 0xFF7F00;
const YELLOW: i32 = 0xFFFF00;
const GREEN: i32 = 0x00FF00;
const LIGHT_GREEN: i32 = 0xCAFFBF;
const BLUE: i32 = 0x0000FF;
const LIGHT_BLUE: i32 = 0x9BF6FF;
const INDIGO: i32 = 0x4B0082;
const VIOLET: i32 = 0x9400D3;

const Colors = [_]i32{ RED, PINK, YELLOW, GREEN, LIGHT_GREEN, BLUE, LIGHT_BLUE, INDIGO, VIOLET };

pub fn display_image() void {
    // Create a panel to display the image
    addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    addControlPictureFromFile(0, 0, 0, 0, @ptrCast("pip_boy.fwi"), 1);
    // Show the Panel
    showPanel(0);
}

pub export fn main() void {
    const MAX_LOOPS: i32 = 20;
    const NUM_LEDS: i32 = 7;
    const DELAY_MS: i32 = 50;
    const LED_FADE_DURATION: i32 = 300;

    display_image();

    var color_choice: u32 = 0;
    var led_on: bool = false;
    for (0..MAX_LOOPS) |_| {
        for (0..NUM_LEDS) |led_index| {
            // Pick the color and set it
            const color: i32 = Colors[color_choice];
            setBoardLED(led_index, (color & 0xFF0000) >> 16, (color & 0x00FF00) >> 8, (color & 0x0000FF) >> 0, LED_FADE_DURATION, @intFromEnum(LEDManagerLEDMode.ledpulsefade));
            // Make sure we don't overflow the array
            if (color_choice >= Colors.len - 1) {
                color_choice = 0;
            } else {
                color_choice += 1;
            }
            // Lets blink the led on the PCB also
            setled(@intFromBool(led_on));
            led_on = !led_on;

            waitms(DELAY_MS);
        }
    }
}
