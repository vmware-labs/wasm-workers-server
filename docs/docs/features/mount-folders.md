---
title: Mount folders
---

:::info

[Available since v1.1](https://github.com/vmware-labs/wasm-workers-server/releases/tag/v1.1.0)

:::

Wasm Workers Server allows you to mount folders in the workers' execution context so they can access the files inside. This configuration is done through the `TOML` file associated to a worker (a `TOML` file with the same filename as the worker). **This means every worker has its own set of mount folders**.

The following `TOML` config file mounts the `_libs` folder so the worker can access it:

```toml
version = "1"

[[folders]]
from = "./_libs"
to = "/mnt/libs"
```

If your worker requires more than one folder, you can mount multiple ones:

```toml
version = "1"

[[folders]]
from = "./_libs"
to = "/mnt/libs"

[[folders]]
from = "./_assets"
to = "/mnt/assets"
```

## Avoid wws looking for workers in folders

In the previous example, all folders starts with an underscore character (`_`). This folder name convention tells `wws` to ignore any file inside it when detecting available workers.

Note that those folders may include files that `wws` recognizes as workers (like `.js` or `.py`). By prefixing those folders with an `_`, you ensure `wws` won't process those files as workers.

## Language compatibility

| Language | Mount folders |
| --- | --- |
| JavaScript | ❌ |
| Rust | ✅ |
| Go | ✅ |
| Ruby | ✅ |
| Python | ✅ |
