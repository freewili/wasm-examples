// tinygo build -o fw_go.wasm -target wasi -no-debug -panic=trap -scheduler=none -gc=leaking -opt=z main.go
package main

import "C"
import "unsafe"

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

func main() {
	led_on := false
	for _ = range 25 {
		data_bytes := [...]C.uchar{0x55, 0x22, 0x11, 0x82}
		_ = i2cWrite(0x23, 11, (*C.char)(unsafe.Pointer(&data_bytes[0])), 4)
		// TODO: check result
		waitms(100)
		if (led_on) {
			setled(1)
		} else {
			setled(0)
		}
		led_on = !led_on
	}
}
