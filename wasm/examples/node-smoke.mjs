import init, { parse_itml, format_itml, lint_itml, get_version } from './pkg/itml_parser.js';

async function main() {
    // Initialize the WASM module
    await init();
    
    console.log('ITML Parser WASM version:', get_version());
    
    const itmlSource = `schema "Address"
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
  - zip: pattern("^[0-9]{5}(-[0-9]{4})?$")`;

    try {
        // Test parsing
        console.log('\n=== Testing Parse ===');
        const ast = parse_itml(itmlSource);
        console.log('Parsed AST:', JSON.stringify(ast, null, 2));
        
        // Test formatting
        console.log('\n=== Testing Format ===');
        const formatted = format_itml(itmlSource, 2);
        console.log('Formatted ITML:');
        console.log(formatted);
        
        // Test linting
        console.log('\n=== Testing Lint ===');
        const diagnostics = lint_itml(itmlSource, ['ITML001', 'ITML002', 'ITML003', 'ITML004', 'ITML005', 'ITML006']);
        console.log('Lint diagnostics:', JSON.stringify(diagnostics, null, 2));
        
        console.log('\n✅ All tests passed!');
        
    } catch (error) {
        console.error('❌ Test failed:', error);
        process.exit(1);
    }
}

main().catch(console.error);
