// tinygo build -target ./target.json -panic=trap -opt=z -size=short -no-debug -print-allocs=.
package main

// #include <stdio.h>
// #include <stdlib.h>
// #include <errno.h>
// #include "../fwwasm/include/fwwasm.h"
import "C"
//import "unsafe"
import "slices"

const (
	NumberOfLeds = 7
)

type PanelInfo struct {
	index int
	event_type int
	color Color
	text string
	sub_fname string
}

func Panels() ([]PanelInfo) {
	return []PanelInfo{
		PanelInfo{1, C.FWGUI_EVENT_GRAY_BUTTON, ColorGray(), "GRAY", "/radio/white.sub"},
		PanelInfo{2, C.FWGUI_EVENT_YELLOW_BUTTON, ColorYellow(), "YELLOW", "/radio/yellow.sub"},
		PanelInfo{3, C.FWGUI_EVENT_GREEN_BUTTON, ColorGreen(), "GREEN", "/radio/green.sub"},
		PanelInfo{4, C.FWGUI_EVENT_BLUE_BUTTON, ColorBlue(), "BLUE", "/radio/blue.sub"},
		PanelInfo{5, C.FWGUI_EVENT_RED_BUTTON, ColorRed(), "RED", "/radio/red.sub"},
	}
}

func Buttons() ([]int) {
	return []int {
		C.FWGUI_EVENT_GRAY_BUTTON,
		C.FWGUI_EVENT_YELLOW_BUTTON,
		C.FWGUI_EVENT_GREEN_BUTTON,
		C.FWGUI_EVENT_BLUE_BUTTON,
		C.FWGUI_EVENT_RED_BUTTON,
	}
}


func SetupPanels() {
	// Setup the main panel that shows pip boy
	C.addPanel(0, 1, 0, 0, 0, 0, 0, 0, 0)
	fname := C.CString("pip_boy.fwi")
	C.addControlPictureFromFile(0, 0, 0, 0, fname, 1);
	// defer C.free(unsafe.Pointer(fname))
	// title := C.CString("Press a Button")
	// C.addControlText(0, 1, 90, 180, 1, 64, C.int(ColorWhite().Red), C.int(ColorWhite().Green), C.int(ColorWhite().Blue), title);
	// defer C.free(unsafe.Pointer(title))
	C.showPanel(0)
	// Setup the rest of the panels
	for _, panel := range Panels() {
		C.addPanel(C.int(panel.index), 1, 0, 0, 0, C.int(panel.color.Red), C.int(panel.color.Green), C.int(panel.color.Blue), 0);
		text := C.CString(panel.text)
		C.addControlText(C.int(panel.index), 1, 10, 50, 2, 0, 0, 0, 0, text);
		// defer C.free(unsafe.Pointer(text))
	}
}

func ShowRainbowLeds(max_loops int) {
	colors := []Color{
		ColorRed(),
		ColorOrange(),
		ColorYellow(),
		ColorGreen(),
		ColorLightGreen(),
		ColorBlue(),
		ColorLightBlue(),
		ColorIndigo(),
		ColorViolet(),
		ColorPink(),
	};
	color_choice := 0
	// do the whole thing multiple times
	for loops := 0; loops < max_loops; loops++ {
		// set every LED one at a time
		for led := 0; led < NumberOfLeds; led++ {
			// Pick a color
			color := &colors[color_choice]
			// Set the LED
			C.setBoardLED(C.int(led),
				C.int(color.Red),
				C.int(color.Green),	
				C.int(color.Blue),
				300,
				C.ledpulsefade,
			);
			// next time, get a new color. If we used all of the colors, start over
			color_choice = (color_choice + 1) % len(colors);
			// wait before setting the next LED
			C.waitms(50);
		}
	}
}

func ProcessEvents() {
	// msg := C.CString("\nListening to events...\n")
	red_count := 0
	// C.printInt(msg, C.printColorNormal, C.printUInt32, 0)
	// defer C.free(unsafe.Pointer(msg))
	for {
		C.waitms(33);
		if C.hasEvent() == 0 {
			continue
		}
		event_data := [C.FW_GET_EVENT_DATA_MAX]C.uchar{0}
		last_event := C.getEventData(&event_data[0])
		// We only want to process button presses
		if !slices.ContainsFunc(Buttons(), func(button int) bool { return button == int(last_event) }) {
			continue
		}
		// Lets match up the button to the panel info and do everything
		for _, panel := range Panels() {
			// If we match the event type, show the panel, flash leds and transmit the radio
			if panel.event_type == int(last_event) {
				C.showPanel(C.int(panel.index))
				for led_index := 0; led_index < NumberOfLeds; led_index++ {
					C.setBoardLED(C.int(led_index),
						C.int(panel.color.Red),
						C.int(panel.color.Green),
						C.int(panel.color.Blue),
						300,
						C.ledpulse,
					)
				}
				sub_fname := C.CString(panel.sub_fname)
				C.RadioTxSubFile(1, sub_fname)
				// defer C.free(unsafe.Pointer(sub_fname))
				// Wait for the radio to stop transmitting
				for {
					if C.RadioSubFileIsTransmitting() == 0 {
						break
					}
					C.waitms(33)
				}
				// Show the main panel
				C.showPanel(0)
				break;
			}	
		}
		// We need an exit condition
		if last_event != C.FWGUI_EVENT_RED_BUTTON {
			red_count = 0
			continue
		}
		// msg := C.CString("\nLast event was red button: %d\n")
		// C.printInt(msg, C.printColorNormal, C.printUInt32, C.int(red_count))
		// defer C.free(unsafe.Pointer(msg))
		red_count += 1
		if red_count >= 3 {
			sub_fname := C.CString("/radio/off.sub")
			C.RadioTxSubFile(1, sub_fname)
			// defer C.free(unsafe.Pointer(sub_fname))
			// Wait for the radio to stop transmitting
			for {
				if C.RadioSubFileIsTransmitting() == 0 {
					break
				}
				C.waitms(33)
			}
			C.exitToMainAppMenu()
			break
		}
			
	}
}

func main() {
	SetupPanels()

	ShowRainbowLeds(5)

	ProcessEvents()
}


type Color struct {
	Red   int
	Green int
	Blue  int
}

func ColorRed() Color {
	return Color{255, 0, 0}
}

func ColorOrange() Color {
	return Color{255, 127, 0}
}

func ColorYellow() Color {
	return Color{255, 255, 0}
}

func ColorGreen() Color {
	return Color{0, 255, 0}
}
func ColorLightGreen() Color {
	return Color{0, 255, 191}
}

func ColorBlue() Color {
	return Color{0, 0, 255}
}

func ColorLightBlue() Color {
	return Color{0, 191, 255}
}

func ColorIndigo() Color {
	return Color{75, 0, 130}
}

func ColorViolet() Color {
	return Color{238, 130, 238}
}

func ColorPink() Color {
	return Color{255, 192, 203}
}

func ColorGray() Color {
	return Color{60, 60, 60}
}

func ColorWhite() Color {
	return Color{255, 255, 255}
}
