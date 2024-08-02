const std = @import("std");

const Token = @import("Token.zig");
const TokenType = Token.TokenType;

const AST = @import("AST.zig");
const Tokenizer = @import("Tokenizer.zig");

tokenizer: Tokenizer,
prev_token: Token = undefined,
line: u16 = 1,
col: u16 = 1,
start: usize = 0,

/// makes an AST.Node from one of the struct that the tagged union AST.Expr takes, just pass that struct as val
pub fn make_node(self: @This(), val: anytype) AST.Node {
    const field = comptime f: {
        const fields = @typeInfo(AST.Expr).Union.fields;

        for (fields) |field| {
            if (field.type == @TypeOf(val)) {
                break :f field.name;
            }
        }
    };

    const node = AST.Node{ .line = self.line, .col = self.col, .width = self.tokenizer.pos - self.start, .expr = @unionInit(AST.Expr, field, val) };
    return node;
}

pub fn init(input: []u8) !@This() {
    var parser = @This(){ .tokenizer = try Tokenizer.init(input) };
    parser.prev_token = parser.tokenizer.current_token;
    return parser;
}

/// takes a look at current token
inline fn peek(self: @This()) Token {
    return self.tokenizer.current_token;
}

/// takes a look at prev token
inline fn peek_prev(self: @This()) Token {
    return self.prev_token;
}

/// advances tokenizer (Tokenizer.next) and returns previous token
fn eat(self: *@This()) !Token {
    try self.advance();
    return self.peek_prev();
}
/// advances tokenizer without returning anything
fn advance(self: *@This()) !void {
    const prev = self.peek();
    try self.tokenizer.next();

    self.prev_token = prev;
    return prev;
}

/// advances tokenizer (Tokenizer.next) only if Token.type(TokenType) is one of the expected matches and returns true otherwise returns false, doesn't respect union values it converts expections to int (@intFromEnum) and checks them as that
fn eat_match(self: *@This(), expections: []TokenType) bool {
    for (expections) |expection| {
        if (@intFromEnum(expection) == @intFromEnum(self.peek().type)) {
            try self.advance();
            return true;
        }
    }

    return false;
}

inline fn is_eof(self: @This()) bool {
    return self.tokenizer.is_eof();
}

pub fn parse_expression(self: @This()) !AST.Node {
    return self.parse_literal();
}

fn parse_literal(self: @This()) !AST.Node {
    return switch (self.peek().type) {
        .string => |val| self.make_node(AST.StrLiteral{ .val = val }),
        .number => |literal| {
            var is_float = false;

            for (literal) |char| {
                if (char == '.') {
                    is_float = true;
                    break;
                }
            }

            if (is_float) {
                const float = std.fmt.parseFloat(f32, literal) catch unreachable;
                return self.make_node(AST.FloatLiteral{ .val = float });
            } else {
                const int = std.fmt.parseInt(u32, literal, 10) catch unreachable;
                return self.make_node(AST.IntLiteral{ .val = int });
            }
        },
        else => error.UnexpectedToken,
    };
}
