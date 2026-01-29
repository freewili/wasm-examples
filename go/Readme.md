# TinyGo WASM Example

Example TinyGo FreeWili WASM application that exercises the API.

## Environment setup

- Install [TinyGo](https://tinygo.org/)
- Building:
    ```tinygo build -target ./target.json -panic=trap -opt=z -size=short -no-debug -print-allocs=.```

## Warning

`C.CString` allocates memory on the heap and is very resource intensive, if more complex binaries are required
we recommend using a different language besides Go for now.

## Upload to Free-Wili
- Install [Python](https://www.python.org/)
- Install [Free-Wili Python module](https://pypi.org/project/freewili/)
  - `pip install freewili`
- Send the fwi file to the display processor's images directory
  - `fwi-serial -s ../../fwi/pip_boy.fwi -fn /images/pip_boy.fwi`
  - *Note:* Not Required for all
- Send WASM file to the Free-Wili:
  - `fwi-serial -s build/examples/project_name/project_name.wasm`
- Run the WASM file
  - `fwi-serial -w project_name.wasm`
- Send and Run at the same time:
  - `fwi-serial -w -s build/examples/project_name/project_name.wasm`
