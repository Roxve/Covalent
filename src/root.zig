const std = @import("std");
const Tokenizer = @import("Tokenizer.zig");
const AST = @import("AST.zig");
const Token = @import("Token.zig");
const Parser = @import("Parser.zig");

pub fn run(input: []u8) !void {
    var parser = try Parser.init(input);
    const node = try parser.parse_expression();

    try node.print();
    node.deinit();
}
