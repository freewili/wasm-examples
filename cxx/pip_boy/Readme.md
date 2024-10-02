# C++ WASM Example

Example C++ FreeWili WASM application that exercises the API.

## Environment setup

- Install [emscripten](https://emscripten.org/docs/getting_started/downloads.html)
  - Quick start (read the actual documentation for more in depth information)
    - `git clone https://github.com/emscripten-core/emsdk.git`
    - `cd emsdk`
    - `./emsdk install latest`
    - `./emsdk activate latest`
    - Check emcc exists and version:
        ```
        emcc -v
        emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.68 (ceee49d2ecdab36a3feb85a684f8e5a453dde910)
        clang version 20.0.0git (https:/github.com/llvm/llvm-project 5cc64bf60bc04b9315de3c679eb753de4d554a8a)
        Target: wasm32-unknown-emscripten
        Thread model: posix
        InstalledDir: C:\dev\emsdk\upstream\bin
        ```
  - Building:
    - command line:
      ```bash
      $ emcmake cmake -B build
      $ cmake --build build
      ```
    - vscode:
      - vscode expects an environment variable `EMSDK_PATH` set to emsdk directory. This will allow vscode to build the project.
        - Linux:
          - `EMSDK_PATH=$(dirname $(which emcc)) code`
        - Windows:
          - TODO
        - Mac OSX:
          - TODO
  

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -di 1 -s ../../fwi/pip_boy.fwi -fn /images/pip_boy.fwi`
- Send WASM file to the Free-Wili:
  - `fwi-serial -mi 1 -s build/cxx_demo.wasm -fn /scripts/cxx_demo.wasm`
- Run the WASM file
  - `fwi-serial -mi 1 -w cxx_demo.wasm`
- Send and Run at the same time:
  - `fwi-serial -mi 1 -s build/cxx_demo.wasm -fn /scripts/cxx_demo.wasm -w cxx_demo.wasm`
