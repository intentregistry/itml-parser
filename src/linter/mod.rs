use crate::ast::*;
use std::collections::HashSet;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Diagnostic level
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// Diagnostic information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    pub code: String,
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
    pub fix: Option<String>,
}

/// Lint rule enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LintRule {
    ITML001, // every intent has inputs and (workflow or rules)
    ITML002, // avoid "*" in network.allow
    ITML003, // routes targets must exist
    ITML004, // tests required when rules exist
    ITML005, // do not mix tabs and spaces
    ITML006, // unknown keys warn with suggestions
}

/// Lint options
#[derive(Debug, Clone)]
pub struct LintOptions {
    pub rules: Vec<LintRule>,
    pub fix: bool,
}

impl Default for LintOptions {
    fn default() -> Self {
        Self {
            rules: vec![
                LintRule::ITML001,
                LintRule::ITML002,
                LintRule::ITML003,
                LintRule::ITML004,
                LintRule::ITML005,
                LintRule::ITML006,
            ],
            fix: false,
        }
    }
}

/// Lint a Document AST
pub fn lint(doc: &Document, opts: &LintOptions) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let enabled_rules: HashSet<LintRule> = opts.rules.iter().cloned().collect();
    
    match &doc.kind {
        TopLevel::Intent(intent) => {
            if enabled_rules.contains(&LintRule::ITML001) {
                diagnostics.extend(lint_intent_structure(intent));
            }
            if enabled_rules.contains(&LintRule::ITML004) {
                diagnostics.extend(lint_tests_with_rules(intent));
            }
        }
        TopLevel::App(app) => {
            if enabled_rules.contains(&LintRule::ITML003) {
                diagnostics.extend(lint_route_targets(app));
            }
        }
        _ => {}
    }
    
    // ITML005: Check for mixed tabs and spaces (this would need the original source)
    if enabled_rules.contains(&LintRule::ITML005) {
        // This would require the original source text, which we don't have in the AST
        // For now, we'll skip this rule or implement it at the parser level
    }
    
    diagnostics
}

/// ITML001: every intent has inputs and (workflow or rules)
fn lint_intent_structure(intent: &Intent) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    // Check if intent has inputs
    if intent.inputs.is_empty() {
        diagnostics.push(Diagnostic {
            code: "ITML001".to_string(),
            level: DiagnosticLevel::Error,
            message: "Intent must have at least one input".to_string(),
            span: intent.span,
            hint: Some("Add an inputs section with at least one input parameter".to_string()),
            fix: None,
        });
    }
    
    // Check if intent has workflow or rules
    if intent.workflow.is_empty() && intent.rules.is_empty() {
        diagnostics.push(Diagnostic {
            code: "ITML001".to_string(),
            level: DiagnosticLevel::Error,
            message: "Intent must have either workflow or rules".to_string(),
            span: intent.span,
            hint: Some("Add either a workflow section or a rules section".to_string()),
            fix: None,
        });
    }
    
    diagnostics
}

/// ITML002: avoid "*" in network.allow
#[allow(dead_code)]
fn lint_network_allow(_doc: &Document) -> Vec<Diagnostic> {
    // This would require parsing network configuration, which isn't in our current AST
    // For now, return empty diagnostics
    Vec::new()
}

/// ITML003: routes targets must exist
fn lint_route_targets(app: &App) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    // Get all component names
    let component_names: HashSet<String> = app.components.iter()
        .map(|c| c.name.value.clone())
        .collect();
    
    // Check each route
    for route in &app.routes {
        if !component_names.contains(&route.component.value) {
            diagnostics.push(Diagnostic {
                code: "ITML003".to_string(),
                level: DiagnosticLevel::Error,
                message: format!("Route component '{}' does not exist", route.component.value),
                span: route.span,
                hint: Some("Make sure the component is defined in the components section".to_string()),
                fix: None,
            });
        }
    }
    
    diagnostics
}

/// ITML004: tests required when rules exist
fn lint_tests_with_rules(intent: &Intent) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    if !intent.rules.is_empty() && intent.tests.is_empty() {
        diagnostics.push(Diagnostic {
            code: "ITML004".to_string(),
            level: DiagnosticLevel::Warning,
            message: "Intent with rules should have tests".to_string(),
            span: intent.span,
            hint: Some("Add a tests section to verify your rules work correctly".to_string()),
            fix: None,
        });
    }
    
    diagnostics
}

/// ITML005: do not mix tabs and spaces
#[allow(dead_code)]
fn lint_mixed_indentation(_source: &str) -> Vec<Diagnostic> {
    // This would require the original source text
    // For now, return empty diagnostics
    Vec::new()
}

/// ITML006: unknown keys warn with suggestions
#[allow(dead_code)]
fn lint_unknown_keys(_doc: &Document) -> Vec<Diagnostic> {
    // This would require parsing unknown keys, which our current parser doesn't handle
    // For now, return empty diagnostics
    Vec::new()
}
