const std = @import("std");
const NodeKind = @import("analysis.zig").NodeKind;

const Token = @import("Token.zig");

pub const Block = struct { body: []*Node };
pub const Ident = struct { ident: []const u8 };

pub const LetDecl = struct { name: Ident, expr: *Node };
pub const UnaryExpr = struct { operator: Token.TokenType, expr: *Node };
pub const BinaryExpr = struct { left: *Node, right: *Node, operator: Token.TokenType };

pub const Literal = union(enum) { int: u32, float: f32, str: []const u8, bool: bool, char: u8 };

pub const Program = struct { body: []*Node, errored: bool };

pub const Expr = union(enum) {
    program: Program,
    let_decl: LetDecl,
    unary_expr: UnaryExpr,
    binary_expr: BinaryExpr,
    literal: Literal,
    Ident: Ident,
};

pub const Node = struct {
    expr: Expr,

    line: u16 = 0,
    col: u16 = 0,
    width: usize = 0,
    kind: NodeKind = undefined,

    fn appendIdent(buf: *std.ArrayList(u8), ident: *u32) !void {
        var i: u32 = 0;
        while (i != ident.*) : (i += 1) {
            try buf.append(' ');
        }
    }

    fn append(buf: *std.ArrayList(u8), slice: []const u8, ident: *u32) !void {
        try appendIdent(buf, ident);
        try buf.appendSlice(slice);
    }

    /// pretty much a mess that converts A node to a string for debugging purposes
    fn to_str_inner(this: @This(), ident: *u32) ![]const u8 {
        var str = std.ArrayList(u8).init(std.heap.c_allocator);
        errdefer str.deinit();

        switch (this.expr) {
            inline else => |x| {
                const ty = @TypeOf(x);

                try append(&str, @typeName(ty) ++ ":\n", ident);
                ident.* += 1;

                inline for (std.meta.fields(ty), 0..) |field, index| {
                    blk: {
                        // dont continue this block if x is a union and field is not active
                        switch (@typeInfo(ty)) {
                            .Union => {
                                if (index != @intFromEnum(x)) {
                                    break :blk;
                                }
                            },
                            else => {},
                        }

                        const value = @field(x, field.name);
                        try append(&str, field.name ++ ":\n", ident);
                        ident.* += 1;

                        switch (field.type) {
                            *const Node, *Node => {
                                const slice = try value.to_str_inner(ident);

                                // ident already aplied on to_str_inner no need to re-apply
                                try str.appendSlice(slice);
                            },

                            []const u8, []u8 => {
                                try appendIdent(&str, ident);
                                try std.fmt.format(str.writer(), "\"{s}\"\n", .{value});
                            },

                            []*const Node, []*Node => {
                                for (value) |node| {
                                    const results = try node.to_str_inner(ident);
                                    try str.appendSlice(results);
                                }
                            },

                            else => {
                                try appendIdent(&str, ident);
                                try std.fmt.format(str.writer(), "{}\n", .{value});
                            },
                        }
                        ident.* -= 1;
                    }
                }
            },
        }

        try appendIdent(&str, ident);
        try std.fmt.format(str.writer(), "kind: {}\n", .{this.kind});

        ident.* -= 1;

        return try str.toOwnedSlice();
    }

    /// warpper around `to_str_inner` that converts a Node into a string from scratch
    pub fn to_str(this: @This()) ![]const u8 {
        var ident: u32 = 0;
        return this.to_str_inner(&ident);
    }

    /// prints the node to stderr after converting it into a str using `to_str`
    pub fn print(this: @This()) !void {
        const str = try this.to_str();

        std.debug.print("{s}\n", .{str});
    }

    /// init a ptr to Node
    pub fn init() !*@This() {
        return try std.heap.c_allocator.create(@This());
    }

    /// deinits a ptr to Node, also deinits any other child Nodes
    pub fn deinit(this: *const @This()) void {
        switch (this.expr) {
            // deinits child nodes
            inline else => |x| {
                inline for (std.meta.fields(@TypeOf(x)), 0..) |field, index| {
                    blk: {
                        // dont continue this block if x is a union and field is not active
                        switch (@typeInfo(@TypeOf(x))) {
                            .Union => {
                                if (index != @intFromEnum(x)) {
                                    break :blk;
                                }
                            },
                            else => {},
                        }

                        switch (field.type) {
                            *const Node, *Node => {
                                @field(x, field.name).deinit();
                            },
                            []*const Node, []*Node => {
                                for (@field(x, field.name)) |node| {
                                    node.deinit();
                                }
                            },

                            else => {},
                        }
                    }
                }
            },
        }

        std.heap.c_allocator.destroy(this);
    }
};
