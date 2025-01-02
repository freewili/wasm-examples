#include <stdint.h>
#include "fwwasm.h"

volatile uint8_t exitApp = 0;

void processAccelData(uint8_t *event_data)
{
    int16_t iX, iY, iZ, iTc, iTf;
    bool iMoving, iMovingX, iMovingY, iMovingZ;

    iX = event_data[0] | event_data[1] << 8;
    iY = event_data[2] | event_data[3] << 8;
    iZ = event_data[4] | event_data[5] << 8;
    iTc = event_data[6] | event_data[7] << 8;
    iTf = event_data[8] | event_data[9] << 8;
    iMoving = event_data[10] & 0x1;
    iMovingX = event_data[10] & 0x2 ? 1 : 0;
    iMovingY = event_data[10] & 0x4 ? 1 : 0;
    iMovingZ = event_data[10] & 0x8 ? 1 : 0;
    printInt("Got accel: X:%d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iX);
    printInt("Got accel: Y:%d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iY);
    printInt("Got accel: Z:%d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iZ);
    printInt("Got iMoving: %d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iMoving);
    printInt("Got isMovingX: %d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iMovingX);
    printInt("Got isMovingY: %d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iMovingY);
    printInt("Got isMovingZ: %d\n", printOutColor::printColorBlue, printOutDataType::printInt16, iMovingZ);
}

void loop()
{
    uint8_t event_data[FW_GET_EVENT_DATA_MAX] = {0};
    int last_event;

    printInt("\nloop()\n", printOutColor::printColorBlack, printOutDataType::printUInt32, 0);

    // check if there is an event, and if so, get the data from it
    last_event = 0;
    if (hasEvent())
    {
        last_event = getEventData(event_data);
        printInt("Last event: %d\n", printOutColor::printColorBlack, printOutDataType::printUInt32, last_event);
    }

    // if the event was SENSOR_DATA, do something with it
    if (last_event == FWGUI_EVENT_GUI_SENSOR_DATA)
    {
        processAccelData(event_data);
    }

//as soon as I enable this if condition the app will not run...?
#if (0)
    // exit condition: red button pressed
    else if (last_event == FWGUI_EVENT_RED_BUTTON)
    {
        printInt("Exit...\n", printOutColor::printColorBlack, printOutDataType::printUInt32, 0);
        exitApp = 1;
    }
#endif
}

int main()
{
    setSensorSettings(1, 0, 1000, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0);
    printInt("\nmain()\n", printOutColor::printColorBlack, printOutDataType::printUInt32, 0);
    while (!exitApp)
    {
        loop();
        waitms(1000);
    }
    return 0;
}