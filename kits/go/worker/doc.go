/**
 *
 *                             === Go support for WASM Workers Server ===
 *
 * This package provides a simple way to write WASM workers in Go. It uses the gjson, sjson libraries instead
 * of Go's standard encoding/json package due to the following reasons:
 *    -- as of writing this file, the default Go compiler does not support the WASI backend,
 *    -- TinyGo (which does support WASI) does not support reflection and hence, we need to rely on a JSON library
 *       that does not use reflection
 *
 */

package worker
