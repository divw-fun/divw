.PHONY: build test clean

build:
	anchor build
	cd sdk && npm run build

test:
	cargo test
	anchor test
	cd sdk && npm test

clean:
	cargo clean
	cd sdk && rm -rf dist node_modules

lint:
	cargo fmt --check
	cargo clippy
	cd sdk && npm run lint

format:
	cargo fmt
	cd sdk && npm run format
