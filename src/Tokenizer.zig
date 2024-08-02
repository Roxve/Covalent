const std = @import("std");
const c_allocator = std.heap.c_allocator;

const Token = @import("Token.zig");
const TokenType = Token.TokenType;

current_token: Token = undefined,
input: []u8,

line: u16 = 1,
col: u16 = 1,
pos: usize = 0,
start_pos: usize = 0, // pos before creating a token

pub fn is_eof(self: @This()) bool {
    return self.pos >= self.input.len;
}

inline fn at(self: @This()) u8 {
    return self.input[self.pos];
}

inline fn skip(self: *@This()) void {
    self.pos += 1;
    self.col += 1;
}

/// skip if at is expected and return true otherwise returns false
fn skip_if(self: *@This(), expected: u8) bool {
    if (self.is_eof()) return false;

    if (self.at() == expected) {
        self.skip();
        return true;
    }

    return false;
}

fn eat(self: *@This()) u8 {
    defer self.skip();
    return self.at();
}

fn token(self: *@This(), ttype: TokenType) void {
    self.current_token = Token{ .type = ttype, .line = self.line, .col = self.col, .start = self.start_pos, .width = @truncate(self.pos - self.start_pos) };
}

inline fn is_num(self: @This()) bool {
    if (self.is_eof()) return false;

    return switch (self.at()) {
        '0'...'9', '.' => true,
        else => false,
    };
}

inline fn is_allowed_ident(self: @This()) bool {
    if (self.is_eof()) return false;
    return switch (self.at()) {
        '0'...'9', '_', 'a'...'z', 'A'...'Z' => true,
        else => false,
    };
}

fn next_num(self: *@This(), start: u8) !TokenType {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    try res.append(start);
    while (self.is_num()) try res.append(self.eat());

    return TokenType{ .number = try res.toOwnedSlice() };
}

fn next_string(self: *@This()) !TokenType {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    while (if (!self.is_eof()) self.at() != '"' else false) try res.append(self.eat());
    if (self.is_eof()) {
        return error.UnterminatedStringLiteral;
    }

    self.skip(); // skips '"'

    const slice = try res.toOwnedSlice();
    return TokenType{ .string = slice };
}

fn next_char(self: *@This()) !TokenType {
    const c = self.eat();
    if (self.is_eof()) return error.UnterminatedCharLiteral;
    if (self.eat() != '\'') return error.UnterminatedCharLiteral;

    return TokenType{ .char = c };
}

fn next_ident(self: *@This(), start: u8) !TokenType {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    try res.append(start);
    while (self.is_allowed_ident()) try res.append(self.eat());

    const slice = try res.toOwnedSlice();
    return Token.keywords.get(slice) orelse return TokenType{ .ident = slice };
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

    self.start_pos = self.pos;

    const ttype: TokenType = switch (self.eat()) {
        '+' => TokenType.plus,
        '-' => TokenType.minus,
        '*' => TokenType.mul,
        '/' => TokenType.div,
        '=' => if (self.skip_if('=')) TokenType.equal_equal else TokenType.equal,
        '(' => TokenType.left_paren,
        ')' => TokenType.right_paren,

        '0'...'9' => |d| try self.next_num(d),
        '"' => try self.next_string(),
        '\'' => try self.next_char(),
        'a'...'z', 'A'...'Z', '_' => |c| try self.next_ident(c),
        '#' => {
            while (if (!self.is_eof()) self.at() != '\n' else false) self.skip();
            return self.next();
        },

        else => |_| return error.UnknownChar,
    };

    self.token(ttype);
}

pub fn init(input: []u8) !@This() {
    var self = @This(){ .input = input };
    try self.next();

    return self;
}
