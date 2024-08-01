const std = @import("std");
const c_allocator = std.heap.c_allocator;

const Token = @import("Token.zig");
const TokenType = Token.TokenType;

current_token: Token = undefined,
input: []u8,

line: u16 = 1,
col: u16 = 1,
pos: usize = 0,

pub fn is_eof(self: @This()) bool {
    return self.pos >= self.input.len;
}

inline fn at(self: @This()) u8 {
    return self.input[self.pos];
}

/// eat if at is expected and return true otherwise returns false
fn eat_if(self: *@This(), expected: u8) bool {
    if (self.is_eof()) return false;

    if (self.at() == expected) {
        _ = self.eat();
        return true;
    }

    return false;
}

inline fn skip(self: *@This()) void {
    self.pos += 1;
    self.col += 1;
}

fn eat(self: *@This()) u8 {
    defer self.skip();
    return self.at();
}

fn token(self: *@This(), ttype: TokenType) void {
    self.current_token = Token{ .type = ttype, .start = self.pos, .line = self.line, .col = self.col, .width = ttype.width() };
}

inline fn is_digit(self: @This()) bool {
    if (self.is_eof()) return false;

    return switch (self.at()) {
        '0'...'9', '.' => true,
        else => false,
    };
}

fn next_num(self: *@This(), start: u8) !TokenType {
    var list = std.ArrayList(u8).init(c_allocator);
    defer list.deinit();

    try list.append(start);

    while (self.is_digit()) try list.append(self.eat());

    return TokenType{ .number = try list.toOwnedSlice() };
}

/// generates a token starting from current position in input buffer then sets @This().current_token to it
pub fn next(self: *@This()) !void {
    if (self.is_eof()) {
        return self.token(TokenType.eof);
    }

    // skip until we can actually make a token
    while (true) {
        switch (self.at()) {
            ' ', '\t', '\r' => self.skip(),
            '\n' => {
                self.col = 0;
                self.line += 1;
                self.pos += 1;
            },
            else => break,
        }
    }

    const ttype: TokenType = switch (self.eat()) {
        '+' => TokenType.plus,
        '-' => TokenType.minus,
        '*' => TokenType.mul,
        '/' => TokenType.div,
        '=' => if (self.eat_if('=')) TokenType.equal_equal else TokenType.equal,
        '0'...'9' => |d| try self.next_num(d),

        else => |_| return error.UnknownChar,
    };

    self.token(ttype);
}

pub fn init(input: []u8) !@This() {
    var self = @This(){ .input = input };
    try self.next();

    return self;
}
