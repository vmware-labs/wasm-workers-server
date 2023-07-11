module go-basic

go 1.20

require github.com/vmware-labs/wasm-workers-server v1.3.0

require (
	github.com/tidwall/gjson v1.14.4 // indirect
	github.com/tidwall/match v1.1.1 // indirect
	github.com/tidwall/pretty v1.2.1 // indirect
	github.com/tidwall/sjson v1.2.5 // indirect
)

replace github.com/vmware-labs/wasm-workers-server => ../../
replace github.com/vmware-labs/wasm-workers-server/kits/go/worker/bindings => ../../kits/go/worker/bindings
