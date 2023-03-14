set fallback := true
export AR := "llvm-ar"

setup:
	brew install llvm

build-wasm:
	cd signer && cargo build --target wasm32-unknown-unknown

build-signer:
	cd signer && wasm-pack build

build-extension: build-signer
	cd extension && yarn run build

serve-extension:
	cd extension && yarn run serve