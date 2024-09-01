const std = @import("std");
const c_allocator = std.heap.c_allocator;

const Token = @import("Token.zig");
const TokenType = Token.TokenType;
const ATError = @import("errors.zig").ATError;

current_token: Token = undefined,
input: []u8,

line: u16 = 1,
col: u16 = 1,
pos: usize = 0,
start_pos: usize = 0, // pos before creating a token

pub fn is_eof(self: @This()) bool {
    return self.pos >= self.input.len;
}

pub inline fn at(self: @This()) u8 {
    return self.input[self.pos];
}

pub inline fn skip(self: *@This()) void {
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

pub fn eat(self: *@This()) u8 {
    defer self.skip();
    return self.at();
}

fn token(self: *@This(), ttype: TokenType, lexeme: []const u8) void {
    self.current_token = Token{ .type = ttype, .lexeme = lexeme, .line = self.line, .col = self.col, .start = self.start_pos, .width = @truncate(self.pos - self.start_pos) };
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

fn next_num(self: *@This(), start: u8) !void {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    res.append(start) catch return ATError.AllocatorError;
    while (self.is_num()) res.append(self.eat()) catch return ATError.AllocatorError;

    const lexeme = res.toOwnedSlice() catch return ATError.AllocatorError;
    return self.token(.number, lexeme);
}

fn next_string(self: *@This()) !void {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    while (if (!self.is_eof()) self.at() != '"' else false) res.append(self.eat()) catch return ATError.AllocatorError;
    if (self.is_eof()) {
        return ATError.UnterminatedStringLiteral;
    }

    self.skip(); // skips '"'

    const slice = res.toOwnedSlice() catch return ATError.AllocatorError;
    return self.token(.string, slice);
}

fn next_char(self: *@This()) !void {
    const c = self.eat();
    if (self.is_eof()) return ATError.UnterminatedCharLiteral;
    if (self.eat() != '\'') return ATError.UnterminatedCharLiteral;

    return self.token(.char, &[_]u8{c});
}

fn next_ident(self: *@This(), start: u8) !void {
    var res = std.ArrayList(u8).init(c_allocator);
    errdefer res.deinit();

    res.append(start) catch return ATError.AllocatorError;
    while (self.is_allowed_ident()) res.append(self.eat()) catch return ATError.AllocatorError;

    const slice = res.toOwnedSlice() catch return ATError.AllocatorError;
    const ttype = Token.keywords.get(slice) orelse .ident;

    return self.token(ttype, slice);
}

/// generates a token starting from current position in input buffer then sets @This().current_token to it
pub fn next(self: *@This()) ATError!void {
    // skip until we can actually make a token
    while (!self.is_eof()) {
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

    if (self.is_eof()) {
        return self.token(TokenType.eof, "<EOF>");
    }

    self.start_pos = self.pos;

    const ttype = switch (self.eat()) {
        '+' => TokenType.plus,
        '-' => TokenType.minus,
        '*' => TokenType.mul,
        '/' => TokenType.div,

        '>' => if (self.skip_if('=')) TokenType.bigger_then_equal else TokenType.bigger_then,
        '<' => if (self.skip_if('=')) TokenType.smaller_then_equal else TokenType.smaller_then,

        '!' => TokenType.bang,
        '=' => if (self.skip_if('=')) TokenType.equal_equal else TokenType.equal,

        '(' => TokenType.left_paren,
        ')' => TokenType.right_paren,

        '0'...'9' => |d| return self.next_num(d),
        '"' => return self.next_string(),
        '\'' => return self.next_char(),
        'a'...'z', 'A'...'Z', '_' => |c| return self.next_ident(c),
        '#' => {
            while (if (!self.is_eof()) self.at() != '\n' else false) self.skip();
            return self.next();
        },

        else => |_| return ATError.InvaildChar,
    };

    self.token(ttype, self.input[self.start_pos .. self.pos - 1]);
}

pub fn init(input: []u8) !@This() {
    var self = @This(){ .input = input };
    try self.next();

    return self;
}
