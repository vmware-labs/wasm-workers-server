const std = @import("std");

pub const Method = std.http.Method;

pub const Request = struct {
    url: []const u8,
    method: Method,
    body: []const u8,
    headers: std.json.ArrayHashMap([]const u8),
    kv: std.json.ArrayHashMap([]const u8),
    params: std.json.ArrayHashMap([]const u8),
};

pub const Response = struct {
    headers: std.json.ArrayHashMap([]const u8) = .{},
    data: []const u8 = "",
    // TODO Use std.http.Status when Response has its own
    // json serialization helper functions
    status: usize = 200,
    kv: std.json.ArrayHashMap([]const u8) = .{},
};

const InputStreamType = enum {
    stdin,
    bytes,
};

const InputStream = union(InputStreamType) {
    stdin: void,
    bytes: []const u8,
};

const ParseStreamError = error{Unknown};

const ParseStreamOptions = struct {
    input_stream: InputStream = .stdin,
};

/// Caller must call deinit on the returned result to free the associated memory
pub inline fn parseStream(allocator: std.mem.Allocator, options: ParseStreamOptions) ParseStreamError!std.json.Parsed(Request) {
    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();

    switch (options.input_stream) {
        .stdin => {
            const stdin = std.io.getStdIn().reader();
            stdin.readAllArrayList(&input, std.math.maxInt(usize)) catch return ParseStreamError.Unknown;
        },
        .bytes => |s| {
            input.appendSlice(s) catch return ParseStreamError.Unknown;
        },
    }

    return std.json.parseFromSlice(
        Request,
        allocator,
        input.items,
        .{ .allocate = .alloc_always },
    ) catch ParseStreamError.Unknown;
}

pub inline fn writeResponse(response: Response, writer: anytype) !void {
    try std.json.stringify(response, .{}, writer);
}
