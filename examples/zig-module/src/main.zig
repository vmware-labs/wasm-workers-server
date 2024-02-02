const std = @import("std");
const wws = @import("wws");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        switch (gpa.deinit()) {
            .ok => {},
            .leak => {},
        }
    }
    const allocator = gpa.allocator();

    // Parse request from stdin
    var event = try wws.parseStream(allocator, .{});
    defer event.destroy();
    const wws_request = event.request;

    // Prepare response
    var body = std.ArrayList(u8).init(allocator);
    defer body.deinit();

    try body.writer().print("{any} {s}\n", .{ wws_request.method, wws_request.url });

    {
        var it = wws_request.storage.iterator();
        while (it.next()) |entry| {
            try body.writer().print("kv.{s}: {s}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
    }

    var headers = wws.Headers.init(allocator);
    defer headers.deinit();
    try headers.append("Content-Type", "text/plain");

    var storage = std.StringHashMap([]const u8).init(allocator);
    defer storage.deinit();
    defer {
        var it = storage.iterator();
        while (it.next()) |entry| {
            allocator.free(entry.value_ptr.*);
        }
    }

    var counter: usize = if (wws_request.storage.get("counter")) |v| try std.fmt.parseInt(usize, v, 10) else 0;

    // Increment counter, save the result to storage
    counter += 1;

    {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        try buf.writer().print("{d}", .{counter});
        try storage.put("counter", try buf.toOwnedSlice());
    }

    const response = try wws.formatResponse(allocator, .{
        .data = body.items,
        .status = 200,
        .headers = &headers,
        .storage = &storage,
    });
    defer allocator.free(response);

    const stdout = std.io.getStdOut();
    try stdout.writer().print("{s}", .{response});
}
