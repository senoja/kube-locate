TAG := $(shell git describe --tags)

.PHONY: build
build: 
	cargo build --release
	cargo build --release --target x86_64-unknown-linux-musl

.PHONY: release
release: build
	@echo Building release for tag $(TAG)
	rm -rf dist
	mkdir -p dist
	tar -czvf dist/kube-locate-$(TAG)-x86_64-apple-darwin.tar.gz -C target/release klo
	tar -czvf dist/kube-locate-$(TAG)-x86_64-unknown-linux-musl.tar.gz -C target/x86_64-unknown-linux-musl/release klo
	shasum -a 256 target/release/klo > dist/kube-locate-$(TAG)-x86_64-apple-darwin.sha256
	shasum -a 256 target/x86_64-unknown-linux-musl/release/klo > dist/kube-locate-$(TAG)-x86_64-unknown-linux-musl.sha256
