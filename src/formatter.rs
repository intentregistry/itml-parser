use crate::ast::*;

/// Format options
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub indent: usize,
    pub trailing_newline: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent: 2,
            trailing_newline: true,
        }
    }
}

/// Format a Document AST into a pretty-printed string
pub fn format(doc: &Document, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    match &doc.kind {
        TopLevel::Schema(schema) => {
            result.push_str(&format_schema(schema, opts));
        }
        TopLevel::App(app) => {
            result.push_str(&format_app(app, opts));
        }
        TopLevel::Intent(intent) => {
            result.push_str(&format_intent(intent, opts));
        }
        TopLevel::Component(component) => {
            result.push_str(&format_component(component, opts));
        }
        TopLevel::Layout(layout) => {
            result.push_str(&format_layout(layout, opts));
        }
        TopLevel::Policy(policy) => {
            result.push_str(&format_policy(policy, opts));
        }
    }
    
    if opts.trailing_newline {
        result.push('\n');
    }
    
    result
}

fn format_schema(schema: &Schema, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // Schema header
    result.push_str(&format!("schema \"{}\"\n", schema.name.value));
    
    // Description
    if let Some(desc) = &schema.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Fields
    if !schema.fields.is_empty() {
        result.push_str("fields:\n");
        for field in &schema.fields {
            let field_str = format_field(field, opts);
            result.push_str(&format!("{}{}\n", " ".repeat(opts.indent), field_str));
        }
        result.push('\n');
    }
    
    // Validation
    if !schema.validation.is_empty() {
        result.push_str("validation:\n");
        for validation in &schema.validation {
            let validation_str = format_validation_rule(validation);
            result.push_str(&format!("{}- {}\n", " ".repeat(opts.indent), validation_str));
        }
    }
    
    result
}

fn format_app(app: &App, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // App header
    result.push_str(&format!("app \"{}\"\n", app.name.value));
    
    // Description
    if let Some(desc) = &app.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Imports
    if !app.imports.is_empty() {
        result.push_str("imports:\n");
        for import in &app.imports {
            result.push_str(&format!("{}- \"{}\"\n", " ".repeat(opts.indent), import.value));
        }
        result.push('\n');
    }
    
    // Routes
    if !app.routes.is_empty() {
        result.push_str("routes:\n");
        for route in &app.routes {
            result.push_str(&format!("{}- path: \"{}\"\n", " ".repeat(opts.indent), route.path.value));
            result.push_str(&format!("{}  component: \"{}\"\n", " ".repeat(opts.indent), route.component.value));
        }
        result.push('\n');
    }
    
    // Components
    if !app.components.is_empty() {
        result.push_str("components:\n");
        for component in &app.components {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), component.name.value));
            
            if !component.inputs.is_empty() {
                result.push_str(&format!("{}  inputs:\n", " ".repeat(opts.indent)));
                for input in &component.inputs {
                    result.push_str(&format!("{}    - name: \"{}\"\n", " ".repeat(opts.indent), input.name.value));
                }
            }
            
            if !component.outputs.is_empty() {
                result.push_str(&format!("{}  outputs:\n", " ".repeat(opts.indent)));
                for output in &component.outputs {
                    result.push_str(&format!("{}    - name: \"{}\"\n", " ".repeat(opts.indent), output.name.value));
                }
            }
        }
        result.push('\n');
    }
    
    // Theme
    if let Some(theme) = &app.theme {
        result.push_str("theme:\n");
        for (key, value) in &theme.properties {
            result.push_str(&format!("{}: \"{}\"\n", key, value.value));
        }
    }
    
    result
}

fn format_intent(intent: &Intent, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // Intent header
    result.push_str(&format!("intent \"{}\"\n", intent.name.value));
    
    // Description
    if let Some(desc) = &intent.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Inputs
    if !intent.inputs.is_empty() {
        result.push_str("inputs:\n");
        for input in &intent.inputs {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), input.name.value));
        }
        result.push('\n');
    }
    
    // Outputs
    if !intent.outputs.is_empty() {
        result.push_str("outputs:\n");
        for output in &intent.outputs {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), output.name.value));
        }
        result.push('\n');
    }
    
    // Workflow
    if !intent.workflow.is_empty() {
        result.push_str("workflow:\n");
        for step in &intent.workflow {
            result.push_str(&format!("{}- step: \"{}\"\n", " ".repeat(opts.indent), step.step.value));
            result.push_str(&format!("{}  action: \"{}\"\n", " ".repeat(opts.indent), step.action.value));
        }
        result.push('\n');
    }
    
    // Rules
    if !intent.rules.is_empty() {
        result.push_str("rules:\n");
        for rule in &intent.rules {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), rule.name.value));
            result.push_str(&format!("{}  condition: \"{}\"\n", " ".repeat(opts.indent), rule.condition.value));
            result.push_str(&format!("{}  action: \"{}\"\n", " ".repeat(opts.indent), rule.action.value));
        }
        result.push('\n');
    }
    
    // Tests
    if !intent.tests.is_empty() {
        result.push_str("tests:\n");
        for test in &intent.tests {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), test.name.value));
        }
    }
    
    result
}

fn format_component(component: &Component, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // Component header
    result.push_str(&format!("component \"{}\"\n", component.name.value));
    
    // Description
    if let Some(desc) = &component.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Inputs
    if !component.inputs.is_empty() {
        result.push_str("inputs:\n");
        for input in &component.inputs {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), input.name.value));
        }
        result.push('\n');
    }
    
    // Outputs
    if !component.outputs.is_empty() {
        result.push_str("outputs:\n");
        for output in &component.outputs {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), output.name.value));
        }
    }
    
    result
}

fn format_layout(layout: &Layout, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // Layout header
    result.push_str(&format!("layout \"{}\"\n", layout.name.value));
    
    // Description
    if let Some(desc) = &layout.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Structure
    if !layout.structure.is_empty() {
        result.push_str("structure:\n");
        for item in &layout.structure {
            result.push_str(&format!("{}\"{}\"\n", " ".repeat(opts.indent), item.value));
        }
    }
    
    result
}

fn format_policy(policy: &Policy, opts: &FormatOptions) -> String {
    let mut result = String::new();
    
    // Policy header
    result.push_str(&format!("policy \"{}\"\n", policy.name.value));
    
    // Description
    if let Some(desc) = &policy.description {
        result.push_str(&format!("description: \"{}\"\n", desc.value));
    }
    
    result.push('\n');
    
    // Rules
    if !policy.rules.is_empty() {
        result.push_str("rules:\n");
        for rule in &policy.rules {
            result.push_str(&format!("{}- name: \"{}\"\n", " ".repeat(opts.indent), rule.name.value));
            result.push_str(&format!("{}  condition: \"{}\"\n", " ".repeat(opts.indent), rule.condition.value));
            result.push_str(&format!("{}  action: \"{}\"\n", " ".repeat(opts.indent), rule.action.value));
        }
    }
    
    result
}

fn format_field(field: &Field, _opts: &FormatOptions) -> String {
    let mut result = format!("{}: {}", field.name.name, format_type_ref(&field.ty));
    
    if let Some(default) = &field.default_value {
        result.push_str(&format!(" = {}", format_value(default)));
    }
    
    result
}

fn format_validation_rule(rule: &ValidationRule) -> String {
    match rule {
        ValidationRule::Required => "required".to_string(),
        ValidationRule::Pattern(pattern) => format!("pattern(\"{}\")", pattern.value),
    }
}

fn format_type_ref(ty: &TypeRef) -> String {
    match ty {
        TypeRef::String => "string".to_string(),
        TypeRef::Number => "number".to_string(),
        TypeRef::Boolean => "boolean".to_string(),
        TypeRef::DateTime => "datetime".to_string(),
        TypeRef::Uuid => "uuid".to_string(),
        TypeRef::Bytes => "bytes".to_string(),
        TypeRef::Enum(variants) => {
            let variant_strs: Vec<String> = variants.iter().map(|v| format!("\"{}\"", v.value)).collect();
            format!("enum({})", variant_strs.join(", "))
        }
        TypeRef::List(inner) => format!("list({})", format_type_ref(inner)),
        TypeRef::Map(inner) => format!("map({})", format_type_ref(inner)),
        TypeRef::Named(name) => name.name.clone(),
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s.value),
        Value::Number(n) => n.value.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Regex(r) => format!("/{}/{}", r.pattern, r.flags),
    }
}
