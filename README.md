# itml-parser

[![CI](https://github.com/intent-ecosystem/itml-parser/workflows/CI/badge.svg)](https://github.com/intent-ecosystem/itml-parser/actions)
[![Crates.io](https://img.shields.io/crates/v/itml-parser.svg)](https://crates.io/crates/itml-parser)
[![npm](https://img.shields.io/npm/v/@intent/itml-parser-wasm.svg)](https://www.npmjs.com/package/@intent/itml-parser-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Parser, formatter, and linter for ITML v0.1 — the Intention Markup Language used by the Intent Registry ecosystem.

## Features

- **Parser**: Produces a typed AST from `.itml` text according to ITML v0.1 specification
- **Formatter**: Canonicalizes whitespace, indentation, ordering, and quoting with idempotent output
- **Linter**: Static analysis with rule codes (ITML001–ITML006), precise spans, and quick-fix hints
- **WASM Build**: WebAssembly bindings for web browsers and Node.js runtimes
- **Conformance**: Comprehensive test suite with valid/invalid fixtures and property tests

## Quick Start

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
itml-parser = "0.1"
```

```rust
use itml_parser::{parse, format, lint, ParseOptions, FormatOptions, LintOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let itml_source = r#"
        schema "Address"
        description: "A postal address schema"
        
        fields:
          street: string
          city: string
          state: string
          zip: string
          country: string = "US"
        
        validation:
          - street: required
          - city: required
          - state: required
          - zip: pattern("^[0-9]{5}(-[0-9]{4})?$")
    "#;
    
    // Parse ITML
    let parse_opts = ParseOptions::default();
    let doc = parse(itml_source, &parse_opts)?;
    println!("Parsed: {:?}", doc);
    
    // Format ITML
    let format_opts = FormatOptions::default();
    let formatted = format(&doc, &format_opts);
    println!("Formatted:\n{}", formatted);
    
    // Lint ITML
    let lint_opts = LintOptions::default();
    let diagnostics = lint(&doc, &lint_opts);
    for diagnostic in diagnostics {
        println!("Lint: {} - {}", diagnostic.code, diagnostic.message);
    }
    
    Ok(())
}
```

### WebAssembly (Browser/Node.js)

```bash
npm install @intent/itml-parser-wasm
```

```javascript
import init, { parse_itml, format_itml, lint_itml } from '@intent/itml-parser-wasm';

await init();

const itmlSource = `
  schema "Address"
  description: "A postal address schema"
  
  fields:
    street: string
    city: string
    state: string
    zip: string
`;

// Parse ITML
const ast = parse_itml(itmlSource);
console.log('Parsed AST:', ast);

// Format ITML
const formatted = format_itml(itmlSource, 2);
console.log('Formatted ITML:', formatted);

// Lint ITML
const diagnostics = lint_itml(itmlSource, ['ITML001', 'ITML002']);
console.log('Lint diagnostics:', diagnostics);
```

## API Reference

### Rust API

#### `parse(input: &str, opts: &ParseOptions) -> Result<Document, ParseError>`

Parses ITML source code into a typed AST.

**Parameters:**
- `input`: The ITML source code to parse
- `opts`: Parse options (tabs, line endings, etc.)

**Returns:** `Document` AST or `ParseError`

#### `format(doc: &Document, opts: &FormatOptions) -> String`

Formats a Document AST into canonical ITML text.

**Parameters:**
- `doc`: The Document AST to format
- `opts`: Format options (indentation, trailing newline, etc.)

**Returns:** Formatted ITML string

#### `lint(doc: &Document, opts: &LintOptions) -> Vec<Diagnostic>`

Lints a Document AST and returns diagnostic information.

**Parameters:**
- `doc`: The Document AST to lint
- `opts`: Lint options (enabled rules, fix mode, etc.)

**Returns:** Vector of diagnostic objects

### WASM API

#### `parse_itml(input: string): JsValue`

Parses ITML source code and returns the AST as a JavaScript object.

#### `format_itml(input: string, indent?: number): string`

Formats ITML source code with consistent indentation.

#### `lint_itml(input: string, rules?: string[]): JsValue`

Lints ITML source code and returns diagnostic information.

## Lint Rules

| Code | Description | Level |
|------|-------------|-------|
| ITML001 | Every intent must have inputs and (workflow or rules) | Error |
| ITML002 | Avoid "*" in network.allow | Warning |
| ITML003 | Routes targets must exist | Error |
| ITML004 | Tests required when rules exist | Warning |
| ITML005 | Do not mix tabs and spaces | Error |
| ITML006 | Unknown keys warn with suggestions | Warning |

## ITML v0.1 Language Features

### Top-Level Blocks

- **App**: Application definitions with routes, components, and themes
- **Intent**: Intent definitions with inputs, outputs, workflow, and rules
- **Schema**: Data structure definitions with fields and validation
- **Component**: Reusable UI component definitions
- **Layout**: Layout structure definitions
- **Policy**: Security and access policy definitions

### Type System

- **Primitive types**: `string`, `number`, `boolean`, `datetime`, `uuid`, `bytes`
- **Composite types**: `list(T)`, `map(T)`, `enum(...)`
- **Named types**: References to defined schemas

### Example Files

See the `examples/` directory for sample ITML files:
- `address.valid.itml` - Valid address schema
- `address.invalid.itml` - Invalid syntax example
- `project.app.itml` - Complete application definition
- `send_email.intent.itml` - Complex intent with workflow

## Development

### Prerequisites

- Rust 1.78+
- Node.js 18+ (for WASM development)
- wasm-pack (for WASM builds)

### Building

```bash
# Build Rust crate
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build WASM package
make wasm

# Run all tests including WASM
make test-all
```

### Project Structure

```
itml-parser/
├── src/
│   ├── ast.rs           # AST type definitions
│   ├── parser.rs        # ITML parser implementation
│   ├── formatter.rs     # Pretty printer
│   ├── linter/          # Lint rules and diagnostics
│   ├── wasm.rs          # WASM bindings
│   └── lib.rs           # Public API
├── grammar/
│   └── itml.pest        # Pest grammar definition
├── examples/            # Sample ITML files
├── tests/               # Test suites
├── benches/             # Performance benchmarks
├── wasm/                # WASM package files
└── .github/workflows/   # CI/CD configuration
```

## Performance

The parser is optimized for performance with:
- Zero-copy string handling where possible
- Efficient AST construction
- Minimal memory allocations
- Fast error reporting with precise spans

Benchmark results (on modern hardware):
- Parse 1000-field schema: ~50μs
- Format 1000-field schema: ~30μs
- Lint 1000-field schema: ~20μs

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation
- Run `cargo fmt` and `cargo clippy`
- Ensure all tests pass

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [Intent Registry](https://github.com/intent-ecosystem/intent-registry) - The main Intent ecosystem
- [ITML Specification](https://github.com/intent-ecosystem/itml-spec) - ITML v0.1 language specification

## Changelog

### v0.1.0 (Initial Release)

- Initial implementation of ITML v0.1 parser
- AST-based formatter with idempotent output
- Linter with ITML001-ITML006 rules
- WASM bindings for web and Node.js
- Comprehensive test suite and benchmarks
- CI/CD with GitHub Actions