//! this stage should verify and give a kind to all the nodes
pub const std = @import("std");
pub const c_allocator = std.heap.c_allocator;
pub const AST = @import("AST.zig");
pub const errors = @import("errors.zig");
pub const ATError = errors.ATError;

pub const NodeKind = union(enum) {
    void,
    f32,
    i32,
    str,
    bool,
    char,
    array: *NodeKind,

    pub fn equals(self: *const @This(), other: *const @This()) bool {
        if (@intFromEnum(self.*) != @intFromEnum(other.*)) {
            return false;
        }

        return switch (self.*) {
            .array => |array| {
                return array.equals(other.array);
            },
            else => true,
        };
    }
};

pub const Analyzer = struct {
    symbol_table: std.StringHashMap(NodeKind),
    pub fn init() @This() {
        return .{ .symbol_table = std.StringHashMap(NodeKind).init(c_allocator) };
    }

    pub fn deinit(self: *@This()) void {
        self.symbol_table.deinit();
    }

    /// just declare a verify function, this does some comptime magic
    /// the verify function should only have 2 arguments, the first should take *@This(), the second argument should be of the type of the expr you want to verify
    /// a compile time error would be given if an expr type doesn't have a verify function
    pub fn verify_node(self: *@This(), node: *AST.Node) ATError!NodeKind {
        const info = @typeInfo(@This()).Struct;

        switch (node.expr) {
            inline else => |x| {
                // special program treatment
                if (@TypeOf(x) == AST.Program) {
                    const kind = try self.verify_program(&node.expr.program);
                    node.kind = kind;
                    return kind;
                }

                comptime var found = false;

                inline for (info.decls) |value| {
                    const field = @field(@This(), value.name);

                    const value_info = @typeInfo(@TypeOf(field));

                    switch (value_info) {
                        .Fn => |func| {
                            if (func.params.len < 2) {
                                continue;
                            }

                            if (func.params[1].type == @TypeOf(x) and func.params.len == 2) {
                                found = true;
                                // call stuff is done here >>>
                                const kind = try field(self, x);
                                node.kind = kind;

                                break;
                            }
                        },

                        else => unreachable,
                    }
                }

                if (!found) {
                    @compileError("no verify function found for " ++ @typeName(@TypeOf(x)));
                }

                return node.kind;
            },
        }
    }

    pub fn verify_program(self: *@This(), program: *AST.Program) ATError!NodeKind {
        for (program.body) |node| {
            _ = self.verify_node(node) catch |err| {
                errors.report(err, node.line, node.col);
                program.errored = true;
            };
        }

        return .void;
    }

    pub fn verify_binary_expr(self: *@This(), expr: AST.BinaryExpr) ATError!NodeKind {
        const left = try self.verify_node(expr.left);
        const right = try self.verify_node(expr.right);

        if (!left.equals(&right)) {
            return error.TypeMismatch;
        }
        return left;
    }

    pub fn verify_unary_expr(self: *@This(), expr: AST.UnaryExpr) ATError!NodeKind {
        const inside = try self.verify_node(expr.expr);

        if ((inside == .bool and expr.operator == .bang) or
            ((inside == .f32 or inside == .i32) and (expr.operator == .plus or expr.operator == .minus)))
        {
            return inside;
        }

        return error.InvaildType;
    }

    pub fn verify_let_decl(self: *@This(), expr: AST.LetDecl) ATError!NodeKind {
        const inside = try self.verify_node(expr.expr);
        self.symbol_table.put(expr.name.ident, inside) catch return ATError.AllocatorError;

        return .void;
    }

    pub fn verify_ident(self: *@This(), expr: AST.Ident) ATError!NodeKind {
        const kind = self.symbol_table.get(expr.ident) orelse error.UndeclaredVariable;
        return kind;
    }

    pub fn verify_literal(_: *@This(), expr: AST.Literal) ATError!NodeKind {
        return switch (expr) {
            .int => .i32,
            .float => .f32,
            .str => .str,
            .bool => .bool,
            .char => .char,
        };
    }
};
