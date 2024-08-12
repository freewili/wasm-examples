# Zig WASM pip boy example

# WARNING: THIS IS A WIP and is currently not working
# WARNING: THIS IS A WIP and is currently not working
# WARNING: THIS IS A WIP and is currently not working
# WARNING: THIS IS A WIP and is currently not working

Example Zig FreeWili WASM application that blinks LEDs and shows PIP Boy


## Environment setup

- Install Zig
- `zig build`
  or:
- `zig build-exe -target wasm32-wasi src/main.zig -fno-entry --export=main --stack 131056 -OReleaseSmall -I../cxx/pip_boy/include/`

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

