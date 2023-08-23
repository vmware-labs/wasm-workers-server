---
title: Multiple language runtimes
sidebar_position: 2
---

:::info

[Available since v1.0](https://github.com/vmware-labs/wasm-workers-server/releases/tag/v1.0.0)

:::

Wasm Workers Server allows you to extend the supported languages by adding new language runtimes. In other words, you can run workers based on languages like Python or Ruby.

## How it works?

To provide these language runtimes, `wws` relies on the [WebAssembly Language Runtimes](https://github.com/vmware-labs/webassembly-language-runtimes) project. It provides popular language runtimes precompiled to WebAssembly, like Ruby and Python. `wws` integrates with this repository and allows you to list, install and uninstall the different available languages:

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

## The .wws.toml file

After installing a language runtime, `wws` creates a `.wws.toml` file in your project folder. This file saves the language runtime metadata. We recommend you to commit this file to your repository. It allows other developers to install the required language runtimes by running a single command:

```
wws runtimes install
```

## Manage language runtimes

### List available language runtimes

You can list the avilable runtimes with the `runtimes list` command:

```
wws runtimes list
⚙️  Fetching data from the repository...
┌────────┬─────────┬───────────┬─────────────┐
│ Name   │ Version │ Extension │ Binary      │
├────────┼─────────┼───────────┼─────────────┤
│ ruby   │ 3.2.0   │ rb        │ ruby.wasm   │
├────────┼─────────┼───────────┼─────────────┤
│ python │ 3.11.1  │ py        │ python.wasm │
└────────┴─────────┴───────────┴─────────────┘
```

The table provides the following data:

* **Name**: the name of the language runtime
* **Version**: a specific version for that language runtime. You can find multiple versions and variations
* **Extension**: the file extension associated to this language runtimes. For example, `wws` will load `*.rb` as workers if you install the `ruby` runtime
* **Binary**: the name of the Wasm module

### Install a new language runtime

To install a new language runtime, you need to provide the name and the version in the given repository:

```
wws runtimes install ruby 3.2.0
⚙️  Fetching data from the repository...
🚀 Installing the runtime...
✅ Done
```

The language runtime and required files will be installed in the `.wws` folder:

```
tree ./.wws
./.wws
└── runtimes
    └── wlr
        ├── python
        │   └── 3.11.1
        │       ├── poly.py
        │       ├── python.wasm
        │       └── template.txt
        └── ruby
            └── 3.2.0
                ├── poly.rb
                ├── ruby.wasm
                └── template.txt
```

#### Install the language runtimes for an existing project

If the project has a `.wws.toml` file, you can install quickly all the required language runtimes. You can run the `runtimes install` command without any extra parameter. `wws` will read the configuration file and install all missing runtimes:

```
./wws runtimes install
⚙️  Checking local configuration...
🚀 Installing: wlr - python / 3.11.1
🚀 Installing: wlr - ruby / 3.2.0
✅ Done
```

### Check installed language runtimes

You can check the installed language runtimes with the `runtimes check` command:

```
wws runtimes check
┌───────────┬────────┬─────────┬───────────┬─────────────┐
│ Installed │ Name   │ Version │ Extension │ Binary      │
├───────────┼────────┼─────────┼───────────┼─────────────┤
│ ✅        │ python │ 3.11.1  │ py        │ python.wasm │
├───────────┼────────┼─────────┼───────────┼─────────────┤
│ ✅        │ ruby   │ 3.2.0   │ rb        │ ruby.wasm   │
└───────────┴────────┴─────────┴───────────┴─────────────┘
```

If a runtime is present in the `.wws.toml` file but it's not installed in the system, the "Installed" field will show an error:

```
wws runtimes check
┌───────────┬────────┬─────────┬───────────┬─────────────┐
│ Installed │ Name   │ Version │ Extension │ Binary      │
├───────────┼────────┼─────────┼───────────┼─────────────┤
│ ❌        │ python │ 3.11.1  │ py        │ python.wasm │
├───────────┼────────┼─────────┼───────────┼─────────────┤
│ ❌        │ ruby   │ 3.2.0   │ rb        │ ruby.wasm   │
└───────────┴────────┴─────────┴───────────┴─────────────┘

💡 Tip: there are missing language runtimes. You can install them with `wws runtimes install`
```

We recommend you to check the 💡 tips as they provide very useful information

### Uninstall a language runtime

To uninstall a language runtime, use the `runtimes uninstall` command and provide the runtime name and version. You can get this information from the [`runtimes check command`](#check-installed-language-runtimes).

```
wws runtimes uninstall ruby 3.2.0
🗑  Uninstalling: wlr - ruby / 3.2.0
✅ Done
```

This command also edits the `.wws.toml` file to remove any reference to that specific runtime

## Configure a different repository

By default, `wws` relies on the binaries from the [WebAssembly Language Runtimes](https://github.com/vmware-labs/webassembly-language-runtimes) project. This project contains precompiled language runtimes for many popular languages. When you call the `list` or `install` commands, `wws` fetches the metadata from this project and continue processing your request.

If you want to set your own language runtimes repository, you can configure `wws` to fetch the metadata from a different place. For that, you can define the `WWS_REPO_NAME` and `WWS_REPO_URL` environment variables or use the `--repo-name` and `--repo-url` arguments:

```
# Via environment variables
export WWS_REPO_NAME=my-repo
export WWS_REPO_URL=https://example.com/index.toml

# Via arguments
wws runtimes list --repo-name=my-repo --repo-url=https://example.com/index.toml
wws runtimes install ruby 3.2.0 --repo-name=my-repo --repo-url=https://example.com/index.toml
```

After installing a language runtime, the repository information is also stored in the `.wws.toml` file. Developers that install the required language runtimes for an existing project will download them always from the right repository.
