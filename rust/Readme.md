# Rust WASM PIP Boy example

Example FreeWili WASM application that blinks LEDs and displays an image

## Environment setup

- Install Rust

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -s ../fwi/pip_boy.fwi`

### Automatic

Cargo is setup to automatically call fwi-serial on run:
- `cargo run --release --bin rust_radio`

  *Note:* if not running on windows change `.cargo/config.toml`'s `runner` argument `fwi-serial.cmd` to `fwi-serial` under the `[target.wasm32-unknown-unknown]` section.
### Manual

You can also skip the automatic upload to freewili with the following commands:

- `cargo build --release --bin project_name`
- Send WASM file to the Free-Wili:
  - `fwi-serial -s target/wasm32-unknown-unknown/release/project_name.wasm`
- Run the WASM file
  - `fwi-serial -w project_name.wasm`
- Send and Run at the same time:
  - `fwi-serial -w -s target/wasm32-unknown-unknown/release/project_name.wasm`