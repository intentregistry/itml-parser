# @intent/itml-parser-wasm

ITML v0.1 parser, formatter, and linter compiled to WebAssembly for use in web browsers and Node.js.

## Installation

```bash
npm install @intent/itml-parser-wasm
```

## Usage

### Browser

```html
<script type="module">
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
</script>
```

### Node.js

```javascript
const { parse_itml, format_itml, lint_itml } = require('@intent/itml-parser-wasm');

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

### `parse_itml(input: string): JsValue`

Parses ITML source code and returns the AST as a JavaScript object.

**Parameters:**
- `input` (string): The ITML source code to parse

**Returns:** JavaScript object representing the parsed AST

**Throws:** Error if parsing fails

### `format_itml(input: string, indent?: number): string`

Formats ITML source code with consistent indentation and structure.

**Parameters:**
- `input` (string): The ITML source code to format
- `indent` (number, optional): Number of spaces for indentation (default: 2)

**Returns:** Formatted ITML string

**Throws:** Error if parsing fails

### `lint_itml(input: string, rules?: string[]): JsValue`

Lints ITML source code and returns diagnostic information.

**Parameters:**
- `input` (string): The ITML source code to lint
- `rules` (string[], optional): Array of lint rule codes to enable (default: all rules)

**Returns:** Array of diagnostic objects

**Throws:** Error if parsing fails

### `get_version(): string`

Returns the version of the ITML parser.

**Returns:** Version string

## Lint Rules

- **ITML001**: Every intent must have inputs and (workflow or rules)
- **ITML002**: Avoid "*" in network.allow
- **ITML003**: Routes targets must exist
- **ITML004**: Tests required when rules exist
- **ITML005**: Do not mix tabs and spaces
- **ITML006**: Unknown keys warn with suggestions

## Building from Source

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for bundlers (webpack, rollup, etc.)
npm run build

# Build for Node.js
npm run build-node

# Build for web (direct browser use)
npm run build-web
```

## License

MIT
