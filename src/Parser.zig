const std = @import("std");

const Token = @import("Token.zig");
const TokenType = Token.TokenType;

const AST = @import("AST.zig");
const Tokenizer = @import("Tokenizer.zig");
const errors = @import("errors.zig");
const ATError = errors.ATError;

tokenizer: Tokenizer,
prev_token: Token = undefined,

pub fn line(self: *const @This()) u16 {
    return self.tokenizer.line;
}

pub fn col(self: *const @This()) u16 {
    return self.tokenizer.col;
}

pub fn start(self: *const @This()) usize {
    return self.tokenizer.start_pos;
}

pub fn pos(self: *const @This()) usize {
    return self.tokenizer.pos;
}

/// makes an AST.Node from one of the struct that the tagged union AST.Expr takes, just pass that struct as val
pub fn make_node(self: @This(), val: anytype) !*AST.Node {
    const field = comptime f: {
        const fields = @typeInfo(AST.Expr).Union.fields;

        for (fields) |field| {
            if (field.type == @TypeOf(val)) {
                break :f field.name;
            }
        }
    };

    const node = AST.Node{ .line = self.line(), .col = self.col(), .width = self.pos() - self.start(), .expr = @unionInit(AST.Expr, field, val) };

    const ptr = std.heap.c_allocator.create(AST.Node) catch return ATError.AllocatorError;
    ptr.* = node;
    return ptr;
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
}

/// advances tokenizer (Tokenizer.next) only if Token.type(TokenType) is one of the expected matches and returns true otherwise returns false, doesn't respect union values it converts expections to int (@intFromEnum) and checks them as that
fn eat_match(self: *@This(), expections: []const TokenType) !bool {
    for (expections) |expection| {
        if (@intFromEnum(expection) == @intFromEnum(self.peek().type)) {
            try self.advance();
            return true;
        }
    }

    return false;
}

fn expect(self: *@This(), token: TokenType) !void {
    const ate = try self.eat_match(&[1]TokenType{token});
    if (!ate) {
        return error.UnexpectedToken;
    }
}

inline fn is_eof(self: @This()) bool {
    return self.tokenizer.is_eof();
}

/// parses the whole file
/// returns a `Node` with `expr` of `Program`
pub fn parse_program(self: *@This()) !*AST.Node {
    var list = std.ArrayList(*AST.Node).init(std.heap.c_allocator);
    errdefer list.deinit();
    var errored = false;

    while (!self.is_eof()) {
        // start parsing again after the first newline
        const expr = self.parse_expression() catch |err| {
            errored = true;
            errors.report(err, self.peek_prev().line, self.peek_prev().col);
            continue;
        };

        try list.append(expr);
    }

    const program = AST.Program{ .body = try list.toOwnedSlice(), .errored = errored };
    return self.make_node(program);
}

pub fn parse_expression(self: *@This()) ATError!*AST.Node {
    return self.parse_coordinative_expression();
}

pub fn parse_coordinative_expression(self: *@This()) !*AST.Node {
    var expr = try self.parse_equality_expression();

    while (try self.eat_match(&[_]TokenType{ .and_kw, .or_kw })) {
        const operator = self.peek_prev().type;
        const right = try self.parse_equality_expression();

        expr = try self.make_node(AST.BinaryExpr{ .left = expr, .right = right, .operator = operator });
    }
    return expr;
}

pub fn parse_equality_expression(self: *@This()) ATError!*AST.Node {
    var expr = try self.parse_comparative_expression();

    while (try self.eat_match(&[_]TokenType{.equal_equal})) {
        const operator = self.peek_prev().type;
        const right = try self.parse_comparative_expression();

        expr = try self.make_node(AST.BinaryExpr{ .left = expr, .right = right, .operator = operator });
    }
    return expr;
}

pub fn parse_comparative_expression(self: *@This()) ATError!*AST.Node {
    var expr = try self.parse_additive_expression();

    while (try self.eat_match(&[_]TokenType{ .smaller_then, .bigger_then, .smaller_then_equal, .bigger_then_equal })) {
        const operator = self.peek_prev().type;
        const right = try self.parse_additive_expression();

        expr = try self.make_node(AST.BinaryExpr{ .left = expr, .right = right, .operator = operator });
    }
    return expr;
}

pub fn parse_additive_expression(self: *@This()) ATError!*AST.Node {
    var expr = try self.parse_multipactive_expression();

    while (try self.eat_match(&[_]TokenType{ .plus, .minus })) {
        const operator = self.peek_prev().type;
        const right = try self.parse_multipactive_expression();

        expr = try self.make_node(AST.BinaryExpr{ .left = expr, .right = right, .operator = operator });
    }
    return expr;
}

pub fn parse_multipactive_expression(self: *@This()) ATError!*AST.Node {
    var expr = try self.parse_unary_expression();

    while (try self.eat_match(&[_]TokenType{ .mul, .div })) {
        const operator = self.peek_prev().type;
        const right = try self.parse_unary_expression();

        expr = try self.make_node(AST.BinaryExpr{ .left = expr, .right = right, .operator = operator });
    }
    return expr;
}

pub fn parse_unary_expression(self: *@This()) ATError!*AST.Node {
    if (try self.eat_match(&[_]TokenType{ .minus, .bang })) {
        const operator = self.peek_prev().type;
        const expr = try self.parse_unary_expression();

        return self.make_node(AST.UnaryExpr{ .operator = operator, .expr = expr });
    }

    return self.parse_primary_expression();
}

pub fn parse_primary_expression(self: *@This()) ATError!*AST.Node {
    return switch (self.peek().type) {
        .left_paren => {
            _ = try self.eat();
            const expr = try self.parse_expression();

            try self.expect(TokenType.right_paren);
            return expr;
        },
        else => self.parse_literal(),
    };
}

fn parse_literal(self: *@This()) ATError!*AST.Node {
    return switch ((try self.eat()).type) {
        .string => |val| self.make_node(AST.Literal{ .str = val }),
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
                return self.make_node(AST.Literal{ .float = float });
            } else {
                const int = std.fmt.parseInt(u32, literal, 10) catch unreachable;
                return self.make_node(AST.Literal{ .int = int });
            }
        },

        .false_kw => self.make_node(AST.Literal{ .bool = false }),
        .true_kw => self.make_node(AST.Literal{ .bool = true }),
        else => ATError.UnexpectedToken,
    };
}
