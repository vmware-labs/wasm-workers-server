---
sidebar_position: 2
---

# Quickstart

Wasm Workers runs almost anywhere. Thanks to its portability, downloading and running it anywhere is quite simple.

First, you need to install `wws`:

```shell-session
curl -fsSL https://workers.wasmlabs.dev/install | bash
```

Now, you can check the different commands and options:

```shell-session
$ wws --help
A WebAssembly framework to develop and run serverless applications anywhere

Usage: wws [OPTIONS] [PATH] [COMMAND]

Commands:
  runtimes  Manage the language runtimes in your project
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Location of the wws project. It could be a local folder or a git repository [default: .]

Options:
      --host <HOSTNAME>          Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>              Port to initiate the server [default: 8080]
      --prefix <PREFIX>          Prepend the given path to all URLs [default: ]
      --ignore <IGNORE>          Patterns to ignore when looking for worker files [default: ]
  -i, --install-runtimes         Install missing runtimes automatically
      --git-commit <GIT_COMMIT>  Set the commit when using a git repository as project
      --git-tag <GIT_TAG>        Set the tag when using a git repository as project
      --git-branch <GIT_BRANCH>  Set the branch when using a git repository as project
      --git-folder <GIT_FOLDER>  Change the directory when using a git repository as project
      --enable-panel             Enable the administration panel
  -h, --help                     Print help
  -V, --version                  Print version
```

You can pass a remote location, like a git repository, to `wws`. To try it, let's run one of the `js-basic` example from the Wasm Workers Server repository:

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/js-basic"
```

It automatically clones the git repository and loads the workers from the given folder (`examples/js-basic`):

```shell-session
⚙️  Preparing the project from: https://github.com/vmware-labs/wasm-workers-server.git
⚙️  Loading routes from: /tmp/dd21e3cd6d0f515301e1c7070e562af06074d9e8d10566179f97dba47e74cec9/examples/js-basic
⏳ Loading workers from 1 routes...
✅ Workers loaded in 108.82825ms.
    - http://127.0.0.1:8080/
      => /tmp/dd21e3cd6d0f515301e1c7070e562af06074d9e8d10566179f97dba47e74cec9/examples/js-basic/index.js
🚀 Start serving requests at http://127.0.0.1:8080
```

Now, open your browser at <http://127.0.0.1:8080>.

## Next Steps

Now you got the taste of Wasm Workers, it's time to create your first worker:

* [Create your first JavaScript worker](../languages/javascript.md)
* [Create your first Rust worker](../languages/rust.md)
* [Create your first Python worker](../languages/python.md)
* [Create your first Ruby worker](../languages/ruby.md)
* [Create your first Go worker](../languages/go.md)

And if you are curious, here you have a guide about [how it works](./how-it-works.md).
