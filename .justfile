set fallback := true
export AR := "llvm-ar"

install-1p-mac:
	brew install --cask 1password/tap/1password-cli

setup:
	brew install llvm

build-wasm:
	cd extension && cargo build --target wasm32-unknown-unknown

build-signer:
	cd extension && wasm-pack build

build-extension: build-signer
	cd extension && yarn add ./pkg --check-files && yarn run build

build-extension-v2: build-extension
	cp extension/versions/manifest_v2.json extension/dist/manifest.json

serve-extension:
	cd extension && yarn run serve

upgrade-extension-patch:
	cd extension/scripts && node upgradeVersion.js patch

get-field FIELD:
	op item get "Firefox Extension Signing Credentials" --fields {{FIELD}}

sign-extension:
	cd extension/dist && web-ext sign --api-key=$(just get-field username) --api-secret=$(just get-field credential) 