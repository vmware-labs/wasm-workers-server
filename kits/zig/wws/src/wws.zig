const std = @import("std");

pub const Method = std.http.Method;
pub const Headers = std.http.Headers;

pub const Request = struct {
    url: []const u8,
    body: []const u8,
    method: Method,
    headers: *Headers,
    storage: *std.StringHashMap([]const u8),
    params: *std.StringHashMap([]const u8),
};

const InputStreamType = enum {
    stdin,
};

const InputStream = union(InputStreamType) {
    stdin: void,
};

const ParseStreamError = error{ OutOfMemory, UnknownError, MalformedRequestObject };

const ParseStreamOptions = struct {
    input_stream: InputStream = .stdin,
};

const ParseStreamResult = struct {
    allocator: std.mem.Allocator,
    request: *Request,

    pub fn destroy(self: *ParseStreamResult) void {
        {
            var it = self.request.params.iterator();
            while (it.next()) |*entry| {
                self.allocator.free(entry.key_ptr.*);
                self.allocator.free(entry.value_ptr.*);
            }
            self.request.params.deinit();
            self.allocator.destroy(self.request.params);
        }

        {
            var it = self.request.storage.iterator();
            while (it.next()) |*entry| {
                self.allocator.free(entry.key_ptr.*);
                self.allocator.free(entry.value_ptr.*);
            }
            self.request.storage.deinit();
            self.allocator.destroy(self.request.storage);
        }

        self.request.headers.deinit();
        self.allocator.destroy(self.request.headers);

        self.allocator.free(self.request.url);
        self.allocator.free(self.request.body);

        self.allocator.destroy(self.request);
    }
};

/// Caller owns the memory
pub fn parseStream(allocator: std.mem.Allocator, options: ParseStreamOptions) ParseStreamError!ParseStreamResult {
    const stream = switch (options.input_stream) {
        .stdin => std.io.getStdIn().reader(),
    };

    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();

    stream.readAllArrayList(&input, std.math.maxInt(usize)) catch return ParseStreamError.UnknownError;

    var json = std.json.parseFromSlice(std.json.Value, allocator, input.items, .{}) catch return ParseStreamError.UnknownError;
    defer json.deinit();

    const r = try allocator.create(Request);

    switch (json.value) {
        .object => |root| {
            switch (root.get("url") orelse return ParseStreamError.MalformedRequestObject) {
                .string => |s| {
                    r.*.url = try allocator.dupe(u8, s);
                },
                else => return ParseStreamError.MalformedRequestObject,
            }

            switch (root.get("method") orelse return ParseStreamError.MalformedRequestObject) {
                .string => |s| {
                    r.*.method = @enumFromInt(Method.parse(s));
                },
                else => return ParseStreamError.MalformedRequestObject,
            }

            switch (root.get("headers") orelse return ParseStreamError.MalformedRequestObject) {
                .object => |o| {
                    const headers = try allocator.create(Headers);
                    headers.* = Headers.init(allocator);
                    errdefer headers.deinit();

                    var it = o.iterator();
                    while (it.next()) |kv| {
                        switch (kv.value_ptr.*) {
                            .string => |v| {
                                try headers.append(kv.key_ptr.*, v);
                            },
                            else => return ParseStreamError.MalformedRequestObject,
                        }
                    }

                    r.*.headers = headers;
                },
                else => return ParseStreamError.MalformedRequestObject,
            }

            switch (root.get("body") orelse return ParseStreamError.MalformedRequestObject) {
                .string => |s| {
                    r.*.body = try allocator.dupe(u8, s);
                },
                else => return ParseStreamError.MalformedRequestObject,
            }

            switch (root.get("kv") orelse return ParseStreamError.MalformedRequestObject) {
                .object => |o| {
                    const storage = try allocator.create(std.StringHashMap([]const u8));
                    storage.* = std.StringHashMap([]const u8).init(allocator);
                    errdefer storage.deinit();

                    var it = o.iterator();
                    while (it.next()) |kv| {
                        switch (kv.value_ptr.*) {
                            .string => |v| {
                                const owned_key = try allocator.dupe(u8, kv.key_ptr.*);
                                errdefer allocator.free(owned_key);

                                const owned_value = try allocator.dupe(u8, v);
                                errdefer allocator.free(owned_value);

                                try storage.put(owned_key, owned_value);
                            },
                            else => return ParseStreamError.MalformedRequestObject,
                        }
                    }

                    r.*.storage = storage;
                },
                else => return ParseStreamError.MalformedRequestObject,
            }

            switch (root.get("params") orelse return ParseStreamError.MalformedRequestObject) {
                .object => |o| {
                    const params = try allocator.create(std.StringHashMap([]const u8));
                    params.* = std.StringHashMap([]const u8).init(allocator);
                    errdefer params.deinit();

                    var it = o.iterator();
                    while (it.next()) |kv| {
                        switch (kv.value_ptr.*) {
                            .string => |v| {
                                const owned_key = try allocator.dupe(u8, kv.key_ptr.*);
                                errdefer allocator.free(owned_key);

                                const owned_value = try allocator.dupe(u8, v);
                                errdefer allocator.free(owned_value);

                                try params.put(owned_key, owned_value);
                            },
                            else => return ParseStreamError.MalformedRequestObject,
                        }
                    }

                    r.*.params = params;
                },
                else => return ParseStreamError.MalformedRequestObject,
            }
        },
        else => return ParseStreamError.MalformedRequestObject,
    }

    return .{
        .allocator = allocator,
        .request = r,
    };
}

const FormatResponseOptions = struct {
    data: []const u8,
    status: usize,
    headers: *Headers,
    storage: *std.StringHashMap([]const u8),
};

const FormatResponseError = error{
    UnknownError,
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

    return buf.toOwnedSlice() catch FormatResponseError.UnknownError;
}
