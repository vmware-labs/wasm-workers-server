# Go kit

This folder contains the Go kit or SDK for Wasm Workers Server. Currently, it uses the regular STDIN / STDOUT approach to receive the request and provide the response. In the latest version we introduced the new HTTP bindings to send HTTP requests from inside the worker.

## Bindings

Wasm Workers Server is on the road to adopt Wasm components, but it's not there yet. However, we started adopting WIT to generate the bindings for the different languages.

The host (Wasm Workers Server) and other languages like Rust and JavaScript rely on [wit-bindgen v0.2](https://github.com/bytecodealliance/wit-bindgen/tree/v0.2.0). However, the Go bindings were not available on that version so it caused some extra work to generate the Go bindings.

These are the steps to recreate the current Go bindings:

- Clone the wit-binding repository and checkout to the [35cb45f2](https://github.com/bytecodealliance/wit-bindgen/commit/35cb45f25eb113b54406f269778d46a37716a7c5) commit (between v0.6 - v0.7). This commit produces compatible binding identifiers and fixes an error with the types on the generated C / Go code:

    ```shell-session
    git clone https://github.com/bytecodealliance/wit-bindgen/tree/main && \
      git checkout 35cb45f25eb113b54406f269778d46a37716a7c5
    ```

- Compile the project:

    ```shell-session
    cargo build --release
    ```

- Change your current directory to `wasm-workers-server/kits/go/worker/bindings`.
- Now, you need to use the compiled `wit-bindgen`:

    ```shell-session
    ~/YOUR_LOCATION/wit-bindgen/target/release/wit-bindgen tiny-go ../../../../wit/go-ephemeral/
    ```

- Just note that we're using a specific `wit` folder for Go. The reason is that the syntax changed from v0.3. We will consolidate it once we adopt components.
- Edit the `bindings.c` file to define the `canonical_abi_realloc` and `canonical_abi_free`. wit-bindgen v0.2 expects these methods to be exported. However, the first method was renamed to `cabi_realloc` and the second was removed on v3.0. To fix it, locate the `__attribute__((__weak__, __export_name__("cabi_realloc")))` and replace it with the following two methods:

    ```c
    __attribute__((__weak__, __export_name__("canonical_abi_realloc"))) void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size)
    {
      if (new_size == 0)
        return (void *)align;
      void *ret = realloc(ptr, new_size);
      if (!ret)
        abort();
      return ret;
    }

    __attribute__((weak, export_name("canonical_abi_free"))) void canonical_abi_free(
        void *ptr,
        size_t size,
        size_t align)
    {
      free(ptr);
    }
    ```

- Done!

## References

* [Go documentation](https://workers.wasmlabs.dev/docs/languages/go)
* [Announcing Go Support for Wasm Workers Server](https://wasmlabs.dev/articles/go-support-on-wasm-workers-server/)
