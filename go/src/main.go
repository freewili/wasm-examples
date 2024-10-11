// tinygo build -target wasip1 -panic=trap -scheduler=none -gc=leaking -opt=z -stack-size=61440B -size=short -no-debug
package main

import "C"

import (
	"main/fwwasm"
)


func SetupPanels() {
	fwwasm.AddPanel(0, 1, 0, 0, 0, 0, 0, 0, 1)


}

func main() {
	SetupPanels()

	for ledIndex := range 7 {
		fwwasm.Waitms(100)

		fwwasm.SetBoardLED(ledIndex, 
			fwwasm.ColorRed().Red, 
			fwwasm.ColorRed().Green, 
			fwwasm.ColorRed().Blue, 
			300,
			fwwasm.LEDManagerLEDMode(fwwasm.Ledflashfade),
		);
	}
}
