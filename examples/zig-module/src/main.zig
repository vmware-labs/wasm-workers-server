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

    const stdin = std.io.getStdIn();
    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();
    try stdin.reader().readAllArrayList(&input, std.math.maxInt(usize));

    std.debug.print("{s}\n", .{input.items});

    var result = try wws.parseStream(allocator, .{ .input_stream = .{ .bytes = input.items } });
    defer result.deinit();
    const wws_request = result.value;
    std.debug.print("{s} {any}\n", .{ wws_request.url, wws_request.method });

    // Prepare response
    var body = std.ArrayList(u8).init(allocator);
    defer body.deinit();

    try body.writer().print("{any} {s}\n", .{ wws_request.method, wws_request.url });

    {
        var it = wws_request.kv.map.iterator();
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

    var counter: usize = if (wws_request.kv.map.get("counter")) |v| try std.fmt.parseInt(usize, v, 10) else 0;

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
