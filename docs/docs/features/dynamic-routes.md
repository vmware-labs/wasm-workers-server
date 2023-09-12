---
title: Dynamic Routes
sidebar_position: 4
---

:::info

[Available since v1.0](https://github.com/vmware-labs/wasm-workers-server/releases/tag/v1.0.0)

:::

Defining static routes may not be enough for some applications. You may need a worker to process URLs that includes identifiers. **To create a worker associated with a dynamic route, include the route parameter in brackets when setting the worker filename**.

For example, imagine you want a worker that replies to the following URLs:

- `/show/1`
- `/show/2`

With dynamic routes, you can create a worker with the `show/[id].js` filename. This worker will reply to any `/show/X` route automatically.

After defining the route paremeters, the worker receives a special argument called `params`. It includes the value of all defined parameters. Note that the name of the parameter will be defined by the text between the brackets.

Check these guides to understand how to read parameters in the different supported languages:

* [Dynamic routes in JavaScript](../languages/javascript.md#dynamic-routes)
* [Dynamic routes in Rust](../languages/rust.md#dynamic-routes)
* [Dynamic routes in Python](../languages/python.md#dynamic-routes)
* [Dynamic routes in Ruby](../languages/ruby.md#dynamic-routes)
* [Dynamic routes in Go](../languages/go.md#dynamic-routes)
* [Dynamic routes in Zig](../languages/zig.md#dynamic-routes)

## Dynamic routes and folders

Folders can follow this pattern too. You can define a folder that has a route parameter:

```
$ tree ./examples/with-params
./examples/with-params
├── [id]
    └── fixed.js
```

In this case, the `./[id]/fixed.js` worker can reply to URLs like `/abc/fixed` and `/test/fixed`.

## Multiple parameters

As both files and folders can be dynamic, workers may receive multiple parameters. The `params` object includes the value of all the different parameters.

```
$ tree .
.
├── [resource]
    └── [id]
        └── show.js
```

In this case, the `./[resource]/[id]/show.js` worker replies to URLs like `/articles/2/show`.

## Catch-all routes

Catch-all routes are route segments that can be matched with any path segment on the route. For example, you can use catch-all routes by having a directory structure like the following:

```
$ tree .
.
└── [...slug]
    └── index.js
```

This means, that the JavaScript worker at `[...slug]/index.js` will serve any path beneath. You can mix and match specific routes with catch-all routes. For example, given the following directory structure:

```
$ tree .
.
├── about.js
└── [...slug]
    └── index.js
```

In this example, two workers are fulfilling HTTP requests:

- `/about` is served by the `about.js` worker.
- Anything else under `/` is served by the `[...slug]/index.js` worker.

You can also place multiple catch-all routes as long as they are splitted by a non-catch-all segment. For example:

```
$ tree .
.
├── about.js
├── other
│   └── [...slug]
│       └── index.js
└── [...slug]
    └── index.js
```

Here, we have the same structure as in the previous example, but we have two catch-all, under two different roots:

- One catch-all, `[...slug]/index.js` is serving all requests, except for requests whose path starts with `/about` or `/other`.
- Another catch-all, `other/[...slug]/index.js` serves all requests under the `/other` path.

### Routing priority

Given catch-all routes could potentially shadow other routes, it is important to settle precedence when routing requests. The rule of thumb is more specific routes win. This is, a route with no catch-all will always against a route with catch-all when they are at the same depth.

## Language compatibility

| Language   | Dynamic routes |
|------------|----------------|
| JavaScript | ✅             |
| Rust       | ✅             |
| Go         | ✅             |
| Ruby       | ✅             |
| Python     | ✅             |
| Zig        | ✅             |
