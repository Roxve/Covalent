const std = @import("std");

type: TokenType,

start: usize,
width: u16,
line: u16,
col: u16,

pub const TokenType = union(enum) {
    plus,
    minus,
    mul,
    div,

    equal,
    equal_equal,

    // literals
    number: []u8,

    eof,
    pub fn width(self: @This()) u16 {
        switch (self) {
            inline else => |v| {
                switch (@typeInfo(@TypeOf(v))) {
                    .Pointer => {
                        return @truncate(@field(v, "len"));
                    },
                    else => return 1,
                }
            },
        }
    }
};
