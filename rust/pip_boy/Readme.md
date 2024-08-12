# Rust WASM PIP Boy example

Example FreeWili WASM application that blinks LEDs and displays an image

## Environment setup

- Install Rust

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -i 2 -s pip_boy.fwi -fn /images/pip_boy.fwi`

### Automatic

Cargo is setup to automatically call fwi-serial on run:
- `cargo run --release`
### Manual

You can also skip the automatic upload to freewili with the following commands:

- `cargo build --release`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s target/wasm32-wasip1/release/pip_boy.wasm -fn /scripts/pip_boy.wasm`
- Run the WASM file
  - `fwi-serial -w pip_boy.wasm`
- Send and Run at the same time:
  - `fwi-serial -s target/wasm32-wasip1/release/pip_boy.wasm -fn /scripts/pip_boy.wasm -w pip_boy.wasm`