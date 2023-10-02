SUBDIRS = rust-basic rust-fetch rust-kv rust-wasi-nn rust-wasi-nn-preload rust-pdf-create
COMPONENTS = components/rust-basic components/rust-kv

all: $(SUBDIRS) rust-params $(COMPONENTS) components/rust-params

.PHONY: $(SUBDIRS) rust-params $(COMPONENTS) components/rust-params

$(SUBDIRS):
	cd $@ && \
		cargo build --target wasm32-wasi --release && \
			cp target/wasm32-wasi/release/$@.wasm ./$@.wasm

rust-params:
	cd $@ && \
		cargo build --target wasm32-wasi --release && \
			cp target/wasm32-wasi/release/$@.wasm "./[id].wasm"

$(COMPONENTS):
	mkdir -p $@
	make $(@:components/%=%)
	cp $(@:components/%=%)/*.toml $@/
	wasm-tools component new $(@:components/%=%)/$(@:components/%=%).wasm \
		--adapt wasi_snapshot_preview1=components/wasi-component-adapter/wasi_snapshot_preview1-command.wasm \
		-o $@/$(@:components/%=%).wasm

components/rust-params:
	mkdir -p $@
	make rust-params
	wasm-tools component new rust-params/[id].wasm \
		--adapt wasi_snapshot_preview1=components/wasi-component-adapter/wasi_snapshot_preview1-command.wasm \
		-o $@/[id].wasm
