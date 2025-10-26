use itml_parser::{parse, format, ParseOptions, FormatOptions};

#[test]
fn test_format_schema() {
    let input = r#"schema "Address"
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
  - zip: pattern("^[0-9]{5}(-[0-9]{4})?$")"#;

    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let formatted = format(&doc, &format_opts);
    
    // The formatted output should contain the schema name and description
    assert!(formatted.contains("schema \"Address\""));
    assert!(formatted.contains("description: \"A postal address schema\""));
}

#[test]
fn test_format_app() {
    let input = r#"app "Project Management"
description: "A project management application"

imports:
  - "@intent/ui-components"
  - "./schemas/address"

routes:
  - path: "/projects"
    component: "ProjectList"
  - path: "/projects/:id"
    component: "ProjectDetail"

components:
  - name: "ProjectList"
    inputs:
      - name: "projects" (list(Project))
    outputs:
      - name: "selected" (Project)
    
  - name: "ProjectDetail"
    inputs:
      - name: "project" (Project)
      - name: "address" (Address)
    outputs:
      - name: "updated" (Project)

theme:
  primary: "007bff"
  secondary: "6c757d"
"#;

    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let formatted = format(&doc, &format_opts);
    
    // The formatted output should contain the app name and description
    assert!(formatted.contains("app \"Project Management\""));
    assert!(formatted.contains("description: \"A project management application\""));
}

#[test]
fn test_format_intent() {
    let input = r#"intent "Send Email"
description: "Send an email to a recipient"

inputs:
  - name: "to" (string)
    required: true
    pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
  - name: "subject" (string)
    required: true
  - name: "body" (string)
    required: true
  - name: "priority" (enum("low", "normal", "high"))
    default: "normal"

outputs:
  - name: "messageId" (string)
  - name: "status" (enum("sent", "failed", "queued"))

workflow:
  - step: "validate"
    action: "validate_email"
  - step: "send"
    action: "send_email"
    depends: ["validate"]
  - step: "notify"
    action: "notify_sender"
    depends: ["send"]

rules:
  - name: "rate_limit"
    condition: "count_last_hour < 100"
    action: "allow"
  - name: "spam_check"
    condition: "body contains spam_keywords"
    action: "reject"

tests:
  - name: "valid_email"
    inputs:
      to: "test@example.com"
      subject: "Test"
      body: "Hello world"
    expected:
      status: "sent"
  
  - name: "invalid_email"
    inputs:
      to: "invalid-email"
      subject: "Test"
      body: "Hello world"
    expected:
      status: "failed"
"#;

    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions::default();
    
    let doc = parse(input, &parse_opts).expect("Failed to parse");
    let formatted = format(&doc, &format_opts);
    
    // The formatted output should contain the intent name and description
    assert!(formatted.contains("intent \"Send Email\""));
    assert!(formatted.contains("description: \"Send an email to a recipient\""));
}

#[test]
fn test_format_idempotency() {
    let input = r#"schema "Test"
description: "A test schema""#;

    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions::default();
    
    // Parse and format once
    let doc1 = parse(input, &parse_opts).expect("Failed to parse");
    let formatted1 = format(&doc1, &format_opts);
    
    // Parse and format again
    let doc2 = parse(&formatted1, &parse_opts).expect("Failed to parse formatted output");
    let formatted2 = format(&doc2, &format_opts);
    
    // The two formatted outputs should be identical (idempotency)
    assert_eq!(formatted1, formatted2);
}
