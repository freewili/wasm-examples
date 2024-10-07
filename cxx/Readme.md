# C++ WASM Example

Example C++ FreeWili WASM application that exercises the API.

## Environment setup

- Install [WASI-SDK](https://github.com/WebAssembly/wasi-sdk/releases)
- Set `WASI_SDK_PATH` environment variable
- Building:
  - command line:
    ```bash
    $ cmake -DCMAKE_TOOLCHAIN_FILE=${WASI_SDK_PATH}/share/cmake/wasi-sdk.cmake -B build . && cmake --build build
    ```
  - Visual Studio Code:
    - vscode expects an environment variable `WASI_SDK_PATH` set to emsdk directory. This will allow vscode to build the project.
      - Linux:
        - `WASI_SDK_PATH=/opt/wasi-sdk-24.0-x86_64-linux code .`
      - Windows (Powershell):
        - `$env:WASI_SDK_PATH="C:\Path\To\wasi-sdk"; code .`
      - Mac OSX:
        - TODO
    - cmake expects to be able to find `${env:WASI_SDK_PATH}/share/cmake/wasi-sdk.cmake`
      - Inside the [VSCode command Palette](https://code.visualstudio.com/api/ux-guidelines/command-palette) run the following commands:
        - `>CMake: Select Variant` and select `MinSizeRel`
        - `>CMake: Configure`
        - `>CMake: Build Target`
        - `>CMake: Select A Kit`
          - Make sure the kit is unspecified.
  

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -di 1 -s ../../fwi/pip_boy.fwi -fn /images/pip_boy.fwi`
  - *Note:* Not Required for all
- Send WASM file to the Free-Wili:
  - `fwi-serial -mi 1 -s build/examples/project_name/project_name.wasm -fn /scripts/project_name.wasm`
- Run the WASM file
  - `fwi-serial -mi 1 -w project_name.wasm`
- Send and Run at the same time:
  - `fwi-serial -mi 1 -s build/examples/project_name/project_name.wasm -fn /scripts/project_name.wasm -w project_name.wasm`
