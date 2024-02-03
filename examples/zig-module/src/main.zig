const std = @import("std");
const wws = @import("wws");

/// Receives an arena allocator such that all allocations made while handling
/// the request can be easily freed by the caller
fn handle(arena: *std.heap.ArenaAllocator, request: *const wws.Request) !wws.Response {
    const allocator = arena.allocator();

    // Prepare response body
    var body = std.ArrayList(u8).init(allocator);
    defer body.deinit();

    try body.writer().print("{any} {s}\n", .{ request.method, request.url });

    {
        var it = request.kv.map.iterator();
        while (it.next()) |entry| {
            try body.writer().print("kv.{s}: {s}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
    }

    var counter: usize = if (request.kv.map.get("counter")) |v| try std.fmt.parseInt(usize, v, 10) else 0;

    // Increment counter, save the result to storage
    counter += 1;

    // Prepare response
    var response = wws.Response{
        .data = try body.toOwnedSlice(),
    };

    try response.headers.map.put(allocator, "Content-Type", "text/plain");
    {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();

        try buf.writer().print("{d}", .{counter});
        try response.kv.map.put(allocator, "counter", try buf.toOwnedSlice());
    }

    return response;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        switch (gpa.deinit()) {
            .ok => {},
            .leak => {},
        }
    }
    const allocator = gpa.allocator();

    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();

    const parse_result = try wws.parseStream(allocator, .{});
    defer parse_result.deinit();

    const response = try handle(&arena, &parse_result.value);

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
