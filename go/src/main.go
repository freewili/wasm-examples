// tinygo build -o fw_go.wasm -target wasi -no-debug -panic=trap -scheduler=none -gc=leaking -opt=z main.go
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
	
	for _ = range 7 {
		fwwasm.Waitms(100)

		fwwasm.SetBoardLED(0, 
			fwwasm.ColorRed().Red, 
			fwwasm.ColorRed().Green, 
			fwwasm.ColorRed().Blue, 
			300,
			fwwasm.LEDManagerLEDMode(fwwasm.Ledflashfade),
		);
	}
}
