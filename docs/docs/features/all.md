# Features

Wasm Workers Server provides different features to develop serverless applications. Some of these features are related to the server like the static asset management, while others relate to workers like sending HTTP requests.

For that reason, we usually distinguish between server and worker features:

* **Server features**: customizes or expands Wasm Worker Server capabilities. For example, you can expose static assets by saving these files in a `public` folder.

* **Worker features**: expose new features to the individual workers so they can perform more complex tasks. For example, workers can access to a K/V store or use environment variables.

## Available features

### Server

* [Static assets management](./static-assets.md)
* [Multiple language runtimes](./multiple-language-runtimes.md)

### Workers

* [Key / Value store](./key-value.md)
* [HTTP Requests (fetch)](./http-requests.md)
* [Dynamic routes](./dynamic-routes.md)
* [Environment variables](./environment-variables.md)
* [Mount folders](./mount-folders.md)

## Language compatibility

You can develop workers in different languages. However, not all of them support all features. **The goal is to support all of them**, although there are some constraints that make some features more complex to implement in certain languages.

The following table shows the language compatibility for the different worker functions:

| Language | K/V Store | Environment Variables | Dynamic Routes | Folders | HTTP Requests |
| --- | --- | --- | --- | --- | --- |
| JavaScript | ✅ | ✅ | ✅ | ❌ | ✅ |
| Rust | ✅ | ✅ | ✅ | ✅ | ✅ |
| Go | ✅ | ✅ | ✅ | ✅ | ✅ |
| Ruby | ✅ | ✅ | ✅ | ✅ | ❌ |
| Python | ✅ | ✅ | ✅ | ✅ | ❌ |
| Zig | ✅ | ❌ | ✅ | ✅ | ❌ |
