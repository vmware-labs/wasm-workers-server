const std = @import("std");
const wws = @import("wws");

fn handle(arena: std.mem.Allocator) !wws.Response {
    const file = try std.fs.Dir.openFile(std.fs.cwd(), "zig.svg", .{
        .mode = std.fs.File.OpenMode.read_only,
        .lock = std.fs.File.Lock.none,
    });
    defer file.close();

    const mb = (1 << 10) << 10;
    const file_contents = try file.readToEndAlloc(arena, mb);

    var response = wws.Response{
        .data = file_contents,
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
