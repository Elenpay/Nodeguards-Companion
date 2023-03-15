set fallback := true
export AR := "llvm-ar"

setup:
	brew install llvm

build-wasm:
	cd extension && cargo build --target wasm32-unknown-unknown

build-signer:
	cd extension && wasm-pack build

build-extension: build-signer
	cd extension && yarn add ./pkg --check-files && yarn run build

serve-extension:
	cd extension && yarn run serve
