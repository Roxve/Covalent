const std = @import("std");
const root = @import("root.zig");

const print = std.debug.print;

const stdout = std.io.getStdOut().writer();
const stdin = std.io.getStdIn().reader();

pub const c_allocator = std.heap.c_allocator;

pub fn main() !void {
    print("All your {s} are belong to us.\n", .{"codebase"});

    var args = try std.process.argsWithAllocator(c_allocator);
    defer args.deinit();
    _ = args.skip();

    const path = args.next() orelse return repl();
    const abs_path = try std.fs.realpathAlloc(c_allocator, path);

    const file = try std.fs.openFileAbsolute(abs_path, std.fs.File.OpenFlags{});
    const input = try file.readToEndAlloc(c_allocator, std.math.maxInt(usize));
    file.close();

    try root.run(input);
    print("path: {s}\ninput:\n{s}\n", .{ path, input });
}

fn repl() !void {
    var buffer = std.ArrayList(u8).init(c_allocator);
    defer buffer.deinit();
    while (true) {
        try stdout.writeAll(">> ");

        try stdin.streamUntilDelimiter(buffer.writer(), '\n', null);
        const input = try buffer.toOwnedSlice();

        try root.run(input);

        print("input: {!s}\n", .{input});
    }
}
