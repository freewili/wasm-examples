#pragma once

#define WASM_IMPORT(NAME)                                                      \
  __attribute__((import_module("wiliwasm"))) __attribute__((import_name(NAME)))
#define WASM_EXPORT                                                            \
  extern "C" __attribute__((used)) __attribute__((visibility("default")))
#define WASM_EXPORT_AS(NAME) WASM_EXPORT __attribute__((export_name(NAME)))

#ifdef __cplusplus
extern "C" {
#endif

typedef enum _FWGuiEventType {
  FWGUI_EVENT_GRAY_BUTTON,
  FWGUI_EVENT_YELLOW_BUTTON,
  FWGUI_EVENT_GREEN_BUTTON,
  FWGUI_EVENT_BLUE_BUTTON,
  FWGUI_EVENT_RED_BUTTON,
  FWGUI_EVENT_IR_CODE,
  FWGUI_EVENT_GUI_BUTTON,
  FWGUI_EVENT_GUI_NUMEDIT,
  FWGUI_EVENT_GUI_TEXTEDIT,
  FWGUI_EVENT_GUI_AUDIO_DATA,
  FWGUI_EVENT_GUI_FFT_DATA,
  FWGUI_EVENT_GUI_I2C_RESPONSE,
  FWGUI_EVENT_EVENTFIFO_OVERFLOW,
  FWGUI_EVENT_GUI_RTC_RESPONSE,
  FWGUI_EVENT_GUI_SENSOR_DATA,
  FWGUI_EVENT_MAIN_APP_SEL,
  FWGUI_EVENT_PANEL_SHOW,
  FWGUI_EVENT_PICKLIST_SEL,
  FWGUI_EVENT_PANEL_HIDE,
  FWGUI_EVENT_MAIN_MENU_SHOWN,
  FWGUI_EVENT_STARTED,
  FWGUI_EVENT_CLR_STATS,
  FWGUI_EVENT_DIALOG_ACTION,
  FWGUI_EVENT_WASM_OVRFLOW,

  FWGUI_EVENT_DATA_MAX,
} FWGuiEventType;

#define FWGUI_EVENT_NUMTYPE_INT 1
#define FWGUI_EVENT_NUMTYPE_UINT 2
#define FWGUI_EVENT_NUMTYPE_FLOAT 3

// Maximum amount of data for an event poll
#define FW_GET_EVENT_DATA_MAX 34

typedef enum _LEDManagerLEDMode {
  ledsimplevalue,
  ledflash,
  ledpulse,
  ledflashfade,
  ledpulsefade,
} LEDManagerLEDMode;

typedef enum _ePanelColorLED {
  LEDColorRed,
  LEDColorGreen,
  LEDColorYellow,
  LEDColorBlue,
  LEDColorOrange,
  LEDColorAqua,
  LEDColorMagenta,
  LEDColorWhite,
} ePanelColorLED;

typedef enum _ePanelSizeLED {
  LEDSize32,
  LEDSize48,
  LEDSize64,
} ePanelSizeLED;

typedef enum _printOutDataType {
  printInt32 = 0,
  printUInt32,
  printInt16,
  printUInt16,
  printUint8,
  printInt8,
  printChar,
  printBool,
} printOutDataType;

typedef enum _printOutColor {
  printColorNormal = 0,
  printColorBlack,
  printColorBlue,
  printColorGreen,
  printColorCyan,
  printColorRed,
  printColorPurple,
  printColorBrown,
  printColorYellow,
  printColorWhite,
} printOutColor;

// ===============================================================================
// General
// ===============================================================================

/**
 * @brief wait for a number of milliseconds
 * @param milliseconds the number of milliseconds to wait
 */
void waitms(int milliseconds) WASM_IMPORT("waitms");

// ===============================================================================
// GPIO
// ===============================================================================
/**
 * @brief set the state of a GPIO
 * @param io the number of the GPIO
 * @param on 1 for on, 0 for off
 */
void setIO(int io, int on) WASM_IMPORT("setIO");
/**
 * @brief get the state of a GPIO
 * @param io the number of the GPIO
 * @return 1 for on, 0 for off
 */
unsigned int getIO(int io) WASM_IMPORT("getIO");
/**
 * @brief get the state of all GPIO
 * @return 1 for on, 0 for off for each bit position
 */
unsigned int getAllIO() WASM_IMPORT("getAllIO");

// ===============================================================================
// I2C
// ===============================================================================
/**
 * @brief read from an I2C device
 * @param address the I2C address
 * @param reg the I2C register
 * @param data pointer to the data to send
 * @param length the length of the data to send
 * @return 1 on success, 0 on failure
 */
int i2cRead(int address, int reg, unsigned char *data, int length)
    WASM_IMPORT("i2cRead");

/**
 * @brief write to an I2C device
 * @param address the I2C address
 * @param reg the I2C register
 * @param data pointer to the data to send
 * @param length the length of the data to send
 * @return 1 on success, 0 on failure
 */
int i2cWrite(int address, int reg, unsigned char *data, int length)
    WASM_IMPORT("i2cWrite");

// ===============================================================================
// SPI
// ===============================================================================
/**
 * @brief read and write data over SPI
 * @param data_in pointer to the data to send
 * @param length the length of the data to send
 * @param data_out pointer to the data to receive
 * @return 1 on success, 0 on failure
 */
int SPIReadWrite(unsigned char *data_in, int length, unsigned char *data_out)
    WASM_IMPORT("SPIReadWrite");

// ===============================================================================
// UART
// ===============================================================================
/**
 * @brief Get the number of bytes in the UART receive buffer
 * @return the number of bytes
 */
int UARTDataRxCount() WASM_IMPORT("UARTDataRxCount");
/**
 * @brief read data from the UART
 * @param data pointer to the data to read
 * @param length the length of the data to read
 * @return 1 on success, 0 on failure
 */
int UARTDataRead(unsigned char *data, int length)
    WASM_IMPORT("UARTDataRead");
/**
 * @brief write data to the UART
 * @param data pointer to the data to write
 * @param length the length of the data to write
 * @return length on success.
 */
int UARTDataWrite(unsigned char *data, int length)
    WASM_IMPORT("UARTDataWrite");

// ===============================================================================
// PWM
// ===============================================================================
/**
 * @brief set the frequency and duty cycle of a PWM pin
 * @param io the number of the PWM pin
 * @param freq_hz the frequency of the PWM
 * @param duty the duty cycle of the PWM
 * @return 1 on success, 0 on failure
 */
int PWMSetFreqDuty(int io, float freq_hz, float duty)
    WASM_IMPORT("PWMSetFreqDuty");
/**
 * @brief stop PWM on a GPIO pin
 * @param io the number of the GPIO pin
 * @return 1 on success, 0 on failure
 */
int PWMStop(int io) WASM_IMPORT("PWMStop");

// ===============================================================================
// RADIO
// ===============================================================================
/**
 * @brief write data to a radio
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @param data pointer to the data to send
 * @param length the length of the data to send
 * @return 1 on success, 0 on failure
 */
int RadioWrite(int index, unsigned char *data, int length)
    WASM_IMPORT("RadioWrite");
/**
 * @brief read data from a radio
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @param data pointer to the data to send
 * @param length the length of the data to send
 * @return number of bytes read on success, 0 on failure
 */
int RadioRead(int index, unsigned char *data, int length)
    WASM_IMPORT("RadioRead");
/**
 * @brief get the number of bytes in the radio receive buffer
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return the number of bytes
 */
int RadioGetRxCount(int index) WASM_IMPORT("RadioGetRxCount");
/**
 * @todo
 */
int RadioLoadConfig(int index, unsigned char *data, int length)
    WASM_IMPORT("RadioLoadConfig");
/**
 * @brief Transmit a sub file to a radio
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @param sub_file the name of the sub file to transmit
 * @return 1 on success, 0 on failure
 */
int RadioTxSubFile(int index, char *sub_file)
    WASM_IMPORT("RadioTxSubFile");
/**
 * @brief set the radio to transmit
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return 1 on success, 0 on failure
 */
int RadioSetTx(int index) WASM_IMPORT("RadioSetTx");
/**
 * @brief set the radio to receive
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return 1 on success, 0 on failure
 */
int RadioSetRx(int index) WASM_IMPORT("RadioSetRx");
/**
 * @brief set the radio to idle
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return 1 on success, 0 on failure
 */
int RadioSetIdle(int index) WASM_IMPORT("RadioSetIdle");

/**
 * @brief get the RSSI of the radio
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return the RSSI
 */
int RadioGetRSSI(int index) WASM_IMPORT("RadioGetRSSI");
/**
 * @brief get the LQI of the radio
 * @param index the index of the radio. 1 for Radio 1, 2 for Radio 2.
 * @return the LQI
 */
int RadioGetLQI(int index) WASM_IMPORT("RadioGetLQI");

// ===============================================================================
// IR
// ===============================================================================
/**
 * @brief send IR data
 * @param data the IR data
 */
void sendIRData(unsigned int data) WASM_IMPORT("sendIRData");

// ===============================================================================
// LEDs
// ===============================================================================
/**
 * @brief set the state of the tri-color LEDs
 * @param led_index the index of the LED
 * @param red the red value
 * @param green the green value
 * @param blue the blue value
 * @param duration_ms the duration of the LED in milliseconds
 * @param mode the mode of the LED. see LEDManagerLEDMode enum for more details.
 */
void setBoardLED(int led_index, int red, int green, int blue, int duration_ms,
                   enum LEDManagerLEDMode mode) WASM_IMPORT("setBoardLED");
/** @brief set the show mode of the LEDs.
 * @param iLEDShow the show mode
 * @todo
 */
void setLEDShowMode(int iLEDShow) WASM_IMPORT("setLEDShowMode");

// ===============================================================================
// Sound
// ===============================================================================
void playSoundFromFile(char *szSoundPath)
    WASM_IMPORT("playSoundFromFile"); // implemented
void playSoundFromNameOrID(const char *szSoundName, int iID)
    WASM_IMPORT("playSoundFromNameOrID"); // tested
void playSoundFromNumber(int bFloat, int iNumber, float fNumber,
                         int iFloatDigits)
    WASM_IMPORT("playSoundFromNumber"); // tested

// ===============================================================================
// File IO
// ===============================================================================
int openFile(char *szFileName, int iForWriting)
    WASM_IMPORT("OpenFile");                         // implemented
int closeFile(int iHandle) WASM_IMPORT("closeFile"); // implemented
int writeFile(int iHandle, unsigned char *btBytes, int iNumByte)
    WASM_IMPORT("writeFile"); // implemented
int preAllocateSpaceForFile(int iHandle, int iFileSizeInBytes)
    WASM_IMPORT("preAllocateSpaceForFile"); // implemented
int readFile(int iHandle, unsigned char *btBytes, int *iNumBytes)
    WASM_IMPORT("readFile"); // implemented
int readFileLine(int iHandle, char *btBytes, int *iNumBytes)
    WASM_IMPORT("readFileLine"); // implemented
int setFilePosition(int iHandle, int iPosition)
    WASM_IMPORT("setFilePosition");                              // implemented
int getFilePosition(int iHandle) WASM_IMPORT("getFilePosition"); // implemented
int getFileSize(int iHandle) WASM_IMPORT("getFileSize");         // implemented

// ===============================================================================
// File System
// ===============================================================================
int renameFileOrDirectory(const char *szDirectoryOrFileName,
                          const char *szNewDirectoryOrFileName)
    WASM_IMPORT("renameFileOrDirectory");                         // implemented
int fileExists(char *szFilePath) WASM_IMPORT("fileExists");       // implemented
int makeDirectory(char *szFilePath) WASM_IMPORT("makeDirectory"); // implemented
int changeDirectory(char *szFilePath) WASM_IMPORT("changeDirectory")
    WASM_IMPORT("changeDirectory");
int getDirectoryItemByIndex(const char *szPathDirectory, char *szFileName,
                            int bIncludeExtension, int iIndex)
    WASM_IMPORT("getDirectoryItemByIndex"); // implemented
void getVolumeInfo(int *piFree, int *piTotal)
    WASM_IMPORT("getVolumeInfo"); // implemented
int removeFileOrDirectory(char *szFilePath)
    WASM_IMPORT("removeFileOrDirectory"); // implemented

// ===============================================================================
// User Interface
// ===============================================================================

/**
 * @brief get the data from the event queue
 * @param data Array to store the event data. Must be at least FWGUI_EVENT_DATA_MAX bytes
 * @return The oldest event in the queue
 */
int getEventData(unsigned char *data) WASM_IMPORT("getEventData");
/**
 * @brief check if there are events in the queue.
 * @return 1 if there are events, 0 if there are no events.
 */
int hasEvent() WASM_IMPORT("hasEvent");

// ===============================================================================
// Panels
// ===============================================================================
void addPanel(int iPanelIndex, int iVisible, int iInRotation, int iUseTile,
              int iTileID, int iR, int iG, int iB,
              int iShowMenu) WASM_IMPORT("addPanel"); // tested
void addPanelPickList(int iPanelIndex, const char *szCaption, int iTileID,
                      int iIconID, unsigned char iRBack, unsigned char iGBack,
                      unsigned iBBack, unsigned char iRFore,
                      unsigned char iGFore, unsigned iBFore, int iLogIndex)
    WASM_IMPORT("addPanelPickList"); // implemented
void setPanelMenuText(int iPanel, int iButtonGreyFromZero,
                      const char *szMessage)
    WASM_IMPORT("setPanelMenuText"); // tested

////////// controls
void addControlLED(int iPanelIndex, int iControlIndex, int iX, int iY,
                   enum ePanelColorLED iColor, enum ePanelSizeLED iSize,
                   int iIntialState)
    WASM_IMPORT("addControlLED"); // redefined and implemented

void setListItemText(int iLogIndex, int iListIndex, const char *szText)
    WASM_IMPORT("setListItemText");                      // implemented
void setListItemSelected(int iLogIndex, int iListIndex); // implemented
void setListItemTopIndex(int iLogIndex, int iListIndex); // implemented
void clearLogOrPlotData(int iLogIndexPlusOne, int iPlotIndexPlusOne)
    WASM_IMPORT("clearLogOrPlotData"); // implemented

void addControlLogList(int iPanelIndex, int iControlIndex, int iVisible,
                       int iLog, int iX, int iY, int iWidth, int iHeight,
                       int iFontType, int iFontSize, int iR, int iG, int iB,
                       int iRFont, int iGFont, int iBFont, int iListMode)
    WASM_IMPORT("addControlLogList"); // implemented

void addControlPlotXAxis(int iPanelIndex, int iControlIndex, int iScrollMode,
                         unsigned long long iTimeMin,
                         unsigned long long iTimeMax)
    WASM_IMPORT("addControlPlotXAxis");

void addControlPlotData(int iPlotDataIndex, int iR, int iG, int iB)
    WASM_IMPORT("addControlPlotData"); // implemented

void addControlPlot(int iPanelIndex, int iControlIndex, int iVisible,
                    int iPlotDataIndexBitField, int iX, int iY, int iWidth,
                    int iHeight, int iMin, int iMax, int iR, int iG, int iB)
    WASM_IMPORT("addControlPlot"); // implemented
void addControlNumber(int iPanelIndex, int iControlIndex, int iVisible, int iX,
                      int iY, int iWidth, int iFontSize, int iFontType, int iR,
                      int iG, int iB, int iIsFloat, int iFloatDigits,
                      int bIsHexFormat,
                      int bIsUnsigned)
    WASM_IMPORT("addControlNumber"); // implemented
void addControlPicture(int iPanelIndex, int iControlIndex, int iX, int iY,
                       int iPictureId, int iVisible)
    WASM_IMPORT("addControlPicture"); // tested
void addControlText(int iPanelIndex, int iControlIndex, int iX, int iY,
                    int iFontType, int iFontSize, int iR, int iG, int iB,
                    const char *szText)
    WASM_IMPORT("addControlText"); // implemented
void addControlBargraph(int iPanelIndex, int iControlIndex, int iVisible,
                        int iX, int iY, int iWidth, int iHeight, int iMin,
                        int iMax, int iR, int iG, int iB)
    WASM_IMPORT("addControlBargraph"); // implemented

void addControlButton(int iPanelIndex, int iControlIndex, int iVisible, int iX,
                      int iY, int iWidth, int iHeight, int iR, int iG, int iB,
                      int iRFore, int iGFore, int iBFore,
                      const char *szText)
    WASM_IMPORT("addControlButton"); // implemented

void setControlValueMinMax(int iPanelIndex, int iControlIndex,
                           int bEnableMinMax, int iMin, int iMax)
    WASM_IMPORT("setControlValueMinMax"); // implemented
void setControlValueMinMaxF(int iPanelIndex, int iControlIndex,
                            int bEnableMinMax, float iMinF, float iMaxF)
    WASM_IMPORT("setControlValueMinMaxF"); // implemented

void setLogDataText(int iLogIndex, const char *szText)
    WASM_IMPORT("setLogDataText"); // implemented
void setPlotData(int iPlotData, int iSettings, int iNewValue)
    WASM_IMPORT("setPlotData"); // implemented
void setControlValue(int iPanelIndex, int iControlIndex, int iNewValue)
    WASM_IMPORT("setControlValue"); // tested
void setControlValueFloat(int iPanelIndex, int iControlIndex, float fNewValue)
    WASM_IMPORT("setControlValueFloat"); // implemented

void exitToMainAppMenu() WASM_IMPORT("exitToMainAppMenu"); // implemented

void showPanel(int iPanel) WASM_IMPORT("showPanel"); // tested

void addControlPictureFromFile(int iPanelIndex, int iControlIndex, int iX,
                               int iY, const char *szPicture, int iVisible)
    WASM_IMPORT("addControlPictureFromFile"); // implemented

// ===============================================================================
// Debug Print
// ===============================================================================
void printInt(const char *szFormatSpec, enum printOutColor iColor,
              enum printOutDataType iDataType, int iDataValue)
    WASM_IMPORT("printInt"); // implemented
void printFloat(const char *szFormatSpec, enum printOutColor iColor,
                float fDataItem) WASM_IMPORT("printFloat"); // implemented

// ===============================================================================
// Sensors
// ===============================================================================
void setAudioSettings(int bStreamMic, int bStreamFFT, int bEnableMicPlotData,
                      int iMICPlotDataIndex, int bEnableFFTPlotData,
                      int iFFTPlotDataIndex)
    WASM_IMPORT("setAudioSettings"); // implemented

// ===============================================================================
// FPGA
// ===============================================================================
int loadFPGAFromFile(char *szFilePath)
    WASM_IMPORT("loadFPGAFromFile"); // implemented

// ===============================================================================
// Zoom IO
// ===============================================================================
int runZoomIOScript(char *szScript)
    WASM_IMPORT("runZoomIOScript"); // implemented

// ===============================================================================
// RTC
// ===============================================================================
void getRTC() WASM_IMPORT("getRTC"); // implemented

// ===============================================================================
// TODO
// ===============================================================================

void setSensorSettings(int bStreamAccel, int bStreamTemp, int iRateMilliseconds,
                       int bEnableAccelXPlotData, int iAccelXPlotDataIndex,
                       int bEnableAccelYPlotData, int iAccelYPlotDataIndex,
                       int bEnableAccelZPlotData, int iAccelZPlotDataIndex,
                       int bEnableTempPlotData, int iTempPlotDataIndex)
    WASM_IMPORT("setSensorSettings");

void setAppLogSettings(int bLogIRCodes, int bLogAccel, int bLogTempC,
                       int bLogTempF, int iLogIndex)
    WASM_IMPORT("setAppLogSettings");

// ===============================================================================
// Dialogs
// ===============================================================================
void showDialogMsgBox(char *szMessage, int bShowOk, int bShowOkCancel,
                      int bShowYesNo, int iPictureIndex, int iAutoCloseHalfSec)
    WASM_IMPORT("showDialogMsgBox");
void showDialogProgressBar(char *szMessage, int iPictureIndex, int iValue0to100,
                           int bShowOkToClose)
    WASM_IMPORT("showDialogProgressBar");
void showDialogNumEdit(char *szMessage, int bUnsigned, int bHexFormat,
                       int bUseMinMax, int iInitalValue, int iMin, int iMax)
    WASM_IMPORT("showDialogNumEdit");
void showDialogNumEditFloat(char *szMessage, int iFoatDigits, int bUseMinMax,
                            float fInitalValue, int fMin, int fMax)
    WASM_IMPORT("showDialogNumEditFloat");
void showDialogTextEdit(char *szMessage, char *szInitialValue)
    WASM_IMPORT("showDialogTextEdit");
void showDialogPickList(char *szMessage, int iLogIndex, int iDefaultItem)
    WASM_IMPORT("showDialogPickList");

#ifdef __cplusplus
} // extern "C"
#endif