const std = @import("std");
const wws = @import("wws");

const tmpl =
    \\Hey! The parameter is: {s}
;

fn handle(arena: std.mem.Allocator, request: wws.Request) !wws.Response {
    const id = request.params.map.get("id") orelse "the value is not available";

    const body = try std.fmt.allocPrint(arena, tmpl, .{id});

    var response = wws.Response{
        .data = body,
    };

    try response.headers.map.put(arena, "x-generated-by", "wasm-workers-server");

    return response;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const parse_result = try wws.parseStream(gpa.allocator(), .{});
    defer parse_result.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const request = parse_result.value;
    const response = try handle(arena.allocator(), request);

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
