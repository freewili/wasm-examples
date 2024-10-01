# C++ WASM PIP Boy example

Example C++ FreeWili WASM application that shows an image on the screen.

## Environment setup

- Install [emscripten](https://emscripten.org/index.html)
- Build:
```bash
$ emcmake cmake -B build
$ cmake --build build
```

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -di 1 -s ../../fwi/pip_boy.fwi -fn /images/pip_boy.fwi`
- Send WASM file to the Free-Wili:
  - `fwi-serial -mi 1 -s build/pip_boy.wasm -fn /scripts/pip_boy.wasm`
- Run the WASM file
  - `fwi-serial -mi 1 -w pip_boy.wasm`
- Send and Run at the same time:
  - `fwi-serial -mi 1 -s build/pip_boy.wasm -fn /scripts/pip_boy.wasm -w pip_boy.wasm`
