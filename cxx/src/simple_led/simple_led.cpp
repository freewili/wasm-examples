#include <fwwasm.h>
#include <ranges>

auto main() -> int {
    for (const auto led_index : std::views::iota(0, 7)) {
        setBoardLED(led_index, 0x30, 0x30, 0x30, 300, LEDManagerLEDMode::ledpulsefade);   
    }

    return 0;
}