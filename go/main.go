// tinygo build -o fw_go.wasm -target wasi -no-debug -panic=trap -scheduler=none -gc=leaking -opt=z main.go
package main

import "C"

//go:wasm-module wiliwasm
//export setled
func setled(on C.int)

//go:wasm-module wiliwasm
//export waitms
func waitms(ms C.int)

//go:wasm-module wiliwasm
//export i2cRead
func i2cRead(address C.int, register C.int, bytes *C.char, length C.int) C.int

//go:wasm-module wiliwasm
//export i2cWrite
func i2cWrite(address C.int, register C.int, bytes *C.char, length C.int) C.int

//go:wasm-module wiliwasm
//export addPanel
func addPanel(iPanelIndex C.int, iVisible C.int, iInRotation C.int, iUseTile C.int, iTileID C.int, iR uint8, iG uint8, iB uint8, iShowMenu C.int)

//go:wasm-module wiliwasm
//export addControlPictureFromFile
func addControlPictureFromFile(iPanelIndex C.int, iControlIndex C.int, iX C.int, iY C.int, szPicture *C.char, iVisible C.int)

//go:wasm-module wiliwasm
//export showPanel
func showPanel(iPanel C.int)

//go:wasm-module wiliwasm
//export setBoardLED
func setBoardLED(iLEDIndex0_7 C.int, iR C.int, iG C.int, iB C.int, iDurationMs C.int, iMode C.int)

const (
	LEDManagerLEDMode_ledsimplevalue = iota
	LEDManagerLEDMode_ledflash       = iota
	LEDManagerLEDMode_ledpulse       = iota
	LEDManagerLEDMode_ledflashfade   = iota
	LEDManagerLEDMode_ledpulsefade   = iota
)

// different color RGB values
const RED = 0xFF0000
const PINK = 0xFFC6FF
const ORANGE = 0xFF7F00
const YELLOW = 0xFFFF00
const GREEN = 0x00FF00
const LIGHT_GREEN = 0xCAFFBF
const BLUE = 0x0000FF
const LIGHT_BLUE = 0x9BF6FF
const INDIGO = 0x4B0082
const VIOLET = 0x9400D3

// Go arrays are mutable, can't make it const
var Colors = [...]uint32{RED, PINK, YELLOW, GREEN, LIGHT_GREEN, BLUE, LIGHT_BLUE, INDIGO, VIOLET}

func display_image() {
	// Create a panel to display the image
	addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1)
	// Add the image to the panel, this file should be in the images directory on the display processor
	addControlPictureFromFile(0, 0, 0, 0, C.CString("pip_boy.fwi"), 1)
	// Show the Panel
	showPanel(0)
}
func main() {
	const MAX_LOOPS = 20
	const NUM_LEDS = 7
	const DELAY_MS = 50
	const LED_FADE_DURATION = 300

	display_image()

	color_choice := 0
	led_on := false
	//led_index = 0
	for _ = range MAX_LOOPS {
		for led_index := range NUM_LEDS {
			// Pick the color and set it
			var color = Colors[color_choice]
			setBoardLED(C.int(led_index),
				C.int((color&0xFF0000)>>16),
				C.int((color&0x00FF00)>>8),
				C.int((color&0x0000FF)>>0),
				LED_FADE_DURATION,
				LEDManagerLEDMode_ledflashfade)
			// Make sure we don't overflow the array
			if color_choice >= len(Colors)-1 {
				color_choice = 0
			} else {
				color_choice += 1
			}
			// Lets blink the led on the PCB also
			if led_on {
				setled(1)
			} else {
				setled(0)
			}
			led_on = !led_on

			waitms(DELAY_MS)
		}
	}
}
