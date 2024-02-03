const std = @import("std");

pub const Method = std.http.Method;
pub const Headers = std.http.Headers;

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

const Request = struct {
    url: []const u8,
    method: Method,
    body: []const u8,
    headers: std.json.ArrayHashMap([]const u8),
    kv: std.json.ArrayHashMap([]const u8),
    params: std.json.ArrayHashMap([]const u8),
};

/// Caller owns the memory
pub fn parseStream(allocator: std.mem.Allocator, options: ParseStreamOptions) ParseStreamError!std.json.Parsed(Request) {
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

const FormatResponseOptions = struct {
    data: []const u8,
    status: usize,
    headers: *Headers,
    storage: *std.StringHashMap([]const u8),
};

const FormatResponseError = error{
    Unknown,
    OutOfMemory,
};

pub fn formatResponse(allocator: std.mem.Allocator, options: FormatResponseOptions) FormatResponseError![]const u8 {
    var buf = std.ArrayList(u8).init(allocator);
    defer buf.deinit();

    var w = std.json.writeStream(buf.writer(), .{ .whitespace = .minified });
    {
        try w.beginObject();

        try w.objectField("data");
        try w.write(std.json.Value{ .string = options.data });

        try w.objectField("status");
        try w.write(options.status);

        {
            var o = std.json.ObjectMap.init(allocator);
            defer o.deinit();

            for (options.headers.list.items) |entry| {
                try o.put(entry.name, .{ .string = entry.value });
            }

            try w.objectField("headers");
            try w.write(std.json.Value{ .object = o });
        }

        {
            var o = std.json.ObjectMap.init(allocator);
            defer o.deinit();

            var it = options.storage.iterator();
            while (it.next()) |entry| {
                try o.put(entry.key_ptr.*, .{ .string = entry.value_ptr.* });
            }

            try w.objectField("kv");
            try w.write(std.json.Value{ .object = o });
        }

        try w.endObject();
    }

    return buf.toOwnedSlice() catch FormatResponseError.Unknown;
}
