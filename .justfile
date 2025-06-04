set fallback := true

install-1p-mac:
	brew install --cask 1password/tap/1password-cli

build-extension: 
	docker compose up

build-extension-v2: build-extension
	docker compose up
	cp extension/versions/manifest_v2.json extension/dist/manifest.json

upgrade-version:
	docker compose up upgrade-version

get-field FIELD:
	op item get "Firefox Extension Signing Credentials" --fields {{FIELD}}

sign-extension:
	cd extension/dist && web-ext sign --api-key=$(just get-field username) --api-secret=$(just get-field credential) 