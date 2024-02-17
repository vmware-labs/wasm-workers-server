const std = @import("std");
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

fn requestFn(resp: *worker.Response, r: *worker.Request) void {
    var params = r.context.params;

    var id: []const u8 = "the value is not available";

    var v = params.get("id");

    if (v) |val| {
        id = val;
    }

    const s =
        \\Hey! The parameter is: {s}
    ;

    var body = std.fmt.allocPrint(allocator, s, .{ id }) catch undefined; // add useragent

    _ = &resp.headers.append("x-generated-by", "wasm-workers-server");
    _ = &resp.writeAll(body);
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
