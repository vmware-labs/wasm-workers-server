const std = @import("std");
const io = std.io;
const http = std.http;
const worker = @import("worker");

fn requestFn(resp: *worker.Response, r: *worker.Request) void {
    _ = r;
    _ = &resp.headers.append("x-generated-by", "wasm-workers-server");
    _ = &resp.writeAll("Hello from Zig ⚡️!");
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
