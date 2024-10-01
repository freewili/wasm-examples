#include <fwwasm.h>

#define MAX_LOOPS 20
#define NUM_LEDS 7
#define DELAY_MS 50
#define LED_FADE_DURATION 300

// different color RGB values
#define RED 0xFF0000
#define PINK 0xFFC6FF
#define ORANGE 0xFF7F00
#define YELLOW 0xFFFF00
#define GREEN 0x00FF00
#define LIGHT_GREEN 0xCAFFBF
#define BLUE 0x0000FF
#define LIGHT_BLUE 0x9BF6FF
#define INDIGO 0x4B0082
#define VIOLET 0x9400D3
#define MAX_COLORS 10

// some macros to get color RGB components
#define GET_RED(x) ((x >> 16) & 0xFF)
#define GET_GREEN(x) ((x >> 8) & 0xFF)
#define GET_BLUE(x) (x & 0xFF)

void display_image() {
    // Create a panel to display the image
    addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    addControlPictureFromFile(0, 0, 0, 0, "pip_boy.fwi", 1);
    // Show the Panel
    showPanel(0);
}

int main() {
    int rainbow[MAX_COLORS] = {RED, ORANGE, YELLOW, GREEN, LIGHT_GREEN, BLUE, LIGHT_BLUE, INDIGO, VIOLET, PINK};
    int color_choice = 0;

    display_image();

    // do the whole thing multiple times
    for (int loops = 0; loops < MAX_LOOPS; loops++) {
        // set every LED one at a time
        for (int led = 0; led < NUM_LEDS; led++) {
            // pick a color
            int color = rainbow[color_choice];

            // set the LED
            setBoardLED(led, GET_RED(color), GET_GREEN(color), GET_BLUE(color), LED_FADE_DURATION,
                        LEDManagerLEDMode::ledpulsefade);

            // next time, get a new color.  If we used all of the colors, start over
            color_choice++;
            if (color_choice >= MAX_COLORS) color_choice = 0;

            // wait before setting the next LED
            waitms(DELAY_MS);
        }
    }

    return 0;
}
