.PHONY: build test bench lint wasm clean help

# Default target
help:
	@echo "Available targets:"
	@echo "  build    - Build the Rust crate"
	@echo "  test     - Run all tests"
	@echo "  bench    - Run benchmarks"
	@echo "  lint     - Run clippy and fmt checks"
	@echo "  wasm     - Build WASM package"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"

# Build the Rust crate
build:
	cargo build

# Run all tests
test:
	cargo test

# Run benchmarks
bench:
	cargo bench

# Run linting checks
lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

# Build WASM package
wasm:
	wasm-pack build --target bundler --out-dir wasm/pkg --features wasm

# Build WASM package for Node.js
wasm-node:
	wasm-pack build --target nodejs --out-dir wasm/pkg --features wasm

# Build WASM package for web
wasm-web:
	wasm-pack build --target web --out-dir wasm/pkg --features wasm

# Test WASM package
wasm-test:
	cd wasm && node examples/node-smoke.mjs

# Clean build artifacts
clean:
	cargo clean
	rm -rf wasm/pkg

# Install wasm-pack (if not already installed)
install-wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Full build including WASM
build-all: build wasm

# Full test including WASM
test-all: test wasm-test
