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
- Send the fwi file to the display processor's images directory
  - `fwi-serial -i 2 -s pip_boy.fwi -fn /images/pip_boy.fwi`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s build/pip_boy.wasm -fn /scripts/pip_boy.wasm`
- Run the WASM file
  - `fwi-serial -w pip_boy.wasm`
- Send and Run at the same time:
  - `fwi-serial -s build/pip_boy.wasm -fn /scripts/pip_boy.wasm -w pip_boy.wasm`