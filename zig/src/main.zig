// zig build-exe -target wasm32-wasi src/main.zig -fno-entry --export=main --stack 131056 -OReleaseSmall
const fw = @cImport({
    @cInclude("fwwasm.h");
});

pub fn display_image() void {
    // Create a panel to display the image
    fw.addPanel(0, 1, 0, 0, 0, 0, 0, 0, 1);
    // Add the image to the panel, this file should be in the images directory on the display processor
    fw.addControlPictureFromFile(0, 0, 0, 0, "pip_boy.fwi", 1);
    // Show the Panel
    fw.showPanel(0);
}

pub export fn main() void {
    //display_image();

    var led_on: bool = false;
    for (0..25) |_| {
        const data_bytes: [4]u8 = .{ 0x55, 0x22, 0x11, 0x82 };
        _ = fw.i2cWrite(
            0x23,
            11,
            @constCast(@ptrCast(&data_bytes)),
            data_bytes.len,
        );
        // TODO: check result
        fw.waitms(100);
        fw.setled(@intFromBool(led_on));
        led_on = !led_on;
    }
}
