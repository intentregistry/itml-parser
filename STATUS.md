## Project Status — itml-parser

Last updated: 28-10-2025

### Overview
Parser, formatter, and linter for ITML v0.1 with optional WASM bindings. Public APIs: `parse`, `format`, `lint` (Rust) and `parse_itml`, `format_itml`, `lint_itml` (WASM).

### Releases
- **Crate**: `itml-parser` v0.1.0 published to crates.io
- **Docs**: available on docs.rs
- **License**: MIT

### Implemented Features
- **Parser**: Generates typed AST from `.itml` (spec v0.1)
- **Formatter**: Canonical whitespace/indent/ordering; idempotent output
- **Linter**: Rules ITML001–ITML006 with codes, levels, spans, and hints
- **WASM Bindings**: Browser and Node.js targets via `wasm-bindgen` (feature `wasm`)
- **Conformance**: Example-based tests plus snapshot tests
- **Benchmarks**: Criterion benches present

### Build Targets
- `cargo build` (native)
- `wasm-pack build --features wasm` (bundler/node/web targets via Makefile)

### Test & CI
- Unit/integration tests passing locally
- Formatter snapshot tests included
- Conformance tests for valid/invalid examples
- GitHub Actions workflow present for Rust + WASM builds

### Packages & Artifacts
- WASM package output: `wasm/pkg/` (`*.wasm`, `*.js`, `*.d.ts`)
- NPM package name in README: `@intent/itml-parser-wasm` (verify/publish as needed)

### Public APIs (Rust)
- `parse(input: &str, &ParseOptions) -> Result<Document, ParseError>`
- `format(&Document, &FormatOptions) -> String`
- `lint(&Document, &LintOptions) -> Vec<Diagnostic>`

### Lint Rules
- ITML001: Intents require `inputs` and (`workflow` or `rules`)
- ITML002: Disallow "*" in `network.allow`
- ITML003: Ensure `routes` targets exist
- ITML004: Require `tests` when `rules` exist
- ITML005: Forbid mixed tabs/spaces
- ITML006: Warn on unknown keys with suggestions

### Roadmap (near-term)
- Publish/verify NPM package for WASM bindings
- Expand conformance fixtures (5–10 official cases)
- Expose JSON schema for AST (optional)
- Add `--fix` for more lint rules where safe
- Set up automated release flow (tags -> crates.io/docs.rs/npm)

### How to Reproduce Locally
- Build: `cargo build`
- Test: `cargo test`
- WASM (bundler): `make wasm`
- Node smoke test: `make wasm-test`

### Links
- Crate: [crates.io/crates/itml-parser](https://crates.io/crates/itml-parser)
- Docs: [docs.rs/itml-parser](https://docs.rs/itml-parser)
- Repo: [github.com/intent-ecosystem/itml-parser](https://github.com/intent-ecosystem/itml-parser)
