#include <fwwasm.h>

auto main() -> int {
    for (auto led_index = 0; led_index < 7; ++led_index) {
        setBoardLED(led_index, 0x30, 0x30, 0x30, 300, LEDManagerLEDMode::ledpulsefade);   
    }

    return 0;
}