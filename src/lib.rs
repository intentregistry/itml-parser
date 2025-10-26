pub mod ast;
pub mod parser;
pub mod formatter;
pub mod linter;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use ast::*;
pub use parser::{parse, ParseOptions, ParseError};
pub use formatter::{format, FormatOptions};
pub use linter::{lint, LintOptions, LintRule, Diagnostic, DiagnosticLevel};

// Re-export commonly used types
pub type Document = ast::Document;
pub type TopLevel = ast::TopLevel;
pub type Span = ast::Span;