
# ✅ itml-parser (Rust -> WASM) — Implementation Checklist

This file is a **developer checklist** for verifying every deliverable in the `itml-parser` repository.

---

## 🧩 Scope
Parser / Formatter / Linter for **ITML v0.1** with Rust → WASM bindings.  
Implements: `parse(itml)`, `format(itml)`, `lint(itml)`.

---

## 1) EBNF v0.1 Consolidated (`grammar/itml.ebnf`)

### Steps
1. Place the grammar source in `grammar/itml.ebnf` and the Pest version in `grammar/itml.pest`.
2. Add a minimal test that validates all `examples/*.valid.itml` pass and all `*.invalid.itml` fail.
3. Document in `README.md` that `itml.ebnf` is the “single source of truth” for the language.

### Commands
```bash
git add grammar/itml.ebnf grammar/itml.pest
cargo test -q --test conformance
```

### Definition of Done
- `itml.ebnf` exists and matches the parser grammar.
- Conformance tests for valid/invalid examples pass.

---

## 2) `parse(itml) -> Ast`

### Steps
1. Implement `parse()` in `src/parser.rs` using types from `src/ast.rs`.
2. Return `Result<Document, ParseError>` with line/column/offset span info.

### Commands
```bash
cargo test -q --test conformance -- --nocapture
```

### Definition of Done
- All valid `.itml` files produce a non-empty AST.
- Invalid `.itml` files produce a structured `ParseError` with proper `span`.

---

## 3) `format(itml) -> String` (Canonical Style)

### Steps
1. Implement `format()` in `src/formatter.rs`.
2. Add snapshot tests using `insta`.
3. Verify idempotency: `format(format(x)) == format(x)`.

### Commands
```bash
cargo test -q --test formatter_snapshots
```

### Definition of Done
- All snapshot tests green.
- Formatter output stable and idempotent.

---

## 4) `lint(itml) -> Diagnostics[]` (ITML001–006)

### Steps
1. Implement `lint()` in `src/linter/` returning `Diagnostic { code, level, message, span, hint, fix? }`.
2. Add fixtures that trigger each lint rule:

| Rule | Description |
|------|--------------|
| ITML001 | Every intent must have `inputs` and (`workflow` or `rules`). |
| ITML002 | Disallow `"*"` in `network.allow`. |
| ITML003 | Ensure all `routes` targets exist. |
| ITML004 | Require `tests` when `rules` exist. |
| ITML005 | Forbid mixed tabs/spaces. |
| ITML006 | Warn on unknown keys with suggestions. |

### Commands
```bash
cargo test -q --test lints
```

### Definition of Done
- Each rule returns at least one diagnostic with correct code and span.
- Autofix (`--fix`) works for trivial cases (e.g. ITML005).

---

## 5) WASM Package (`wasm-pack build`) + Bindings

### Steps
1. Add target: `rustup target add wasm32-unknown-unknown`.
2. Implement `src/wasm.rs` with `wasm-bindgen` under feature `wasm`.
3. Create `wasm/package.json` (type=module) and export JS bindings.

### Commands
```bash
rustup target add wasm32-unknown-unknown
wasm-pack build --target bundler --out-dir wasm/pkg --features wasm
node -e "import('./wasm/pkg/itml_parser.js').then(async m=>{await m.default(); console.log(!!m.parse('intent \"Hi\"\ninputs:\n  - name (string)'));})"
```

### Definition of Done
- Build produces `wasm/pkg/*.wasm`, `*.js`, `*.d.ts`.
- Node smoke test runs without errors.

---

## 6) Conformance Tests (5–10 Official Examples)

### Steps
1. Provide fixtures:
   - `examples/address.valid.itml`
   - `examples/address.invalid.itml`
   - `examples/project.app.itml`
   - Plus 2–7 additional cases (imports, routes, rules, policies, theme, etc.)
2. In `tests/conformance.rs`, iterate over all examples and assert validity.

### Commands
```bash
cargo test -q --test conformance
```

### Definition of Done
- 5–10 fixtures cover core ITML patterns.
- Valid files parse successfully; invalid ones emit `ParseError`.

---

## 7) CI: Build + Tests + Binary Size Report

### Steps
1. Create `.github/workflows/ci.yml` with two jobs:
   - **Rust Build/Test**: `fmt`, `clippy`, `test`.
   - **WASM Build**: `wasm-pack build` and artifact upload.
2. Append size summary to the GitHub job output.

### Example workflow
```yaml
name: CI
on: [push, pull_request]

jobs:
  build-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Lint & Test
        run: |
          cargo fmt --all -- --check
          cargo clippy --all-targets -- -D warnings
          cargo test --all --locked

  wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Add wasm target
        run: rustup target add wasm32-unknown-unknown
      - uses: jetli/wasm-pack-action@v0.4.0
      - name: Build wasm
        run: wasm-pack build --target bundler --out-dir wasm/pkg --features wasm
      - name: Size report
        run: |
          NATIVE_SIZE=$(du -h target/debug/deps | tail -n1 | awk '{print $1}' || echo "n/a")
          WASM_FILE=$(ls -S wasm/pkg/*.wasm | head -n1)
          WASM_SIZE=$(du -h "$WASM_FILE" | awk '{print $1}')
          echo "## Build Artifacts Size" >> $GITHUB_STEP_SUMMARY
          echo "- Native (debug): ${NATIVE_SIZE}" >> $GITHUB_STEP_SUMMARY
          echo "- WASM: ${WASM_SIZE} ($(basename "$WASM_FILE"))" >> $GITHUB_STEP_SUMMARY
      - uses: actions/upload-artifact@v4
        with:
          name: itml-parser-wasm
          path: wasm/pkg/
```

### Definition of Done
- CI passes on all pushes and pull requests.
- Summary reports both native and WASM sizes.
- WASM artifact uploaded successfully.

---

## 🧰 Suggested Makefile Targets

```make
dev:
	cargo fmt
	cargo clippy -D warnings

test:
	cargo test -q

conformance:
	cargo test -q --test conformance

format-snapshots:
	cargo test -q --test formatter_snapshots

wasm:
	rustup target add wasm32-unknown-unknown || true
	wasm-pack build --target bundler --out-dir wasm/pkg --features wasm

size:
	du -h wasm/pkg/*.wasm | sort -h
```

---

## ✅ Completion Summary

| Item | Status | Verification |
|------|--------|--------------|
| EBNF v0.1 | ☐ | `cargo test --test conformance` |
| parse() | ☐ | Valid/invalid fixtures parsed correctly |
| format() | ☐ | Idempotent + snapshots stable |
| lint() | ☐ | ITML001–006 tests passing |
| WASM package | ☐ | `wasm-pack build` + Node smoke test |
| Conformance tests | ☐ | 5–10 examples validated |
| CI pipeline | ☐ | Green on PR + artifact + size summary |

Mark each box after confirming via command line or CI logs.

---

**End of CHECKLIST.md**
