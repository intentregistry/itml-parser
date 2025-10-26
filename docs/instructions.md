
# instructions.md — Cursor Playbook for itml-parser

This is a hands-on plan for implementing the parser/formatter/linter with Rust -> WASM. Paste sections into Cursor as tasks.

---

## 0) Repo bootstrap (1 hour)

Files & Scaffolding
- cargo new itml-parser --lib
- Add crates to Cargo.toml:
  [dependencies]
  pest = "2"
  pest_derive = "2"
  thiserror = "1"
  miette = { version = "7", features = ["fancy"] }
  serde = { version = "1", features = ["derive"], optional = true }
  once_cell = "1"

  [dev-dependencies]
  insta = "1"
  criterion = "0.5"

  [features]
  default = []
  wasm = ["wasm-bindgen"]
  serde = ["dep:serde"]

  [dependencies.wasm-bindgen]
  version = "0.2"
  optional = true

- Create folders: src, grammar, tests, benches, wasm.

Commit: chore: init crate.

---

## 1) Grammar (3-4 hours)

Task
- Translate ITML v0.1 EBNF to Pest grammar grammar/itml.pest.
- Cover tokens: identifiers, strings with interpolation, numbers, booleans, enums, lists, maps, regex /.../flags, comments (# and triple-slash fences).

Acceptance
- Minimal parser that recognizes document -> top-level blocks.
- Add tests/conformance.rs with valid/invalid fixtures.

Prompt to Cursor:
Create grammar/itml.pest from the EBNF in docs, include whitespace, newline, and indentation handling. Implement a Rule::document entry and unit tests that parse examples/address.valid.itml successfully and reject examples/address.invalid.itml with a helpful ParseError.

---

## 2) AST & Parse Mapping (4-6 hours)

Task
- Design src/ast.rs types for all canonical blocks: App, Intent, Layout, Component, Schema, Policy, plus common blocks like Inputs, Outputs, Context, Workflow, Rules, Tests, Policies, Routes, Imports, Theme.
- Implement src/parser.rs that walks Pest pairs -> strongly-typed AST with spans (line/column).

Acceptance
- parse(&str, &ParseOptions) -> Result<Document, ParseError>
- Rich errors with code, message, and precise span.

Prompt:
Implement ast.rs and parser.rs. Provide conversions from Pest pairs to Document. Add helpers to compute spans and attach them to nodes. Write unit tests in tests/conformance.rs that assert the top-level kind and counts of child blocks.

---

## 3) Formatter (3-4 hours)

Task
- Implement format(&str, &FormatOptions) that (a) parses, (b) pretty-prints.
- Normalize: 2-space indent, newline = \n, ordering for sections (header, imports, routes/blocks), stable quoting, space after :.

Acceptance
- Idempotency: format(format(x)) == format(x)
- Snapshot tests in tests/formatter_snapshots.rs with insta.

Prompt:
Implement a pretty-printer in formatter.rs. Ensure keys are ordered deterministically and indentation is correct. Add insta snapshots for examples/project.app.itml and examples/address.valid.itml.

---

## 4) Linter (4-5 hours)

Task
- Define Diagnostic { code, level, message, span, hint, fix } and LintRule enum.
- Implement rules ITML001..ITML006 with safe autofixes where trivial (e.g., tabs->spaces).

Acceptance
- lint(&str, &LintOptions) -> Vec<Diagnostic>
- Unit tests in tests/lints.rs covering each rule.

Prompt:
Implement linter with rules ITML001..ITML006. Return diagnostics with human-friendly hints. Add tests that assert codes and spans. Provide a --fix path for tabs/spaces normalization (ITML005).

---

## 5) WASM bindings (2-3 hours)

Task
- Add src/wasm.rs behind feature wasm exposing parse/format/lint via wasm-bindgen.
- Scaffold wasm/package.json (name @intent/itml-parser-wasm, type module).

Commands:
wasm-pack build --target bundler --out-dir wasm/pkg --features wasm

Acceptance
- Node smoke test:
  import init, { parse, format, lint } from "./wasm/pkg/itml_parser.js";
  await init();
  console.log(parse('intent "Hi"\ninputs:\n  - name (string)'));

---

## 6) Benchmarks & Limits (1-2 hours)

Task
- Add criterion bench on large .itml (synthetic 5k lines).
- Document typical performance and memory notes in README.

---

## 7) CI + Release (2 hours)

Task
- GitHub Actions: fmt+clippy, tests, wasm-pack build.
- Release on tag: publish crate (crates.io) + npm (changesets).

---

## 8) Developer UX & Examples (1 hour)

Task
- Provide /examples with valid + invalid files (address schema, app, policy with wildcard to trigger ITML002, etc.).
- Add quickstart snippet to README.

---

## Commands Reference

# Dev
cargo fmt
cargo clippy -D warnings
cargo test

# Bench
cargo bench

# WASM
wasm-pack build --target bundler --out-dir wasm/pkg --features wasm
node wasm/examples/node-smoke.mjs

---

## Definition of Done (MVP)

- parse, format, lint compile and pass tests
- Formatter idempotency snapshots
- Lint rules ITML001–ITML006 implemented
- WASM package builds and parses a sample file in Node
- CI green on main
- README explains APIs and release flow

---

## Stretch Goals

- Autocomplete helpers for LSP
- Lossless CST (concrete syntax tree) layer for round-tripping comments
- Incremental parser mode
- Source maps for formatter fixes
- More lint rules (namespacing, imports aliasing, unused provides/requires)
