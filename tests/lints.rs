use itml_parser::{parse, lint, ParseOptions, LintOptions, LintRule, DiagnosticLevel};

#[test]
fn test_lint_intent_missing_inputs() {
    let input = r#"intent "Test Intent"
description: "A test intent without inputs"

workflow:
  - step: "test"
    action: "test_action""#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should have ITML001 error for missing inputs
    let itml001_errors: Vec<_> = diagnostics.iter()
        .filter(|d| d.code == "ITML001" && d.level == DiagnosticLevel::Error)
        .collect();
    
    assert!(!itml001_errors.is_empty());
    assert!(itml001_errors[0].message.contains("inputs"));
}

#[test]
fn test_lint_intent_missing_workflow_and_rules() {
    let input = r#"intent "Test Intent"
description: "A test intent without workflow or rules"

inputs:
  - name: "test" (string)"#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should have ITML001 error for missing workflow/rules
    let itml001_errors: Vec<_> = diagnostics.iter()
        .filter(|d| d.code == "ITML001" && d.level == DiagnosticLevel::Error)
        .collect();
    
    assert!(!itml001_errors.is_empty());
    assert!(itml001_errors[0].message.contains("workflow or rules"));
}

#[test]
fn test_lint_intent_valid() {
    let input = r#"intent "Test Intent"
description: "A valid test intent"

inputs:
  - name: "test" (string)

workflow:
  - step: "test"
    action: "test_action""#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should not have ITML001 errors
    let itml001_errors: Vec<_> = diagnostics.iter()
        .filter(|d| d.code == "ITML001")
        .collect();
    
    assert!(itml001_errors.is_empty());
}

#[test]
fn test_lint_app_missing_component() {
    let input = r#"app "Test App"
description: "A test app with missing component"

routes:
  - path: "/test"
    component: "NonExistentComponent""#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should have ITML003 error for missing component
    let itml003_errors: Vec<_> = diagnostics.iter()
        .filter(|d| d.code == "ITML003" && d.level == DiagnosticLevel::Error)
        .collect();
    
    assert!(!itml003_errors.is_empty());
    assert!(itml003_errors[0].message.contains("NonExistentComponent"));
}

#[test]
fn test_lint_intent_rules_without_tests() {
    let input = r#"intent "Test Intent"
description: "A test intent with rules but no tests"

inputs:
  - name: "test" (string)

rules:
  - name: "test_rule"
    condition: "test_condition"
    action: "test_action""#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should have ITML004 warning for rules without tests
    let itml004_warnings: Vec<_> = diagnostics.iter()
        .filter(|d| d.code == "ITML004" && d.level == DiagnosticLevel::Warning)
        .collect();
    
    assert!(!itml004_warnings.is_empty());
    assert!(itml004_warnings[0].message.contains("tests"));
}

#[test]
fn test_lint_disabled_rules() {
    let input = r#"intent "Test Intent"
description: "A test intent without inputs"

workflow:
  - step: "test"
    action: "test_action""#;

    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions {
        rules: vec![], // No rules enabled
        fix: false,
    };
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let diagnostics = lint(&doc, &lint_opts);
    
    // Should have no diagnostics since no rules are enabled
    assert!(diagnostics.is_empty());
}
