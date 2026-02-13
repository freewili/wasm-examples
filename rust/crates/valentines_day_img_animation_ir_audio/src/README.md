# Valentine's Day Demo for Free-Wili 🌹❤️

An interactive Valentine's Day demonstration for the Free-Wili embedded platform, showcasing WASM capabilities with animations, audio, LED effects, and IR communication.

## 🎯 What This Demo Does

This demo creates an interactive Valentine's Day experience on the Free-Wili device:

1. **Start Screen**: Displays a custom Valentine's image (picturestart.fwi)
2. **Audio Loop**: Continuously plays sound while waiting for user interaction
3. **Button Activation**: Press the green button to start the animation sequence
4. **LED Heartbeat**: Flashes all 7 RGB LEDs in a red double-beat heartbeat pattern
5. **IR Transmission**: Sends 4 IR codes sequentially with delays
6. **Image Animation**: Cycles through 4 Valentine's images in a continuous loop
7. **Continuous Experience**: Audio and IR codes continue transmitting throughout the animation

## 🛠️ Hardware Requirements

**Free-Wili Device** with:
  - 320x240 color display
  - 7 RGB LEDs
  - Green button
  - IR transmitter
  - Audio playback capability

## ✨ Features

**Pure Rust**: Written in no_std Rust for embedded WASM target
**Event-Driven**: Responds to button press events
**Multi-Sensory**: Combines visual (display + LEDs), audio, and IR communication
**Continuous Loop**: All effects repeat for an ongoing Valentine's experience
**Optimized**: Compiles to ~1.5-2KB WASM binary

## 📋 Required Assets

Place these files on your Free-Wili device:

picturestart.fwi - Initial Valentine's message/image
picture1.fwi - Animation frame 1
picture2.fwi - Animation frame 2
picture3.fwi - Animation frame 3
picture4.fwi - Animation frame 4

## 🔧 Building

### Prerequisites

1. Install Rust toolchain
2. Add WASM target:
bash
rustup target add wasm32-unknown-unknown

### Compile

bash
cd radio
cargo build --target wasm32-unknown-unknown --release

The compiled WASM file will be located at:
target/wasm32-unknown-unknown/release/rust_radio.wasm

## 🚀 Usage

1. Load the valentines_day.wasm file onto your Free-Wili device
2. Ensure all .fwi image files are present on the device
3. Run the WASM application
4. The start image will display with looping audio
5. Press the **green button** to activate the Valentine's sequence
6. Watch the LED heartbeat, then enjoy the continuous image animation!

## 🎮 Controls

**Green Button**: Start the Valentine's animation sequence

## 📡 IR Codes Transmitted

The demo transmits these IR codes in sequence (useful for controlling other devices):
0xa659ff00
0xa758ff00
0xba45ff00
0xbdfbffbf

Each code is sent with 300ms delay, repeating every loop cycle (500ms after the last code).

## 🎨 LED Pattern

The heartbeat pattern flashes all 7 LEDs:
**First beat**: 200ms red flash
**Second beat**: 200ms red flash (after 200ms pause)
Total pattern duration: ~500ms

## 🎵 Audio

Uses the built-in Free-Wili ROM sound: 
Plays during waiting screen
Continues playing throughout the animation loop

## 🧩 Technical Details

**Language**: Rust (edition 2021)
**Target**: wasm32-unknown-unknown
**Entry Point**: extern "C" fn _start()
**FFI Library**: fwwasm-ffi for Free-Wili hardware bindings
**Memory**: No heap allocation (no_std)
**Display Resolution**: 320x240 pixels
**Image Format**: .fwi (Free-Wili Image format)

## 📁 Project Structure

Valentine/
├── Cargo.toml          # Project configuration
├── src/
│   └── main.rs  # Main source code
└── README.md           # This file

## 🤝 Contributing

Feel free to fork and customize this demo! Some ideas:
Add more image frames
Change LED colors/patterns
Use different audio files
Add yellow button exit functionality
Implement different IR code sequences

## 📄 License

This is an example/demo project for the Free-Wili platform. Use and modify as you wish!

## 💝 Perfect For

Valentine's Day celebrations
Learning embedded Rust + WASM
Demonstrating Free-Wili capabilities
Creating interactive gift experiences
Teaching event-driven programming

---

**Happy Valentine's Day! 🌹💕**

Made with ❤️ for the FREE-WILi community