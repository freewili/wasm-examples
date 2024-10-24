const fw = @cImport({
    @cInclude("fwwasm.h");
});

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
    fw.addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    fw.addControlPictureFromFile(0, 0, 0, 0, @ptrCast("pip_boy.fwi"), 1);
    // Show the Panel
    fw.showPanel(0);
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
            fw.setBoardLED(@intCast(led_index), (color & 0xFF0000) >> 16, (color & 0x00FF00) >> 8, (color & 0x0000FF) >> 0, LED_FADE_DURATION, fw.ledpulsefade);
            // Make sure we don't overflow the array
            if (color_choice >= Colors.len - 1) {
                color_choice = 0;
            } else {
                color_choice += 1;
            }
            // Lets blink the led on the PCB also
            //fw.setled(@intFromBool(led_on));
            led_on = !led_on;

            fw.waitms(DELAY_MS);
        }
    }
}
