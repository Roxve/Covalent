const std = @import("std");
const Token = @import("Token.zig");

pub const BinaryExpr = struct { left: *const Node, right: *const Node, operator: Token.TokenType };

pub const IntLiteral = struct { val: u32 };
pub const FloatLiteral = struct { val: f32 };
pub const StrLiteral = struct { val: []const u8 };

pub const Expr = union(enum) {
    BinaryExpr: BinaryExpr,

    IntLiteral: IntLiteral,
    FloatLiteral: FloatLiteral,
    StrLiteral: StrLiteral,
};

pub const Node = struct {
    expr: Expr,

    line: u16 = 0,
    col: u16 = 0,
    width: usize = 0,

    /// pretty much a mess that converts A node to a string for debugging purposes
    pub fn visit_str(this: @This()) ![][]const u8 {
        var str = std.ArrayList([]const u8).init(std.heap.c_allocator);
        errdefer str.deinit();

        switch (this.expr) {
            inline else => |e| {
                const ty = @TypeOf(e);
                const name = @typeName(ty);
                try str.append((name ++ ":"));

                const info = @typeInfo(ty).Struct;
                const fields = info.fields;

                inline for (fields) |field| {
                    const value = @field(e, field.name);
                    const f_info = @typeInfo(field.type);

                    try str.append("\t");
                    try str.append(field.name ++ ":");
                    try str.append("\t");

                    switch (field.type) {
                        *const Node => {
                            const visited = try value.visit_str();
                            for (visited) |line| {
                                try str.append("\t");
                                try str.append(line);
                            }
                        },

                        u32, f32 => {
                            var buf: [20]u8 = undefined;

                            const parsed = try std.fmt.bufPrint(&buf, "{}", .{value});
                            try str.append("\t");
                            try str.append(try std.mem.Allocator.dupe(std.heap.c_allocator, u8, parsed));
                        },

                        []u8, []const u8 => {
                            try str.append("\t");
                            try str.append(value);
                        },

                        else => {
                            switch (f_info) {
                                .Union => {
                                    const un = f_info.Union;
                                    const union_fields = un.fields;

                                    switch (value) {
                                        inline else => {
                                            const expected_index = @intFromEnum(value);

                                            inline for (union_fields, 0..) |union_field, index| {
                                                if (index == expected_index) {
                                                    try str.append("\t");
                                                    try str.append(@typeName(field.type) ++ "." ++ union_field.name);
                                                    break;
                                                }
                                            }
                                        },
                                    }
                                },
                                else => {},
                            }
                        },
                    }
                }
            },
        }

        return try str.toOwnedSlice();
    }

    /// prints the node after visiting it using this.visit_str(), treats "\t" as one space
    pub fn print(this: @This()) !void {
        const str = try this.visit_str();

        for (str) |line| {
            if (std.mem.eql(u8, line, "\t")) {
                std.debug.print(" ", .{});
                continue;
            }

            std.debug.print("{s}\n", .{line});
        }
    }
};
