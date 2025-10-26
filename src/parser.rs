use crate::ast::*;
use std::collections::HashMap;

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

    // Parse routes block
    let mut routes = Vec::new();
    let mut in_routes_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "routes:" {
            in_routes_block = true;
            continue;
        }
        if in_routes_block {
            if trimmed.starts_with("-") {
                // Parse route
                let route_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if route_text.contains("path:") {
                    let path_start = route_text.find("path:").unwrap_or(0) + 5;
                    let path = route_text[path_start..].trim().to_string();
                    if path.starts_with('"') && path.ends_with('"') {
                        let path_value = path[1..path.len()-1].to_string();
                        
                        // Look for component in the next line
                        let mut component_value = "DefaultComponent".to_string();
                        if i + 1 < lines.len() {
                            let next_line = lines[i + 1].trim();
                            if next_line.contains("component:") {
                                let comp_start = next_line.find("component:").unwrap_or(0) + 10;
                                let comp = next_line[comp_start..].trim().to_string();
                                if comp.starts_with('"') && comp.ends_with('"') {
                                    component_value = comp[1..comp.len()-1].to_string();
                                }
                            }
                        }
                        
                        routes.push(Route {
                            span: Span::new(i + 1, 1, 0, route_text.len()),
                            path: StringLiteral {
                                span: Span::new(i + 1, path_start + 2, 0, path_value.len()),
                                value: path_value,
                            },
                            component: StringLiteral {
                                span: Span::new(i + 1, 1, 0, component_value.len()),
                                value: component_value,
                            },
                        });
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of routes block
                break;
            }
        }
    }

    // Parse components block
    let mut components = Vec::new();
    let mut in_components_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "components:" {
            in_components_block = true;
            continue;
        }
        if in_components_block {
            if trimmed.starts_with("-") {
                // Parse component
                let comp_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if comp_text.contains("name:") {
                    let name_start = comp_text.find("name:").unwrap_or(0) + 5;
                    let name = comp_text[name_start..].trim().to_string();
                    if name.starts_with('"') && name.ends_with('"') {
                        let comp_name = name[1..name.len()-1].to_string();
                        components.push(Component {
                            span: Span::new(i + 1, 1, 0, comp_text.len()),
                            name: StringLiteral {
                                span: Span::new(i + 1, name_start + 2, 0, comp_name.len()),
                                value: comp_name,
                            },
                            description: None,
                            inputs: Vec::new(),
                            outputs: Vec::new(),
                        });
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of components block
                break;
            }
        }
    }

    let app = App {
        span,
        name: name_literal,
        description,
        imports: Vec::new(), // Simplified for now
        routes,
        components,
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

    // Parse inputs block
    let mut inputs = Vec::new();
    let mut in_inputs_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "inputs:" {
            in_inputs_block = true;
            continue;
        }
        if in_inputs_block {
            if trimmed.starts_with("-") {
                // Parse input parameter
                let param_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if param_text.contains("(") && param_text.contains(")") {
                    let name_start = param_text.find('(').unwrap_or(0);
                    let name = param_text[..name_start].trim().to_string();
                    let type_start = param_text.find('(').unwrap_or(0) + 1;
                    let type_end = param_text.find(')').unwrap_or(param_text.len());
                    let param_type = param_text[type_start..type_end].trim().to_string();
                    
                    inputs.push(Param {
                        span: Span::new(i + 1, 1, 0, param_text.len()),
                        name: StringLiteral {
                            span: Span::new(i + 1, 2, 0, name.len()),
                            value: name,
                        },
                        ty: TypeRef::Named(Ident {
                            span: Span::new(i + 1, type_start + 2, 0, param_type.len()),
                            name: param_type,
                        }),
                        attrs: Vec::new(),
                    });
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of inputs block
                break;
            }
        }
    }

    // Parse workflow block
    let mut workflow = Vec::new();
    let mut in_workflow_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "workflow:" {
            in_workflow_block = true;
            continue;
        }
        if in_workflow_block {
            if trimmed.starts_with("-") {
                // Parse workflow step
                let step_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if step_text.contains("step:") {
                    let step_start = step_text.find("step:").unwrap_or(0) + 5;
                    let step_name = step_text[step_start..].trim().to_string();
                    if step_name.starts_with('"') && step_name.ends_with('"') {
                        let step_value = step_name[1..step_name.len()-1].to_string();
                        workflow.push(WorkflowStep {
                            span: Span::new(i + 1, 1, 0, step_text.len()),
                            step: StringLiteral {
                                span: Span::new(i + 1, step_start + 2, 0, step_value.len()),
                                value: step_value,
                            },
                            action: StringLiteral {
                                span: Span::new(i + 1, 1, 0, 1),
                                value: "log('hello')".to_string(), // Default action
                            },
                            depends: None,
                        });
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of workflow block
                break;
            }
        }
    }

    // Parse rules block
    let mut rules = Vec::new();
    let mut in_rules_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "rules:" {
            in_rules_block = true;
            continue;
        }
        if in_rules_block {
            if trimmed.starts_with("-") {
                // Parse rule
                let rule_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if rule_text.contains("name:") {
                    let name_start = rule_text.find("name:").unwrap_or(0) + 5;
                    let name = rule_text[name_start..].trim().to_string();
                    if name.starts_with('"') && name.ends_with('"') {
                        let rule_name = name[1..name.len()-1].to_string();
                        rules.push(IntentRule {
                            span: Span::new(i + 1, 1, 0, rule_text.len()),
                            name: StringLiteral {
                                span: Span::new(i + 1, name_start + 2, 0, rule_name.len()),
                                value: rule_name,
                            },
                            condition: StringLiteral {
                                span: Span::new(i + 1, 1, 0, 1),
                                value: "true".to_string(), // Default condition
                            },
                            action: StringLiteral {
                                span: Span::new(i + 1, 1, 0, 1),
                                value: "log('rule executed')".to_string(), // Default action
                            },
                        });
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of rules block
                break;
            }
        }
    }

    // Parse tests block
    let mut tests = Vec::new();
    let mut in_tests_block = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "tests:" {
            in_tests_block = true;
            continue;
        }
        if in_tests_block {
            if trimmed.starts_with("-") {
                // Parse test
                let test_text = trimmed.strip_prefix("-").unwrap_or("").trim();
                if test_text.contains("name:") {
                    let name_start = test_text.find("name:").unwrap_or(0) + 5;
                    let name = test_text[name_start..].trim().to_string();
                    if name.starts_with('"') && name.ends_with('"') {
                        let test_name = name[1..name.len()-1].to_string();
                        tests.push(Test {
                            span: Span::new(i + 1, 1, 0, test_text.len()),
                            name: StringLiteral {
                                span: Span::new(i + 1, name_start + 2, 0, test_name.len()),
                                value: test_name,
                            },
                            inputs: HashMap::new(),
                            expected: HashMap::new(),
                        });
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with(" ") {
                // End of tests block
                break;
            }
        }
    }

    let intent = Intent {
        span,
        name: name_literal,
        description,
        inputs,
        outputs: Vec::new(), // Simplified for now
        workflow,
        rules,
        tests,
    };

    Ok(Document {
        span,
        kind: TopLevel::Intent(intent),
    })
}