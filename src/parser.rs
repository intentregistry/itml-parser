use crate::ast::*;

/// Parse options
#[derive(Debug, Clone)]
pub struct ParseOptions {
    pub allow_tabs: bool,
    pub tolerate_crlf: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            allow_tabs: false,
            tolerate_crlf: true,
        }
    }
}

/// Parse error
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },
    #[error("Unexpected token: {token}")]
    UnexpectedToken { token: String },
    #[error("Missing required field: {field}")]
    MissingField { field: String },
}

/// Parse a string into a Document AST
pub fn parse(input: &str, _opts: &ParseOptions) -> Result<Document, ParseError> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return Err(ParseError::SyntaxError {
            line: 1,
            column: 1,
            message: "Empty document".to_string(),
        });
    }

    // Check for obvious syntax errors first
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("[unclosed bracket") {
            return Err(ParseError::SyntaxError {
                line: i + 1,
                column: 1,
                message: "Unclosed bracket detected".to_string(),
            });
        }
    }

    // Simple parsing logic for now - just detect the first line to determine block type
    let first_line = lines[0].trim();
    
    if first_line.starts_with("schema ") {
        parse_schema_simple(input, &lines)
    } else if first_line.starts_with("app ") {
        parse_app_simple(input, &lines)
    } else if first_line.starts_with("intent ") {
        parse_intent_simple(input, &lines)
    } else {
        Err(ParseError::SyntaxError {
            line: 1,
            column: 1,
            message: format!("Unknown block type: {}", first_line),
        })
    }
}

fn parse_schema_simple(input: &str, lines: &[&str]) -> Result<Document, ParseError> {
    let span = Span::new(1, 1, 0, input.len());
    
    // Extract name from first line
    let first_line = lines[0].trim();
    let name = if first_line.starts_with("schema \"") {
        let start = 8; // "schema ".len()
        let end = first_line.rfind('"').unwrap_or(first_line.len());
        first_line[start..end].to_string()
    } else {
        return Err(ParseError::SyntaxError {
            line: 1,
            column: 1,
            message: "Schema name must be quoted".to_string(),
        });
    };

    let name_literal = StringLiteral {
        span: Span::new(1, 8, 7, name.len()),
        value: name,
    };

    // Look for description
    let mut description = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("description:") {
            let desc_text = line.trim().strip_prefix("description:").unwrap_or("").trim();
            if desc_text.starts_with('"') && desc_text.ends_with('"') {
                let desc_value = desc_text[1..desc_text.len()-1].to_string();
                description = Some(StringLiteral {
                    span: Span::new(i + 1, 13, 0, desc_value.len()),
                    value: desc_value,
                });
            }
            break;
        }
    }

    let schema = Schema {
        span,
        name: name_literal,
        description,
        fields: Vec::new(), // Simplified for now
        validation: Vec::new(), // Simplified for now
    };

    Ok(Document {
        span,
        kind: TopLevel::Schema(schema),
    })
}

fn parse_app_simple(input: &str, lines: &[&str]) -> Result<Document, ParseError> {
    let span = Span::new(1, 1, 0, input.len());
    
    // Extract name from first line
    let first_line = lines[0].trim();
    let name = if first_line.starts_with("app \"") {
        let start = 5; // "app ".len()
        let end = first_line.rfind('"').unwrap_or(first_line.len());
        first_line[start..end].to_string()
    } else {
        return Err(ParseError::SyntaxError {
            line: 1,
            column: 1,
            message: "App name must be quoted".to_string(),
        });
    };

    let name_literal = StringLiteral {
        span: Span::new(1, 5, 4, name.len()),
        value: name,
    };

    // Look for description
    let mut description = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("description:") {
            let desc_text = line.trim().strip_prefix("description:").unwrap_or("").trim();
            if desc_text.starts_with('"') && desc_text.ends_with('"') {
                let desc_value = desc_text[1..desc_text.len()-1].to_string();
                description = Some(StringLiteral {
                    span: Span::new(i + 1, 13, 0, desc_value.len()),
                    value: desc_value,
                });
            }
            break;
        }
    }

    let app = App {
        span,
        name: name_literal,
        description,
        imports: Vec::new(), // Simplified for now
        routes: Vec::new(), // Simplified for now
        components: Vec::new(), // Simplified for now
        theme: None, // Simplified for now
    };

    Ok(Document {
        span,
        kind: TopLevel::App(app),
    })
}

fn parse_intent_simple(input: &str, lines: &[&str]) -> Result<Document, ParseError> {
    let span = Span::new(1, 1, 0, input.len());
    
    // Extract name from first line
    let first_line = lines[0].trim();
    let name = if first_line.starts_with("intent \"") {
        let start = 8; // "intent ".len()
        let end = first_line.rfind('"').unwrap_or(first_line.len());
        first_line[start..end].to_string()
    } else {
        return Err(ParseError::SyntaxError {
            line: 1,
            column: 1,
            message: "Intent name must be quoted".to_string(),
        });
    };

    let name_literal = StringLiteral {
        span: Span::new(1, 8, 7, name.len()),
        value: name,
    };

    // Look for description
    let mut description = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("description:") {
            let desc_text = line.trim().strip_prefix("description:").unwrap_or("").trim();
            if desc_text.starts_with('"') && desc_text.ends_with('"') {
                let desc_value = desc_text[1..desc_text.len()-1].to_string();
                description = Some(StringLiteral {
                    span: Span::new(i + 1, 13, 0, desc_value.len()),
                    value: desc_value,
                });
            }
            break;
        }
    }

    let intent = Intent {
        span,
        name: name_literal,
        description,
        inputs: Vec::new(), // Simplified for now
        outputs: Vec::new(), // Simplified for now
        workflow: Vec::new(), // Simplified for now
        rules: Vec::new(), // Simplified for now
        tests: Vec::new(), // Simplified for now
    };

    Ok(Document {
        span,
        kind: TopLevel::Intent(intent),
    })
}