# C++ WASM LED example

Example C++ FreeWili WASM application that blinks LEDs

## Environment setup

- Install [WASI-SDK](https://github.com/WebAssembly/wasi-sdk/releases)
- Set `WASI_SDK_PATH` environment variable
  - CMAKE expects to be able to find `${env:WASI_SDK_PATH}/share/cmake/wasi-sdk.cmake`
- Inside the [VSCode command Palette](https://code.visualstudio.com/api/ux-guidelines/command-palette) run the following commands:
  - `>CMake: Select Variant` and select `MinSizeRel`
  - `>CMake: Configure`
  - `>CMake: Build Target`

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s build/led_demo.wasm -fn /scripts/led_demo.wasm`
- Run the WASM file
  - `fwi-serial -w led_demo.wasm`
- Send and Run at the same time:
  - `fwi-serial -s build/led_demo.wasm -fn /scripts/led_demo.wasm -w led_demo.wasm`