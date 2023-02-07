# Introduction

Wasm Workers Server can run `.wasm` modules natively. When you run `wws` in a folder, any file with this extension will run as a worker:

```bash
wws

⚙️ Loading routes from: .
🗺 Detected routes:
    - http://127.0.0.1:8080/
    => index.wasm (name: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

However, these are not the only files `wws` can manage. You can extend `wws` with different language runtimes to run files from several languages like [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript), [Ruby](https://www.ruby-lang.org) and [Python](https://www.python.org/).

## How are multiple language runtimes supported?

By default, `wws` only supports Wasm modules and JavaScript files. For any other language, you need the language interpreter or runtime. These runtimes are also Wasm modules. When you run `wws`, it loads the available runtimes and mount the source code from your workers.

However, compiling a language interpreter is not an easy task. To simplify this process, `wws` relies by default in the [WebAssembly Language Runtimes](https://github.com/vmware-labs/webassembly-language-runtimes). This projects offers a set of precompiled languages runtimes you can plug and play in projects like `wws`.

## How to manage language runtimes in wws?

You can check the available commands directly in the `wws` CLI:

```
wws runtimes --help

Usage: wws runtimes [OPTIONS] <COMMAND>

Commands:
  install    Install a new language runtime (like Ruby, Python, etc)
  list       List all available runtimes to install. By default, it uses the WebAssembly Language Runtime repository
  check      List of locally installed runtimes
  uninstall  Uninstall a language runtime
  help       Print this message or the help of the given subcommand(s)

Options:
      --repo-url <REPO_URL>    Set a different repository URL
      --repo-name <REPO_NAME>  Gives a name to the given repository URL
  -h, --help                   Print help information
```

For a more complete documentation, please refer to the [Multiple Languages Runtimes](../features/multiple-language-runtimes.md) section.