const std = @import("std");
const io = std.io;
const http = std.http;

// This is from ChatGPT - I have no clue whether this works, nor whats going on here :D
fn isValidUtf8(data: []const u8) bool {
    var i: usize = 0;
    while (i < data.len) {
        const byte: u8 = data[i];
        if (byte < 0x80) {
            // ASCII character
            i += 1;
        } else if (byte < 0xC2) {
            // Invalid continuation byte
            return false;
        } else if (byte < 0xE0) {
            // 2-byte sequence
            if ((i + 1 >= data.len) or ((data[i + 1] & 0xC0) != 0x80)) {
                return false;
            }
            i += 2;
        } else if (byte < 0xF0) {
            // 3-byte sequence
            if ((i + 2 >= data.len) or ((data[i + 1] & 0xC0) != 0x80) or ((data[i + 2] & 0xC0) != 0x80)) {
                return false;
            }
            i += 3;
        } else if (byte < 0xF5) {
            // 4-byte sequence
            if ((i + 3 >= data.len) or ((data[i + 1] & 0xC0) != 0x80) or ((data[i + 2] & 0xC0) != 0x80) or ((data[i + 3] & 0xC0) != 0x80)) {
                return false;
            }
            i += 4;
        } else {
            // Invalid UTF-8 byte
            return false;
        }
    }
    return true;
}

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

pub var cache = std.StringHashMap([]const u8).init(allocator);
pub var params = std.StringHashMap([]const u8).init(allocator);

pub const Input = struct {
    url: []const u8,
    method: []const u8,
    headers: std.StringArrayHashMap([]const u8),
    body: []const u8,
};

pub const Output = struct {
	data: []const u8,
	headers: std.StringArrayHashMap([]const u8),
	status:  u16,
	base64:  bool,

	httpHeader: http.Headers,

    const Self = @This();

    pub fn init() Self {
        return .{
            .data = "",
            .headers =  std.StringArrayHashMap([]const u8).init(allocator),
            .status = 0,
            .base64 = false,
            .httpHeader = http.Headers.init(allocator),
        };
    }
    
    pub fn header(self: *Self) http.Headers {
        if (self.httpHeader == undefined) {
            self.httpHeader = http.Headers.init(allocator);
        }

        return self.httpHeader;
    }

    pub fn writeHeader(self: *Self, statusCode: u16) void {
        self.status = statusCode;
    }

    pub fn write(self: *Self, data: []const u8) !u32 {

        if (isValidUtf8(data)) {
            self.data = data;
        } else {
            self.base64 = true;
            // is this correct?
            const enc = std.base64.Base64Encoder.init(std.base64.url_safe_alphabet_chars, '=');
            var dest: []u8 = undefined;
            self.data = std.base64.Base64Encoder.encode(&enc, dest, data);
        }

        if (self.status == 0) {
            self.status = 200;
        }

        for (self.httpHeader.list.items) |item| {
            try self.headers.put(item.name, item.value);
        }

        // prepare writer for json
        var out_buf: [1024]u8 = undefined;
        var slice_stream = std.io.fixedBufferStream(&out_buf);
        const out = slice_stream.writer();
        var w = std.json.writeStream(out, .{ .whitespace = .minified });

        slice_stream.reset();
        try w.beginObject();

        try w.objectField("data");
        try w.write(self.data);

        try w.objectField("status");
        try w.write(self.status);

        try w.objectField("base64");
        try w.write(self.base64);

        try w.objectField("headers");
        try w.write(try getHeadersJsonObject(self.headers));

        try cache.put("hello", "world");
        try w.objectField("kv");
        try w.write(try getCacheJsonObject(cache));

        try w.endObject();
        const result = slice_stream.getWritten();

        std.debug.print("output json: {s}\n", .{ result });

        // I assume this works, as no stdout seems to be logged by wws, only stderr
        // https://zig.news/kristoff/where-is-print-in-zig-57e9
        const stdout = std.io.getStdOut().writer();
        try stdout.print("{s}", .{result});

        return self.data.len;
    }
};

fn getHeadersJsonObject(s: std.StringArrayHashMap([]const u8)) !std.json.Value {
    var value = std.json.Value{ .object = std.json.ObjectMap.init(allocator) };

    var i = s.iterator();
    while (i.next()) |kv| {
        try value.object.put(kv.key_ptr.*, std.json.Value{ .string = kv.value_ptr.*});
    }

    return value;
}

fn getCacheJsonObject(s: std.StringHashMap([]const u8)) !std.json.Value {
    var value = std.json.Value{ .object = std.json.ObjectMap.init(allocator) };

    var i = s.iterator();
    while (i.next()) |kv| {
        try value.object.put(kv.key_ptr.*, std.json.Value{ .string = kv.value_ptr.*});
    }

    return value;
}

pub fn readInput() !Input {
    // https://www.openmymind.net/Reading-A-Json-Config-In-Zig/
    const in = std.io.getStdIn();
    var buf = std.io.bufferedReader(in.reader());
    var r = buf.reader();

    var msg_buf: [4096]u8 = undefined;

    // delimiter "\n" might not be adequate?
    if (r.readUntilDelimiterOrEof(&msg_buf, '\n')) |msg| {
        if (msg) | m | {
            std.debug.print("raw input json: {s}\n", .{m});
            return getInput(m);
        }
    } else |err| {
        std.debug.print("error parsing json: {!}\n", .{err});
    }

    // TODO: proper return value
    return undefined;
}

fn getInput(s: []const u8) !Input {
    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, s, .{});
    defer parsed.deinit();

    var input = Input{
        .url = parsed.value.object.get("url").?.string,
        .method = parsed.value.object.get("method").?.string,
        .body = parsed.value.object.get("body").?.string,
        .headers = std.StringArrayHashMap([]const u8).init(allocator),
    };

    var headers_map = parsed.value.object.get("headers").?.object;

    // can we maybe use an iterator here?
    // var i = headers_map.iterator();
    // while (i.next()) |kv| {
    //     try input.headers.put(kv.key_ptr.*.string, kv.value_ptr.*.string);
    // }

    // std.debug.print("headers1: {!}", .{ input.headers });

    for (headers_map.keys()) |key| {
        var v = try headers_map.getOrPut(key);
        if (v.found_existing) {
            var value = v.value_ptr.*.string;
            // std.debug.print("headers key: {s}, value: {s}\n", .{key, value});
            try input.headers.put(key, value);
        }
    }
    
    return input;
}

pub fn createRequest(in: *Input) !http.Client.Request {

    // Create an HTTP client.
    var client = http.Client{ .allocator = allocator };
    // Release all associated resources with the client.
    defer client.deinit();
    // Parse the URI.

    var req = http.Client.Request{
        .client = &client,
        .uri = try std.Uri.parseWithoutScheme(in.url),
        .method = http.Method.GET,
        .arena = arena,
        .connection = null,
        .headers = http.Headers.init(allocator),
        .redirects_left = 0,
        .handle_redirects = false,
        .response = undefined,
    };

    // is it even necessary to copy headers from Input to Request struct?
    var i = in.headers.iterator();
    while (i.next()) |kv| {
        try req.headers.append(kv.key_ptr.*, kv.value_ptr.*);
    }

    // req = req.WithContext(context.WithValue(req.Context(), CacheKey, cache));
	// req = req.WithContext(context.WithValue(req.Context(), ParamsKey, params));

    return req;
}

const RequestAndOutput = struct {
    req: http.Client.Request,
    output: Output,
};

pub fn getWriterRequest() !RequestAndOutput {

    var in = readInput() catch |err| {
        std.debug.print("error reading input: {!}\n", .{err});
        return std.os.exit(1);
    };

    var req = createRequest(&in) catch |err| {
        std.debug.print("error creating request : {!}\n", .{err});
        return std.os.exit(1);
    };

    var output = Output.init();

    return RequestAndOutput{
        .req = req,
        .output = output,
    };
}

pub const Response = struct {
    data: []const u8,
    httpHeader: http.Headers,

    pub fn writeAll(self: *Response, data: []const u8) !u32 {
        self.data = data;
        return self.data.len;
    }
};

// Function parameter as function pointer
pub fn ServeFunc(requestFn: *const fn (*Response, *http.Client.Request) void) void {
    var r = try getWriterRequest();
    var request = r.req;
    var output = r.output;

    var response = Response{ .data = "", .httpHeader = http.Headers.init(allocator) };
    requestFn(&response, &request);

    // TODO: do we need request.headers in response?
    // output.httpHeader = request.headers;
    output.httpHeader = response.httpHeader;

    _ = output.write(response.data) catch |err| {
        std.debug.print("error writing data: {!} \n", .{ err });
    };

    std.debug.print("Done.\n", .{ });
}


// Alternative ways for function parameter
// pub fn ServeFunc(requestFn: anytype) void {
//     requestFn();
// }

// works as function budy, must be comptime-known
// pub fn ServeFunc(comptime requestFn: fn () void) void {
//     requestFn();
// }
