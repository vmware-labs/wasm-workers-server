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

        if (true) {
        // if (isValidUtf8(data)) {
            self.data = data;
        } else {
            self.base64 = true;
            // is this correct?
            const enc = std.base64.Base64Encoder.init(std.base64.url_safe_alphabet_chars, '=');
            var dest: []u8 = undefined;
            self.data = std.base64.Base64Encoder.encode(&enc, dest, data);
        }

        if (self.status == 0) {
            self.writeHeader(200);
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

        try w.objectField("kv");
        try w.write(try getCacheJsonObject(cache));

        try w.endObject();
        const result = slice_stream.getWritten();

        std.debug.print("output json: {s}\n", .{ result });

        const stdout = std.io.getStdOut().writer();
        try stdout.print("{s}", .{ result });

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
    while (i.next()) |entry| {
        try value.object.put(entry.key_ptr.*, std.json.Value{ .string = entry.value_ptr.*});
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
    var headersIterator = headers_map.iterator();
    while (headersIterator.next()) |entry| {
        try input.headers.put(entry.key_ptr.*, entry.value_ptr.*.string);
    }

    var kv = parsed.value.object.get("kv").?.object;
    var kvIterator = kv.iterator();
    while (kvIterator.next()) |entry| {
        try cache.put(entry.key_ptr.*, entry.value_ptr.*.string);
    }

    var p = parsed.value.object.get("params").?.object;
    var paramsIterator = p.iterator();
    while (paramsIterator.next()) |entry| {
        try params.put(entry.key_ptr.*, entry.value_ptr.*.string);
    }
    
    return input;
}

pub fn createRequest(in: *Input) !Request {

    var req = Request{
        .url = try std.Uri.parseWithoutScheme(in.url),
        .method = in.method,
        .headers = http.Headers.init(allocator),
        .data = in.body,
        .context = Context.init(),
    };

    // is it even necessary to copy headers from Input to Request struct?
    var i = in.headers.iterator();
    while (i.next()) |kv| {
        try req.headers.append(kv.key_ptr.*, kv.value_ptr.*);
    }

    return req;
}

const RequestAndOutput = struct {
    req: Request,
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

pub const Request = struct {
    url: std.Uri,
    method: []const u8, // TODO: change to http.Method enum
    headers: http.Headers,
    data: []const u8,
    context: Context,
};

pub const Context = struct {
    cache: *std.StringHashMap([]const u8),
    params: *std.StringHashMap([]const u8),

    pub fn init() Context {
        return .{
            .cache = &cache,
            .params = &params,
        };
    }
};

pub const Response = struct {
    data: []const u8,
    headers: http.Headers,
    request: Request,

    pub fn writeAll(res: *Response, data: []const u8) !u32 {
        res.data = data;
        return res.data.len;
    }
};

// Function parameter as function pointer
pub fn ServeFunc(requestFn: *const fn (*Response, *Request) void) void {
    var r = try getWriterRequest();
    var request = r.req;
    var output = r.output;

    var response = Response{ .data = "", .headers = http.Headers.init(allocator), .request = request, };
    
    requestFn(&response, &request);

    output.httpHeader = response.headers;

    _ = output.write(response.data) catch |err| {
        std.debug.print("error writing data: {!} \n", .{ err });
    };
}
