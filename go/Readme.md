# Go WASM LED example

Example Go FreeWili WASM application that blinks LEDs

## Environment setup

- Install [TinyGo](https://tinygo.org)
  - Windows:
    - `scoop install go tinygo binaryen`
- Build the project:
  - `fwi-serial -s pip_boy.wasm -fn /scripts/pip_boy.wasm -w pip_boy.wasm`


## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -i 2 -s pip_boy.fwi -fn /images/pip_boy.fwi`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s pip_boy.wasm -fn /scripts/pip_boy.wasm`
- Run the WASM file
  - `fwi-serial -w pip_boy.wasm`
- Send and Run at the same time:
  - `fwi-serial -s pip_boy.wasm -fn /scripts/pip_boy.wasm -w pip_boy.wasm`


## Windows

error: could not find wasm-opt, set the WASMOPT environment variable to override

`scoop install binaryen`