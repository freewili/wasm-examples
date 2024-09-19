/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals, unused, dead_code, non_camel_case_types)]

pub const FWGUI_EVENT_NUMTYPE_INT: u32 = 1;
pub const FWGUI_EVENT_NUMTYPE_UINT: u32 = 2;
pub const FWGUI_EVENT_NUMTYPE_FLOAT: u32 = 3;
pub const FW_GET_EVENT_DATA_MAX: u32 = 34;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _FWGuiEventType {
    FWGUI_EVENT_GRAY_BUTTON = 0,
    FWGUI_EVENT_YELLOW_BUTTON = 1,
    FWGUI_EVENT_GREEN_BUTTON = 2,
    FWGUI_EVENT_BLUE_BUTTON = 3,
    FWGUI_EVENT_RED_BUTTON = 4,
    FWGUI_EVENT_IR_CODE = 5,
    FWGUI_EVENT_GUI_BUTTON = 6,
    FWGUI_EVENT_GUI_NUMEDIT = 7,
    FWGUI_EVENT_GUI_TEXTEDIT = 8,
    FWGUI_EVENT_GUI_AUDIO_DATA = 9,
    FWGUI_EVENT_GUI_FFT_DATA = 10,
    FWGUI_EVENT_GUI_I2C_RESPONSE = 11,
    FWGUI_EVENT_EVENTFIFO_OVERFLOW = 12,
    FWGUI_EVENT_GUI_RTC_RESPONSE = 13,
    FWGUI_EVENT_GUI_SENSOR_DATA = 14,
    FWGUI_EVENT_MAIN_APP_SEL = 15,
    FWGUI_EVENT_PANEL_SHOW = 16,
    FWGUI_EVENT_PICKLIST_SEL = 17,
    FWGUI_EVENT_PANEL_HIDE = 18,
    FWGUI_EVENT_MAIN_MENU_SHOWN = 19,
    FWGUI_EVENT_STARTED = 20,
    FWGUI_EVENT_CLR_STATS = 21,
    FWGUI_EVENT_DIALOG_ACTION = 22,
    FWGUI_EVENT_WASM_OVRFLOW = 23,
    FWGUI_EVENT_DATA_MAX = 24,
}
pub use self::_FWGuiEventType as FWGuiEventType;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _LEDManagerLEDMode {
    ledsimplevalue = 0,
    ledflash = 1,
    ledpulse = 2,
    ledflashfade = 3,
    ledpulsefade = 4,
}
pub use self::_LEDManagerLEDMode as LEDManagerLEDMode;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _ePanelColorLED {
    LEDColorRed = 0,
    LEDColorGreen = 1,
    LEDColorYellow = 2,
    LEDColorBlue = 3,
    LEDColorOrange = 4,
    LEDColorAqua = 5,
    LEDColorMagenta = 6,
    LEDColorWhite = 7,
}
pub use self::_ePanelColorLED as ePanelColorLED;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _ePanelSizeLED {
    LEDSize32 = 0,
    LEDSize48 = 1,
    LEDSize64 = 2,
}
pub use self::_ePanelSizeLED as ePanelSizeLED;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _printOutDataType {
    printInt32 = 0,
    printUInt32 = 1,
    printInt16 = 2,
    printUInt16 = 3,
    printUint8 = 4,
    printInt8 = 5,
    printChar = 6,
    printBool = 7,
}
pub use self::_printOutDataType as printOutDataType;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _printOutColor {
    printColorNormal = 0,
    printColorBlack = 1,
    printColorBlue = 2,
    printColorGreen = 3,
    printColorCyan = 4,
    printColorRed = 5,
    printColorPurple = 6,
    printColorBrown = 7,
    printColorYellow = 8,
    printColorWhite = 9,
}
pub use self::_printOutColor as printOutColor;
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief wait for a number of milliseconds\n @param milliseconds the number of milliseconds to wait"]
    pub fn waitms(milliseconds: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the state of a GPIO\n @param io the number of the GPIO\n @param on 1 for on, 0 for off"]
    pub fn setIO(io: ::core::ffi::c_int, on: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief get the state of a GPIO\n @param io the number of the GPIO\n @return 1 for on, 0 for off"]
    pub fn getIO(io: ::core::ffi::c_int) -> ::core::ffi::c_uint;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief get the state of all GPIO\n @return 1 for on, 0 for off for each bit position"]
    pub fn getAllIO() -> ::core::ffi::c_uint;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief read from an I2C device\n @param address the I2C address\n @param reg the I2C register\n @param data pointer to the data to send\n @param length the length of the data to send\n @return 1 on success, 0 on failure"]
    pub fn i2cRead(
        address: ::core::ffi::c_int,
        reg: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief write to an I2C device\n @param address the I2C address\n @param reg the I2C register\n @param data pointer to the data to send\n @param length the length of the data to send\n @return 1 on success, 0 on failure"]
    pub fn i2cWrite(
        address: ::core::ffi::c_int,
        reg: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief read and write data over SPI\n @param data_in pointer to the data to send\n @param length the length of the data to send\n @param data_out pointer to the data to receive\n @return 1 on success, 0 on failure"]
    pub fn SPIReadWrite(
        data_in: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
        data_out: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief Get the number of bytes in the UART receive buffer\n @return the number of bytes"]
    pub fn UARTDataRxCount() -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief read data from the UART\n @param data pointer to the data to read\n @param length the length of the data to read\n @return 1 on success, 0 on failure"]
    pub fn UARTDataRead(
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief write data to the UART\n @param data pointer to the data to write\n @param length the length of the data to write\n @return length on success."]
    pub fn UARTDataWrite(
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the frequency and duty cycle of a PWM pin\n @param io the number of the PWM pin\n @param freq_hz the frequency of the PWM\n @param duty the duty cycle of the PWM\n @return 1 on success, 0 on failure"]
    pub fn PWMSetFreqDuty(io: ::core::ffi::c_int, freq_hz: f32, duty: f32) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief stop PWM on a GPIO pin\n @param io the number of the GPIO pin\n @return 1 on success, 0 on failure"]
    pub fn PWMStop(io: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief write data to a radio\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @param data pointer to the data to send\n @param length the length of the data to send\n @return 1 on success, 0 on failure"]
    pub fn RadioWrite(
        index: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief read data from a radio\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @param data pointer to the data to send\n @param length the length of the data to send\n @return number of bytes read on success, 0 on failure"]
    pub fn RadioRead(
        index: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief get the number of bytes in the radio receive buffer\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return the number of bytes"]
    pub fn RadioGetRxCount(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @todo"]
    pub fn RadioLoadConfig(
        index: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief Transmit a sub file to a radio\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @param sub_file the name of the sub file to transmit\n @return 1 on success, 0 on failure"]
    pub fn RadioTxSubFile(
        index: ::core::ffi::c_int,
        sub_file: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the radio to transmit\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return 1 on success, 0 on failure"]
    pub fn RadioSetTx(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the radio to receive\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return 1 on success, 0 on failure"]
    pub fn RadioSetRx(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the radio to idle\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return 1 on success, 0 on failure"]
    pub fn RadioSetIdle(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief get the RSSI of the radio\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return the RSSI"]
    pub fn RadioGetRSSI(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief get the LQI of the radio\n @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.\n @return the LQI"]
    pub fn RadioGetLQI(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief send IR data\n @param data the IR data"]
    pub fn sendIRData(data: ::core::ffi::c_uint);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the state of the tri-color LEDs\n @param led_index the index of the LED\n @param red the red value\n @param green the green value\n @param blue the blue value\n @param duration_ms the duration of the LED in milliseconds\n @param mode the mode of the LED. see LEDManagerLEDMode enum for more details."]
    pub fn setBoardLED(
        led_index: ::core::ffi::c_int,
        red: ::core::ffi::c_int,
        green: ::core::ffi::c_int,
        blue: ::core::ffi::c_int,
        duration_ms: ::core::ffi::c_int,
        mode: u32,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " @brief set the show mode of the LEDs.\n @param iLEDShow the show mode\n @todo"]
    pub fn setLEDShowMode(iLEDShow: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn playSoundFromFile(szSoundPath: *mut ::core::ffi::c_char);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn playSoundFromNameOrID(szSoundName: *const ::core::ffi::c_char, iID: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn playSoundFromNumber(
        bFloat: ::core::ffi::c_int,
        iNumber: ::core::ffi::c_int,
        fNumber: f32,
        iFloatDigits: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn openFile(
        szFileName: *mut ::core::ffi::c_char,
        iForWriting: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn closeFile(iHandle: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn writeFile(
        iHandle: ::core::ffi::c_int,
        btBytes: *mut ::core::ffi::c_uchar,
        iNumByte: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn preAllocateSpaceForFile(
        iHandle: ::core::ffi::c_int,
        iFileSizeInBytes: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn readFile(
        iHandle: ::core::ffi::c_int,
        btBytes: *mut ::core::ffi::c_uchar,
        iNumBytes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn readFileLine(
        iHandle: ::core::ffi::c_int,
        btBytes: *mut ::core::ffi::c_char,
        iNumBytes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setFilePosition(
        iHandle: ::core::ffi::c_int,
        iPosition: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getFilePosition(iHandle: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getFileSize(iHandle: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn renameFileOrDirectory(
        szDirectoryOrFileName: *const ::core::ffi::c_char,
        szNewDirectoryOrFileName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn fileExists(szFilePath: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn makeDirectory(szFilePath: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn changeDirectory(szFilePath: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getDirectoryItemByIndex(
        szPathDirectory: *const ::core::ffi::c_char,
        szFileName: *mut ::core::ffi::c_char,
        bIncludeExtension: ::core::ffi::c_int,
        iIndex: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getVolumeInfo(piFree: *mut ::core::ffi::c_int, piTotal: *mut ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn removeFileOrDirectory(szFilePath: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getEventData(pData: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn hasEvent() -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addPanel(
        iPanelIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iInRotation: ::core::ffi::c_int,
        iUseTile: ::core::ffi::c_int,
        iTileID: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
        iShowMenu: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addPanelPickList(
        iPanelIndex: ::core::ffi::c_int,
        szCaption: *const ::core::ffi::c_char,
        iTileID: ::core::ffi::c_int,
        iIconID: ::core::ffi::c_int,
        iRBack: ::core::ffi::c_uchar,
        iGBack: ::core::ffi::c_uchar,
        iBBack: ::core::ffi::c_uint,
        iRFore: ::core::ffi::c_uchar,
        iGFore: ::core::ffi::c_uchar,
        iBFore: ::core::ffi::c_uint,
        iLogIndex: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setPanelMenuText(
        iPanel: ::core::ffi::c_int,
        iButtonGreyFromZero: ::core::ffi::c_int,
        szMessage: *const ::core::ffi::c_char,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    #[doc = " controls"]
    pub fn addControlLED(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iColor: u32,
        iSize: u32,
        iIntialState: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setListItemText(
        iLogIndex: ::core::ffi::c_int,
        iListIndex: ::core::ffi::c_int,
        szText: *const ::core::ffi::c_char,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setListItemSelected(iLogIndex: ::core::ffi::c_int, iListIndex: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setListItemTopIndex(iLogIndex: ::core::ffi::c_int, iListIndex: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn clearLogOrPlotData(
        iLogIndexPlusOne: ::core::ffi::c_int,
        iPlotIndexPlusOne: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlLogList(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iLog: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iWidth: ::core::ffi::c_int,
        iHeight: ::core::ffi::c_int,
        iFontType: ::core::ffi::c_int,
        iFontSize: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
        iRFont: ::core::ffi::c_int,
        iGFont: ::core::ffi::c_int,
        iBFont: ::core::ffi::c_int,
        iListMode: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlPlotXAxis(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iScrollMode: ::core::ffi::c_int,
        iTimeMin: ::core::ffi::c_ulonglong,
        iTimeMax: ::core::ffi::c_ulonglong,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlPlotData(
        iPlotDataIndex: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlPlot(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iPlotDataIndexBitField: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iWidth: ::core::ffi::c_int,
        iHeight: ::core::ffi::c_int,
        iMin: ::core::ffi::c_int,
        iMax: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlNumber(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iWidth: ::core::ffi::c_int,
        iFontSize: ::core::ffi::c_int,
        iFontType: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
        iIsFloat: ::core::ffi::c_int,
        iFloatDigits: ::core::ffi::c_int,
        bIsHexFormat: ::core::ffi::c_int,
        bIsUnsigned: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlPicture(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iPictureId: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlText(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iFontType: ::core::ffi::c_int,
        iFontSize: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
        szText: *const ::core::ffi::c_char,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlBargraph(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iWidth: ::core::ffi::c_int,
        iHeight: ::core::ffi::c_int,
        iMin: ::core::ffi::c_int,
        iMax: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlButton(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iVisible: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        iWidth: ::core::ffi::c_int,
        iHeight: ::core::ffi::c_int,
        iR: ::core::ffi::c_int,
        iG: ::core::ffi::c_int,
        iB: ::core::ffi::c_int,
        iRFore: ::core::ffi::c_int,
        iGFore: ::core::ffi::c_int,
        iBFore: ::core::ffi::c_int,
        szText: *const ::core::ffi::c_char,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setControlValueMinMax(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        bEnableMinMax: ::core::ffi::c_int,
        iMin: ::core::ffi::c_int,
        iMax: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setControlValueMinMaxF(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        bEnableMinMax: ::core::ffi::c_int,
        iMinF: f32,
        iMaxF: f32,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setLogDataText(iLogIndex: ::core::ffi::c_int, szText: *const ::core::ffi::c_char);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setPlotData(
        iPlotData: ::core::ffi::c_int,
        iSettings: ::core::ffi::c_int,
        iNewValue: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setControlValue(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iNewValue: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setControlValueFloat(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        fNewValue: f32,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn exitToMainAppMenu();
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showPanel(iPanel: ::core::ffi::c_int);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn addControlPictureFromFile(
        iPanelIndex: ::core::ffi::c_int,
        iControlIndex: ::core::ffi::c_int,
        iX: ::core::ffi::c_int,
        iY: ::core::ffi::c_int,
        szPicture: *const ::core::ffi::c_char,
        iVisible: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn printInt(
        szFormatSpec: *const ::core::ffi::c_char,
        iColor: u32,
        iDataType: u32,
        iDataValue: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn printFloat(szFormatSpec: *const ::core::ffi::c_char, iColor: u32, fDataItem: f32);
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setAudioSettings(
        bStreamMic: ::core::ffi::c_int,
        bStreamFFT: ::core::ffi::c_int,
        bEnableMicPlotData: ::core::ffi::c_int,
        iMICPlotDataIndex: ::core::ffi::c_int,
        bEnableFFTPlotData: ::core::ffi::c_int,
        iFFTPlotDataIndex: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn loadFPGAFromFile(szFilePath: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn runZoomIOScript(szScript: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn getRTC();
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setSensorSettings(
        bStreamAccel: ::core::ffi::c_int,
        bStreamTemp: ::core::ffi::c_int,
        iRateMilliseconds: ::core::ffi::c_int,
        bEnableAccelXPlotData: ::core::ffi::c_int,
        iAccelXPlotDataIndex: ::core::ffi::c_int,
        bEnableAccelYPlotData: ::core::ffi::c_int,
        iAccelYPlotDataIndex: ::core::ffi::c_int,
        bEnableAccelZPlotData: ::core::ffi::c_int,
        iAccelZPlotDataIndex: ::core::ffi::c_int,
        bEnableTempPlotData: ::core::ffi::c_int,
        iTempPlotDataIndex: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn setAppLogSettings(
        bLogIRCodes: ::core::ffi::c_int,
        bLogAccel: ::core::ffi::c_int,
        bLogTempC: ::core::ffi::c_int,
        bLogTempF: ::core::ffi::c_int,
        iLogIndex: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogMsgBox(
        szMessage: *mut ::core::ffi::c_char,
        bShowOk: ::core::ffi::c_int,
        bShowOkCancel: ::core::ffi::c_int,
        bShowYesNo: ::core::ffi::c_int,
        iPictureIndex: ::core::ffi::c_int,
        iAutoCloseHalfSec: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogProgressBar(
        szMessage: *mut ::core::ffi::c_char,
        iPictureIndex: ::core::ffi::c_int,
        iValue0to100: ::core::ffi::c_int,
        bShowOkToClose: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogNumEdit(
        szMessage: *mut ::core::ffi::c_char,
        bUnsigned: ::core::ffi::c_int,
        bHexFormat: ::core::ffi::c_int,
        bUseMinMax: ::core::ffi::c_int,
        iInitalValue: ::core::ffi::c_int,
        iMin: ::core::ffi::c_int,
        iMax: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogNumEditFloat(
        szMessage: *mut ::core::ffi::c_char,
        iFoatDigits: ::core::ffi::c_int,
        bUseMinMax: ::core::ffi::c_int,
        fInitalValue: f32,
        fMin: ::core::ffi::c_int,
        fMax: ::core::ffi::c_int,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogTextEdit(
        szMessage: *mut ::core::ffi::c_char,
        szInitialValue: *mut ::core::ffi::c_char,
    );
}
#[link(wasm_import_module = "wiliwasm")]
extern "C" {
    pub fn showDialogPickList(
        szMessage: *mut ::core::ffi::c_char,
        iLogIndex: ::core::ffi::c_int,
        iDefaultItem: ::core::ffi::c_int,
    );
}
