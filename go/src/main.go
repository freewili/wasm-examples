// tinygo build -target wasip1 -panic=trap -scheduler=none -gc=leaking -opt=z -stack-size=61440B -size=short -no-debug -print-allocs=.
package main

// #include <stdio.h>
// #include <stdlib.h>
// #include <errno.h>
// #include "../../fwwasm/include/fwwasm.h"
import "C"
import "unsafe"


func SetupPanels() {
	C.addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1)
	fname := C.CString("pip_boy.fwi")
	C.addControlPictureFromFile(0, 0, 0, 0, fname, 1);
	defer C.free(unsafe.Pointer(fname))
	//fwwasm.AddControlText(0, 1, 90, 180, 1, 64, 
	//	fwwasm.ColorWhite().Red, fwwasm.ColorWhite().Green, fwwasm.ColorWhite().Blue, "Press a Button");
	C.showPanel(0)
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

func ColorGreen() Color {
	return Color{0, 255, 0}
}	

func ColorBlue() Color {
	return Color{0, 0, 255}
}

func ColorWhite() Color {
	return Color{255, 255, 255}
}

func ColorLightGreen() Color {
	return Color{0, 255, 191}
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