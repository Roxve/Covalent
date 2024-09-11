const std = @import("std");

pub const ATError = error{ InvaildChar, UnexpectedToken, UnterminatedStringLiteral, UnterminatedCharLiteral, AllocatorError, InvaildType, TypeMismatch, UndeclaredVariable };

/// reports an error to stderr
// TODO: error messages
// TODO: error file information and line displaying
pub fn report(err: ATError, line: u16, col: u16) void {
    std.debug.print("error {}, at {}:{}\n", .{ err, line, col });
}
