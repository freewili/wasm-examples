# Zig WASM pip boy example

Example Zig FreeWili WASM application that blinks LEDs and shows PIP Boy

## Environment setup

- Install Zig
- Build the project:
  - `zig build`

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -i 2 -s zig_radio.fwi -fn /images/zig_radio.fwi`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s zig-out/bin/zig_radio.wasm -fn /scripts/zig_radio.wasm`
- Run the WASM file
  - `fwi-serial -w zig_radio.wasm`
- Send and Run at the same time:
  - `fwi-serial -s zig-out/bin/zig_radio.wasm -fn /scripts/zig_radio.wasm -w zig_radio.wasm`
