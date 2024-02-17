const std = @import("std");
const wws = @import("wws");

const tmpl =
    \\The environment variable value is: {s}
;

fn handle(arena: std.mem.Allocator) !wws.Response {
    const envvar = std.process.getEnvVarOwned(arena, "MESSAGE") catch "";

    const body = try std.fmt.allocPrint(arena, tmpl, .{envvar});

    var response = wws.Response{
        .data = body,
    };

    try response.headers.map.put(arena, "x-generated-by", "wasm-workers-server");

    return response;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const response = try handle(arena.allocator());

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
