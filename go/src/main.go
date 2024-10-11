// tinygo build -target wasip1 -panic=trap -scheduler=none -gc=leaking -opt=z -stack-size=61440B -size=short -no-debug
package main

import "C"

import (
	"main/fwwasm"
)


func SetupPanels() {
	fwwasm.AddPanel(0, 1, 0, 0, 0, 0, 0, 0, 1)
	//fwwasm.AddControlPictureFromFile(0, 0, 0, 0, "pip_boy.fwi", 1);
	//fwwasm.AddControlText(0, 1, 90, 180, 1, 64, 
	//	fwwasm.ColorWhite().Red, fwwasm.ColorWhite().Green, fwwasm.ColorWhite().Blue, "Press a Button");
	fwwasm.ShowPanel(0)


}

func main() {
	SetupPanels()

	for ledIndex := range 7 {
		fwwasm.Waitms(15)

		fwwasm.SetBoardLED(ledIndex, 
			fwwasm.ColorGreen().Red/8, 
			fwwasm.ColorGreen().Green/8, 
			fwwasm.ColorGreen().Blue/8, 
			500,
			fwwasm.LEDManagerLEDMode(fwwasm.Ledflashfade),
		);
	}
}
