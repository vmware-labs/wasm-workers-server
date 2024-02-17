const std = @import("std");
const wws = @import("wws");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Read request from stdin
    const parse_result = try wws.parseStream(allocator, .{});
    defer parse_result.deinit();

    const request = parse_result.value;

    // Simple echo response
    const response = wws.Response{
        .data = request.url,
    };

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
