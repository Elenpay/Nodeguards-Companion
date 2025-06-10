set fallback := true

install-1p-mac:
	brew install --cask 1password/tap/1password-cli

install-depenencies:
	docker compose up build-ext

build-extension: 
	docker compose up build-ext

build-extension-v2: build-extension
	cp extension/versions/manifest_v2.json extension/dist/manifest.json

serve-extension:
	docker compose up --build serve-ext

upgrade-version:
	docker compose up upgrade-version

get-field FIELD *args='':
	op item get "Firefox Extension Signing Credentials" --fields {{FIELD}} {{args}}

sign-extension:
	#!/bin/bash
	API_KEY=$(just get-field username)
	API_SECRET=$(just get-field credential --reveal)
	docker compose run --rm -e API_KEY="$API_KEY" -e API_SECRET="$API_SECRET" web-ext