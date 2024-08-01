const std = @import("std");
const Tokenizer = @import("Tokenizer.zig");

pub fn run(input: []u8) !void {
    var tokenizer = Tokenizer{ .input = input };
    while (!tokenizer.is_eof()) {
        try tokenizer.next();
        std.debug.print("{}\n", .{tokenizer.current_token});
    }
}
