const std = @import("std");
const fw = @cImport({
    @cInclude("fwwasm.h");
});

pub fn main() void {
    fw.printInt(@ptrCast("Hello from Zig!\n"), fw.printColorNormal, fw.printUInt32, 0);

    // Create the heap allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }
    fw.printInt(@ptrCast("Trying to allocate Memory!\n"), fw.printColorNormal, fw.printUInt32, 0);
    // Allocate some memory to test the allocator
    const data = allocator.alloc(u32, 10) catch {
        fw.printInt(@ptrCast("ERROR: Failed to allocate memory\n"), fw.printColorRed, fw.printUInt32, 0);
        return;
    };
    defer allocator.free(data);
    fw.printInt(@ptrCast("Success: Memory allocated successfully!\n"), fw.printColorGreen, fw.printUInt32, 0);

    for (data, 0..) |*value, i| {
        value.* = @intCast(i);
        fw.printInt(@ptrCast("Allocated value: %d\n"), fw.printColorNormal, fw.printUInt32, @intCast(value.*));
    }

    fw.exitToMainAppMenu();
}
