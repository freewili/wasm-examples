// tinygo build -target ./target.json -panic=trap -opt=z -size=short -no-debug -print-allocs=.
package main

// #include <stdio.h>
// #include <stdlib.h>
// #include <errno.h>
// #include "../../fwwasm/include/fwwasm.h"
import "C"
import "unsafe"

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
	defer C.free(unsafe.Pointer(fname))
	title := C.CString("Press a Button")
	C.addControlText(0, 1, 90, 180, 1, 64, C.int(ColorWhite().Red), C.int(ColorWhite().Green), C.int(ColorWhite().Blue), title);
	defer C.free(unsafe.Pointer(title))
	C.showPanel(0)
	// Setup the rest of the panels
	for _, panel := range Panels() {
		C.addPanel(C.int(panel.index), 1, 0, 0, 0, C.int(panel.color.Red), C.int(panel.color.Green), C.int(panel.color.Blue), 1);
		text := C.CString(panel.text)
		C.addControlText(C.int(panel.index), 1, 10, 50, 2, 0, 0, 0, 0, text);
		defer C.free(unsafe.Pointer(text))
	}
}

func main() {
	SetupPanels()

	for ledIndex := range 7 {
		C.waitms(15)

		C.setBoardLED(C.int(ledIndex), 
			C.int(ColorGreen().Red/8), 
			C.int(ColorGreen().Green/8), 
			C.int(ColorGreen().Blue/8), 
			300,
			C.ledflashfade,
		);
	}
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
