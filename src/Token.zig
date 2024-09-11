const std = @import("std");

type: TokenType,
lexeme: []const u8,
start: usize,
width: u16,
line: u16,
col: u16,

pub const TokenType = enum {
    plus,
    minus,
    bang,
    mul,
    div,

    bigger_then,
    smaller_then,
    bigger_then_equal,
    smaller_then_equal,

    left_paren,
    right_paren,

    equal,
    equal_equal,

    // literals
    number,
    char,
    string,

    ident,

    // keywords
    let_kw,
    and_kw,
    or_kw,
    fn_kw,

    true_kw,
    false_kw,

    eof,
};

// some comptime magic that creates a slice of .{(any TokenType variant that ends with _kw name without _kw), (that variant)}
fn get_keywords() ![]struct { []const u8, TokenType } {
    const fields = comptime @typeInfo(TokenType).Enum.fields;
    comptime var results: [fields.len]struct { []const u8, TokenType } = .{undefined} ** fields.len;
    comptime var i = 0;

    for (fields) |field| {
        if (std.mem.eql(u8, field.name[field.name.len - 3 ..], "_kw")) {
            results[i] = .{ field.name[0 .. field.name.len - 3], @field(TokenType, field.name) };
            i += 1;
        }
    }

    return results[0..i];
}

pub const keywords = std.StaticStringMap(TokenType).initComptime(get_keywords() catch {});
