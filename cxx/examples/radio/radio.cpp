#include <fwwasm.h>

#include <array>
#include <cstdint>
#include <ranges>

const auto NUMBER_OF_LEDS = 7;

struct Color {
    uint8_t red;
    uint8_t green;
    uint8_t blue;

    constexpr Color(uint8_t red, uint8_t green, uint8_t blue) : red(red), green(green), blue(blue) {}
};

constexpr Color RED(255, 0, 0);
constexpr Color ORANGE(255, 127, 0);
constexpr Color YELLOW(255, 255, 0);
constexpr Color GREEN(0, 255, 0);
constexpr Color LIGHT_GREEN(0, 255, 191);
constexpr Color BLUE(0, 0, 255);
constexpr Color LIGHT_BLUE(0, 191, 255);
constexpr Color INDIGO(75, 0, 130);
constexpr Color VIOLET(238, 130, 238);
constexpr Color PINK(255, 192, 203);
constexpr Color GRAY(0x30, 0x30, 0x30);
constexpr Color WHITE(255, 255, 255);

struct PanelInfo {
    const uint8_t index;
    const FWGuiEventType event_type;
    const Color color;
    const char* text;
    const char* sub_fname;
};

constexpr std::array Panels{
    PanelInfo{1, FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON, GRAY, "GRAY", "/radio/white.sub"},
    PanelInfo{2, FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON, YELLOW, "YELLOW", "/radio/yellow.sub"},
    PanelInfo{3, FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON, GREEN, "GREEN", "/radio/green.sub"},
    PanelInfo{4, FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON, BLUE, "BLUE", "/radio/blue.sub"},
    PanelInfo{5, FWGuiEventType::FWGUI_EVENT_RED_BUTTON, RED, "RED", "/radio/red.sub"},
};

constexpr std::array Buttons{
    FWGuiEventType::FWGUI_EVENT_GRAY_BUTTON,  FWGuiEventType::FWGUI_EVENT_YELLOW_BUTTON,
    FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON, FWGuiEventType::FWGUI_EVENT_BLUE_BUTTON,
    FWGuiEventType::FWGUI_EVENT_RED_BUTTON,
};

auto setup_panels() -> void {
    // Setup the main panel that shows pip boy
    addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    addControlPictureFromFile(0, 0, 0, 0, "pip_boy.fwi", 1);
    addControlText(0, 1, 90, 180, 1, 64, WHITE.red, WHITE.green, WHITE.blue, "Press a Button");
    showPanel(0);
    // Setup the rest of the panels
    for (auto& panel : Panels) {
        addPanel(panel.index, 1, 0, 0, 0, panel.color.red, panel.color.green, panel.color.blue, 1);
        addControlText(panel.index, 1, 10, 50, 2, 0, 0, 0, 0, panel.text);
    }
}

auto show_rainbow_leds(const auto max_loops) -> void {
    const std::array colors{RED, ORANGE, YELLOW, GREEN, LIGHT_GREEN, BLUE, LIGHT_BLUE, INDIGO, VIOLET, PINK};
    size_t color_choice = 0;
    // do the whole thing multiple times
    for (int loops = 0; loops < max_loops; loops++) {
        // set every LED one at a time

        for (int led = 0; led < NUMBER_OF_LEDS; led++) {
            // pick a color
            const Color* const color = &colors[color_choice];
            // set the LED
            setBoardLED(led, color->red, color->green, color->blue, 300, LEDManagerLEDMode::ledpulsefade);

            // next time, get a new color.  If we used all of the colors, start over
            color_choice++;
            if (color_choice >= colors.size()) {
                color_choice = 0;
            }
            // wait before setting the next LED
            waitms(50);
        }
    }
}

auto process_events() -> void {
    printInt("\nListening to events...\n", printOutColor::printColorNormal, printOutDataType::printUInt32, 0);
    auto red_count = 0;
    while (true) {
        waitms(33);
        if (hasEvent() == 0) {
            continue;
        }
        uint8_t event_data[FW_GET_EVENT_DATA_MAX] = {0};
        auto last_event = getEventData(event_data);
        printInt("\nLast event: %d\n", printOutColor::printColorNormal, printOutDataType::printUInt32, last_event);
        // We only want to process button presses
        if (std::find(std::begin(Buttons), std::end(Buttons), last_event) == std::end(Buttons)) {
            continue;
        }
        // Lets match up the button to the panel info and do everything
        for (auto& panel : Panels) {
            // If we match the event type, show the panel, flash leds and transmit the radio
            if (panel.event_type == last_event) {
                showPanel(panel.index);
                for (const auto led_index : std::views::iota(0, NUMBER_OF_LEDS)) {
                    setBoardLED(led_index, panel.color.red, panel.color.green, panel.color.blue, 250,
                                LEDManagerLEDMode::ledpulse);
                }
                RadioTxSubFile(1, panel.sub_fname);
                // Wait for the radio to stop transmitting
                while (RadioSubFileIsTransmitting() != 0) {
                    waitms(33);
                }
                // show the main panel
                showPanel(0);
                break;
            }
        }
        // we need an exit condition
        if (last_event == FWGuiEventType::FWGUI_EVENT_RED_BUTTON) {
            printInt("\nLast event was red button: %d\n", printOutColor::printColorNormal,
                     printOutDataType::printUInt32, red_count);
            red_count += 1;
            if (red_count >= 3) {
                RadioTxSubFile(1, "/radio/off.sub");
                // Wait for the radio to stop transmitting
                while (RadioSubFileIsTransmitting() != 0) {
                    waitms(33);
                }
                exitToMainAppMenu();
                break;
            }
        } else {
            red_count = 0;
        }
    }
}

auto main() -> int {
    setup_panels();

    show_rainbow_leds(5);

    process_events();

    return 0;
}
