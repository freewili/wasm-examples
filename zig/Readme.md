# Zig WASM PIP Boy Example

Example Zig FreeWili WASM application that blinks LEDs and shows PIP Boy interface.

## Prerequisites

- [Zig](https://ziglang.org/) compiler (tested with v0.14.0-dev)
- [Python](https://www.python.org/) with [uv](https://docs.astral.sh/uv/) package manager
- [FreeWili](https://pypi.org/project/freewili/) Python module

## Setup

1. Install uv (if not already installed):

2. Install FreeWili module:
   ```bash
   uv add freewili
   ```

## Building

- **Build only**: `zig build`
- **Build and deploy**: `zig build fwi` (builds and automatically uploads to FreeWili)

## Manual Deployment

If you prefer to deploy manually or need more control:

### Send WASM file to FreeWili:
```bash
uv run fwi-serial -s zig-out/bin/zig_radio.wasm
```

### Run the WASM file:
```bash
uv run fwi-serial -w zig_radio.wasm
```

### Send and run in one command:
```bash
uv run fwi-serial -w -s zig-out/bin/zig_radio.wasm
```

### Send FWI display file:
```bash
uv run fwi-serial -s ../fwi/pip_boy.fwi
```

## Memory Configuration

This project uses optimized memory settings for FreeWili's 64KB constraint:
- `global_base = 1024` - Reserves first 1KB for runtime use
- `stack_size = 59392` - ~58KB stack size
- `initial_memory = 65536` - 64KB initial memory
- `max_memory = 65536` - 64KB maximum memory limit
