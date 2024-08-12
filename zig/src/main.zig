// zig build-exe -target wasm32-wasi src/main.zig -fno-entry --export=main --stack 131056 -OReleaseSmall
// const fw = @cImport({
//     @cInclude("fwwasm.h");
// });

extern "wiliwasm" fn setled(on: c_int) void;
extern "wiliwasm" fn waitms(ms: c_int) void;
extern "wiliwasm" fn i2cRead(address: c_int, register: c_int, bytes: *c_char, length: c_int) c_int;
extern "wiliwasm" fn i2cWrite(address: c_int, register: c_int, bytes: *c_char, length: c_int) c_int;
extern "wiliwasm" fn addPanel(iPanelIndex: c_int, iVisible: c_int, iVisible: c_int, iUseTile: c_int, iTileID: c_int, iR: u8, iG: u8, iB: u8, iShowMenu: c_int) void;
extern "wiliwasm" fn addControlPictureFromFile(iPanelIndex: c_int, iControlIndex: c_int, iX: c_int, iY: c_int, szPicture: [:0]const u8, iVisible: c_int) void;
extern "wiliwasm" fn showPanel(iPanel: c_int) void;

pub fn display_image() void {
    // Create a panel to display the image
    addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    addControlPictureFromFile(0, 0, 0, 0, "pip_boy.fwi", 1);
    // Show the Panel
    showPanel(0);
}

pub export fn main() void {
    display_image();

    var led_on: bool = false;
    for (0..25) |_| {
        const data_bytes: [4]u8 = .{ 0x55, 0x22, 0x11, 0x82 };
        _ = i2cWrite(
            0x23,
            11,
            @constCast(@ptrCast(&data_bytes)),
            data_bytes.len,
        );
        // TODO: check result
        waitms(100);
        setled(@intFromBool(led_on));
        led_on = !led_on;
    }
}
