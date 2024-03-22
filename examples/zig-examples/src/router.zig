const std = @import("std");
const wws = @import("wws");
const zig_router = @import("zig-router");

fn getHello(arena: std.mem.Allocator) !wws.Response {
    var response = wws.Response{
        .data = "Hello, world!",
    };

    try response.headers.map.put(arena, "x-generated-by", "wasm-workers-server");
    return response;
}

const GetBlogPostParams = struct {
    id: []const u8,
};
fn getBlogPost(arena: std.mem.Allocator, params: GetBlogPostParams) !wws.Response {
    var body = std.ArrayList(u8).init(arena);

    try body.writer().print("Blog article contents for id {s}", .{params.id});

    var response = wws.Response{
        .data = try body.toOwnedSlice(),
    };

    try response.headers.map.put(arena, "x-generated-by", "wasm-workers-server");
    return response;
}

fn handle(arena: std.mem.Allocator, request: wws.Request) !wws.Response {
    const router = zig_router.Router(
        .{},
        .{
            zig_router.Route(.GET, "/router/hello", getHello, .{}),
            zig_router.Route(.GET, "/router/post/:id", getBlogPost, .{}),
        },
    );

    const uri = try std.Uri.parse(request.url);

    const response = router.match(
        arena,
        .{
            .method = request.method,
            .path = uri.path,
            .query = uri.query orelse "",
        },
        .{arena},
    ) catch |err| switch (err) {
        error.not_found => wws.Response{ .status = 404, .data = "not found" },
        error.bad_request => wws.Response{ .status = 400, .data = "bad request" },
        else => wws.Response{ .status = 500, .data = "internal server error" },
    };

    return response;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const parse_result = try wws.parseStream(gpa.allocator(), .{});
    defer parse_result.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const request = parse_result.value;
    const response = try handle(arena.allocator(), request);

    std.debug.print("response {s}\n", .{response.data});

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
