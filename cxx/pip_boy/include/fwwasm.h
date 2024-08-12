//
// Created on 6/13/24.
// Free Wili C API
// - v20.1 release 8-6-24

#pragma once

#define WASM_IMPORT(NAME) __attribute__((import_module("wiliwasm"))) __attribute__((import_name(NAME)))
#define WASM_EXPORT extern "C" __attribute__((used)) __attribute__((visibility("default")))
#define WASM_EXPORT_AS(NAME) WASM_EXPORT __attribute__((export_name(NAME)))

#ifdef __cplusplus
extern "C" {
#endif

#define FWGUI_EVENT_GRAY_BUTTON 0
#define FWGUI_EVENT_YELLOW_BUTTON 1
#define FWGUI_EVENT_GREEN_BUTTON 2
#define FWGUI_EVENT_BLUE_BUTTON 3
#define FWGUI_EVENT_RED_BUTTON 4
#define FWGUI_EVENT_IR_CODE 5
#define FWGUI_EVENT_GUI_BUTTON 6
#define FWGUI_EVENT_GUI_NUMEDIT 7
#define FWGUI_EVENT_GUI_TEXTEDIT 8
#define FWGUI_EVENT_GUI_AUDIO_DATA 9
#define FWGUI_EVENT_GUI_FFT_DATA 10
#define FWGUI_EVENT_GUI_I2C_RESPONSE 11
#define FWGUI_EVENT_EVENTFIFO_OVERFLOW 12
#define FWGUI_EVENT_GUI_RTC_RESPONSE 13
#define FWGUI_EVENT_GUI_SENSOR_DATA 14
#define FWGUI_EVENT_MAIN_APP_SEL 15
#define FWGUI_EVENT_PANEL_SHOW 16
#define FWGUI_EVENT_PICKLIST_SEL 17
#define FWGUI_EVENT_PANEL_HIDE 18
#define FWGUI_EVENT_MAIN_MENU_SHOWN 19
#define FWGUI_EVENT_STARTED 20
#define FWGUI_EVENT_CLR_STATS 21
#define FWGUI_EVENT_DIALOG_ACTION 22
#define FWGUI_EVENT_WASM_OVRFLOW 23

#define FWGUI_EVENT_NUMTYPE_INT 1
#define FWGUI_EVENT_NUMTYPE_UINT 2
#define FWGUI_EVENT_NUMTYPE_FLOAT 3

#define RPFWGUI_EVENT_DATA_MAX 34

// general
void waitms(int iMilliseconds) WASM_IMPORT("waitms");  // tested

// gpio
void setled(int iOn) WASM_IMPORT("setled");           // tested
void setIO(int iGPIO, int iOn) WASM_IMPORT("setIO");  // implemented
unsigned int getIO(int iGPIO) WASM_IMPORT("getIO");   // implemented
unsigned int getAllIO() WASM_IMPORT("getAllIO");      // implemented


// i2c
int i2cRead(int iAddress, int iRegister, unsigned char *pBytes, int iLength) WASM_IMPORT("i2cRead");    // implemented
int i2cWrite(int iAddress, int iRegister, unsigned char *pBytes, int iLength) WASM_IMPORT("i2cWrite");  // implemented

// SPI
int SPIReadWrite(unsigned char *pBytesIn, int iLength, unsigned char *pBytesOut)
    WASM_IMPORT("SPIReadWrite");  // implemented (dont like no chip select)

// UART
int UARTDataRxCount() WASM_IMPORT("UARTDataRxCount");                                  // implemented
int UARTDataRead(unsigned char *pBytesOut, int iLength) WASM_IMPORT("UARTDataRead");   // implemented
int UARTDataWrite(unsigned char *pBytesIn, int iLength) WASM_IMPORT("UARTDataWrite");  // implemented

// PWM
int PWMSetFreqDuty(int iGPIO, float fFreq, float fDuty) WASM_IMPORT("PWMSetFreqDuty");  // implemeted
int PWMStop(int iGPIO) WASM_IMPORT("PWMStop");                                          // implemented

// radio
int RadioWrite(int iRadioIndex, unsigned char *btBytes, int iLength) WASM_IMPORT("RadioWrite");
int RadioRead(int iRadioIndex, unsigned char *btBytes, int iLength) WASM_IMPORT("RadioRead");
int RadioGetRxCount(int iRadioIndex) WASM_IMPORT("RadioGetRxCount");
int RadioLoadConfig(int iRadioIndex, unsigned char *btBytes, int iLength) WASM_IMPORT("RadioLoadConfig");
int RadioTxSubFile(int iRadioIndex, char *szSoundPath) WASM_IMPORT("RadioTxSubFile"); 
int RadioSetTx(int iRadioIndex) WASM_IMPORT("RadioSetTx"); 
int RadioSetRx(int iRadioIndex) WASM_IMPORT("RadioSetRx"); 
int RadioSetIdle(int iRadioIndex) WASM_IMPORT("RadioSetIdle"); 

// ir
void sendIRData(unsigned int iData) WASM_IMPORT("sendIRData");  // implemented

// LEDs
enum LEDManagerLEDMode {
    ledsimplevalue,
    ledflash,
    ledpulse,
    ledflashfade,
    ledpulsefade,
};

enum ePanelColorLED {
    LEDColorRed,
    LEDColorGreen,
    LEDColorYellow,
    LEDColorBlue,
    LEDColorOrange,
    LEDColorAqua,
    LEDColorMagenta,
    LEDColorWhite,
};

enum ePanelSizeLED {
    LEDSize32,
    LEDSize48,
    LEDSize64,
};

void setBoardLED(int iLEDIndex0_7, int iR, int iG, int iB, int iDurationMs, enum LEDManagerLEDMode iMode)
    WASM_IMPORT("setBoardLED");                                   // tested
void setLEDShowMode(int iLEDShow) WASM_IMPORT("setLEDShowMode");  // tested

// sound speaker
void playSoundFromFile(char *szSoundPath) WASM_IMPORT("playSoundFromFile");                         // implemented
void playSoundFromNameOrID(const char *szSoundName, int iID) WASM_IMPORT("playSoundFromNameOrID");  // tested
void playSoundFromNumber(int bFloat, int iNumber, float fNumber, int iFloatDigits)
    WASM_IMPORT("playSoundFromNumber");  // tested

// fileio
int openFile(char *szFileName, int iForWriting) WASM_IMPORT("OpenFile");                                // implemented
int closeFile(int iHandle) WASM_IMPORT("closeFile");                                                    // implemented
int writeFile(int iHandle, unsigned char *btBytes, int iNumByte) WASM_IMPORT("writeFile");              // implemented
int preAllocateSpaceForFile(int iHandle, int iFileSizeInBytes) WASM_IMPORT("preAllocateSpaceForFile");  // implemented
int readFile(int iHandle, unsigned char *btBytes, int *iNumBytes) WASM_IMPORT("readFile");              // implemented
int readFileLine(int iHandle, char *btBytes, int *iNumBytes) WASM_IMPORT("readFileLine");               // implemented
int setFilePosition(int iHandle, int iPosition) WASM_IMPORT("setFilePosition");                         // implemented
int getFilePosition(int iHandle) WASM_IMPORT("getFilePosition");                                        // implemented
int getFileSize(int iHandle) WASM_IMPORT("getFileSize");                                                // implemented

// file system
int renameFileOrDirectory(const char *szDirectoryOrFileName, const char *szNewDirectoryOrFileName)
    WASM_IMPORT("renameFileOrDirectory");                          // implemented
int fileExists(char *szFilePath) WASM_IMPORT("fileExists");        // implemented
int makeDirectory(char *szFilePath) WASM_IMPORT("makeDirectory");  // implemented
int changeDirectory(char *szFilePath) WASM_IMPORT("changeDirectory") WASM_IMPORT("changeDirectory");
int getDirectoryItemByIndex(const char *szPathDirectory, char *szFileName, int bIncludeExtension, int iIndex)
    WASM_IMPORT("getDirectoryItemByIndex");                                        // implemented
void getVolumeInfo(int *piFree, int *piTotal) WASM_IMPORT("getVolumeInfo");        // implemented
int removeFileOrDirectory(char *szFilePath) WASM_IMPORT("removeFileOrDirectory");  // implemented

// user interface
#define FW_GET_EVENT_DATA_MAX 34
int getEventData(unsigned char *pData) WASM_IMPORT("getEventData");  // tested
int hasEvent() WASM_IMPORT("hasEvent");                              // tested

// Panel Apis
void addPanel(int iPanelIndex, int iVisible, int iInRotation, int iUseTile, int iTileID, unsigned char iR,
              unsigned char iG, unsigned iB, int iShowMenu) WASM_IMPORT("addPanel");  // tested
void addPanelPickList(int iPanelIndex, const char *szCaption, int iTileID, int iIconID, unsigned char iRBack,
                      unsigned char iGBack, unsigned iBBack, unsigned char iRFore, unsigned char iGFore,
                      unsigned iBFore, int iLogIndex) WASM_IMPORT("addPanelPickList");  // implemented
void setPanelMenuText(int iPanel, int iButtonGreyFromZero, const char *szMessage)
    WASM_IMPORT("setPanelMenuText");  // tested

////////// controls
void addControlLED(int iPanelIndex, int iControlIndex, int iX, int iY, enum ePanelColorLED iColor, enum ePanelSizeLED iSize,
                   int iIntialState) WASM_IMPORT("addControlLED");  // redefined and implemented

void setListItemText(int iLogIndex, int iListIndex, const char *szText) WASM_IMPORT("setListItemText");  // implemented
void setListItemSelected(int iLogIndex, int iListIndex);                                                 // implemented
void setListItemTopIndex(int iLogIndex, int iListIndex);                                                 // implemented
void clearLogOrPlotData(int iLogIndexPlusOne, int iPlotIndexPlusOne) WASM_IMPORT("clearLogOrPlotData");  // implemented

void addControlLogList(int iPanelIndex, int iControlIndex, int iVisible, int iLog, int iX, int iY, int iWidth,
                       int iHeight, int iFontType, int iFontSize, int iR, int iG, int iB, int iRFont, int iGFont,
                       int iBFont, int iListMode) WASM_IMPORT("addControlLogList");  // implemented

void addControlPlotXAxis(int iPanelIndex, int iControlIndex, int iScrollMode, unsigned long long iTimeMin,
                         unsigned long long iTimeMax) WASM_IMPORT("addControlPlotXAxis");

void addControlPlotData(int iPlotDataIndex, int iR, int iG, int iB) WASM_IMPORT("addControlPlotData");  // implemented

void addControlPlot(int iPanelIndex, int iControlIndex, int iVisible, int iPlotDataIndexBitField, int iX, int iY,
                    int iWidth, int iHeight, int iMin, int iMax, int iR, int iG, int iB)
    WASM_IMPORT("addControlPlot");  // implemented
void addControlNumber(int iPanelIndex, int iControlIndex, int iVisible, int iX, int iY, int iWidth, int iFontSize,
                      int iFontType, int iR, int iG, int iB, int iIsFloat, int iFloatDigits, int bIsHexFormat,
                      int bIsUnsigned) WASM_IMPORT("addControlNumber");  // implemented
void addControlPicture(int iPanelIndex, int iControlIndex, int iX, int iY, int iPictureId, int iVisible)
    WASM_IMPORT("addControlPicture");  // tested
void addControlText(int iPanelIndex, int iControlIndex, int iX, int iY, int iFontType, int iFontSize, int iR, int iG,
                    int iB,
                    const char *szText) WASM_IMPORT("addControlText");  // implemented
void addControlBargraph(int iPanelIndex, int iControlIndex, int iVisible, int iX, int iY, int iWidth, int iHeight,
                        int iMin, int iMax, int iR, int iG, int iB) WASM_IMPORT("addControlBargraph");  // implemented

void addControlButton(int iPanelIndex, int iControlIndex, int iVisible, int iX, int iY, int iWidth, int iHeight, int iR,
                      int iG, int iB, int iRFore, int iGFore, int iBFore,
                      const char *szText) WASM_IMPORT("addControlButton");  // implemented

void setControlValueMinMax(int iPanelIndex, int iControlIndex, int bEnableMinMax, int iMin, int iMax)
    WASM_IMPORT("setControlValueMinMax");  // implemented
void setControlValueMinMaxF(int iPanelIndex, int iControlIndex, int bEnableMinMax, float iMinF, float iMaxF)
    WASM_IMPORT("setControlValueMinMaxF");  // implemented

void setLogDataText(int iLogIndex, const char *szText) WASM_IMPORT("setLogDataText");                    // implemented
void setPlotData(int iPlotData, int iSettings, int iNewValue) WASM_IMPORT("setPlotData");                // implemented
void setControlValue(int iPanelIndex, int iControlIndex, int iNewValue) WASM_IMPORT("setControlValue");  // tested
void setControlValueFloat(int iPanelIndex, int iControlIndex, float fNewValue)
    WASM_IMPORT("setControlValueFloat");  // implemented

void exitToMainAppMenu() WASM_IMPORT("exitToMainAppMenu");  // implemented

void showPanel(int iPanel) WASM_IMPORT("showPanel");  // tested

void addControlPictureFromFile(int iPanelIndex, int iControlIndex, int iX, int iY, const char *szPicture, int iVisible)
    WASM_IMPORT("addControlPictureFromFile");  // implemented

enum printOutDataType {
    printInt32 = 0,
    printUInt32,
    printInt16,
    printUInt16,
    printUint8,
    printInt8,
    printChar,
    printBool,
};

enum printOutColor {
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
};

void printInt(const char *szFormatSpec, enum printOutColor iColor, enum printOutDataType iDataType, int iDataValue)
    WASM_IMPORT("printInt");  // implemented
void printFloat(const char *szFormatSpec, enum printOutColor iColor, float fDataItem)
    WASM_IMPORT("printFloat");  // implemented

// Sensors
void setAudioSettings(int bStreamMic, int bStreamFFT, int bEnableMicPlotData, int iMICPlotDataIndex,
                      int bEnableFFTPlotData, int iFFTPlotDataIndex) WASM_IMPORT("setAudioSettings");  // implemented

// fpga
int loadFPGAFromFile(char *szFilePath) WASM_IMPORT("loadFPGAFromFile");  // implemented
// run script
int runZoomIOScript(char *szScript) WASM_IMPORT("runZoomIOScript");  // implemented

// RTC
void getRTC() WASM_IMPORT("getRTC");  // implemented

// TODO

void setSensorSettings(int bStreamAccel, int bStreamTemp, int iRateMilliseconds, int bEnableAccelXPlotData,
                       int iAccelXPlotDataIndex, int bEnableAccelYPlotData, int iAccelYPlotDataIndex,
                       int bEnableAccelZPlotData, int iAccelZPlotDataIndex, int bEnableTempPlotData,
                       int iTempPlotDataIndex) WASM_IMPORT("setSensorSettings");

void setAppLogSettings(int bLogIRCodes, int bLogAccel, int bLogTempC, int bLogTempF, int iLogIndex)
    WASM_IMPORT("setAppLogSettings");

// dialogs
void showDialogMsgBox(char *szMessage, int bShowOk, int bShowOkCancel, int bShowYesNo, int iPictureIndex,
                      int iAutoCloseHalfSec) WASM_IMPORT("showDialogMsgBox");
void showDialogProgressBar(char *szMessage, int iPictureIndex, int iValue0to100, int bShowOkToClose)
    WASM_IMPORT("showDialogProgressBar");
void showDialogNumEdit(char *szMessage, int bUnsigned, int bHexFormat, int bUseMinMax, int iInitalValue, int iMin,
                       int iMax) WASM_IMPORT("showDialogNumEdit");
void showDialogNumEditFloat(char *szMessage, int iFoatDigits, int bUseMinMax, float fInitalValue, int fMin, int fMax)
    WASM_IMPORT("showDialogNumEditFloat");
void showDialogTextEdit(char *szMessage, char *szInitialValue) WASM_IMPORT("showDialogTextEdit");
void showDialogPickList(char *szMessage, int iLogIndex, int iDefaultItem) WASM_IMPORT("showDialogPickList");

#ifdef __cplusplus
}  // extern "C"
#endif
