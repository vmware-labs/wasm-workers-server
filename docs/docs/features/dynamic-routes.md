---
sidebar_position: 4
---

# Dynamic routes

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

## Language compatibility

| Language | Dynamic routes |
| --- | --- |
| JavaScript | ✅ |
| Rust | ✅ |
| Go | ✅ |
| Ruby | ✅ |
| Python | ✅ |
