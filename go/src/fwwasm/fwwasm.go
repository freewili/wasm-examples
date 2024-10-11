package fwwasm

import "C"

type FWGUIEvent int

const (
	FWGUI_EVENT_GRAY_BUTTON FWGUIEvent = iota
	FWGUI_EVENT_YELLOW_BUTTON
	FWGUI_EVENT_GREEN_BUTTON
	FWGUI_EVENT_BLUE_BUTTON
	FWGUI_EVENT_RED_BUTTON
	FWGUI_EVENT_IR_CODE
	FWGUI_EVENT_GUI_BUTTON
	FWGUI_EVENT_GUI_NUMEDIT
	FWGUI_EVENT_GUI_TEXTEDIT
	FWGUI_EVENT_GUI_AUDIO_DATA
	FWGUI_EVENT_GUI_FFT_DATA
	FWGUI_EVENT_GUI_I2C_RESPONSE
	FWGUI_EVENT_EVENTFIFO_OVERFLOW
	FWGUI_EVENT_GUI_RTC_RESPONSE
	FWGUI_EVENT_GUI_SENSOR_DATA
	FWGUI_EVENT_MAIN_APP_SEL
	FWGUI_EVENT_PANEL_SHOW
	FWGUI_EVENT_PICKLIST_SEL
	FWGUI_EVENT_PANEL_HIDE
	FWGUI_EVENT_MAIN_MENU_SHOWN
	FWGUI_EVENT_STARTED
	FWGUI_EVENT_CLR_STATS
	FWGUI_EVENT_DIALOG_ACTION
	FWGUI_EVENT_WASM_OVRFLOW

	FWGUI_EVENT_DATA_MAX
)

const FW_GET_EVENT_DATA_MAX = 34

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

type LEDManagerLEDMode C.int

const (
	Ledsimplevalue LEDManagerLEDMode = iota
	Ledflash
	Ledpulse
	Ledflashfade
	Ledpulsefade
)

//go:wasm-module wiliwasm
//export waitms
func Waitms(ms C.int)

//go:wasm-module wiliwasm
//export i2cRead
func I2cRead(address C.int, register C.int, bytes *C.char, length C.int) C.int

//go:wasm-module wiliwasm
//export i2cWrite
func I2cWrite(address C.int, register C.int, bytes *C.char, length C.int) C.int

//go:wasm-module wiliwasm
//export setBoardLED
func setBoardLED(led_index C.int, red C.int, green C.int, blue C.int, duration_ms C.int, mode C.int)

func SetBoardLED(led_index int, red int, green int, blue int, duration_ms int, mode LEDManagerLEDMode) {
	setBoardLED(C.int(led_index), C.int(red), C.int(green), C.int(blue), C.int(duration_ms), C.int(mode))
}

//go:wasm-module wiliwasm
//export RadioTxSubFile
func RadioTxSubFile(index C.int, sub_file *C.char) C.int

//go:wasm-module wiliwasm
//export hasEvent
func HasEvent() C.int

//go:wasm-module wiliwasm
//export hasEvent
func GetEventData(data *C.char) C.int

// void addPanel(int index, int visible, int in_rotation, int use_tile, int tile_id, int bg_red, int bg_green, int bg_blue, int show_menu)
//
//go:wasm-module wiliwasm
//export addPanel
func AddPanel(index C.int, visible C.int, in_rotation C.int, use_tile C.int, tile_id C.int, bg_red C.int, bg_green C.int, bg_blue C.int, show_menu C.int)

// void addControlPictureFromFile(int panel_index, int control_index, int x, int y, const char* file_name, int visible)
//
//go:wasm-module wiliwasm
//export addControlPictureFromFile
func addControlPictureFromFile(panel_index C.int, control_index C.int, x C.int, y C.int, file_name *C.char, visible C.int)

func AddControlPictureFromFile(panel_index int, control_index int, x int, y int, file_name string, visible int) {
	addControlPictureFromFile(C.int(panel_index), C.int(control_index), C.int(x), C.int(y), C.CString(file_name), C.int(visible))
}
// void addControlText(int panel_index,
//
//	int control_index,
//	int x,
//	int y,
//	int font_type,
//	int font_size,
//	int red,
//	int green,
//	int blue,
//	const char* text_value) WASM_IMPORT("addControlText");
//
//go:wasm-module wiliwasm
//export addControlText
func addControlText(panel_index C.int, control_index C.int, x C.int, y C.int, font_type C.int, font_size C.int, red C.int, green C.int, blue C.int, text_value *C.char)

func AddControlText(panel_index int, control_index int, x int, y int, font_type int, font_size int, red int, green int, blue int, text_value string) {
	addControlText(C.int(panel_index), C.int(control_index), C.int(x), C.int(y), C.int(font_type), C.int(font_size), C.int(red), C.int(green), C.int(blue), C.CString(text_value))
}

// void showPanel(int index) WASM_IMPORT("showPanel");
//
//go:wasm-module wiliwasm
//export showPanel
func ShowPanel(index C.int)

// void printInt(const char* szFormatSpec, _printOutColor iColor, _printOutDataType iDataType, int iDataValue)
//
//	WASM_IMPORT("printInt");
//
//go:wasm-module wiliwasm
//export printInt
func PrintInt(szFormatSpec *C.char, iColor C.int, iDataType C.int, iDataValue C.int)

// void printFloat(const char* szFormatSpec, _printOutColor iColor, float fDataItem) WASM_IMPORT("printFloat");
//
//go:wasm-module wiliwasm
//export printFloat
func PrintFloat(szFormatSpec *C.char, iColor C.int, fDataItem C.float)
