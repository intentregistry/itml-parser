use itml_parser::{parse, ParseOptions, TopLevel};

#[test]
fn test_parse_valid_address_schema() {
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

    let opts = ParseOptions::default();
    let result = parse(input, &opts);
    
    match result {
        Ok(doc) => {
            match doc.kind {
                TopLevel::Schema(schema) => {
                    assert_eq!(schema.name.value, "Address");
                    assert!(schema.description.is_some());
                    assert_eq!(schema.description.unwrap().value, "A postal address schema");
                }
                _ => panic!("Expected schema block"),
            }
        }
        Err(e) => panic!("Parse failed: {:?}", e),
    }
}

#[test]
fn test_parse_valid_app() {
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

    let opts = ParseOptions::default();
    let result = parse(input, &opts);
    
    match result {
        Ok(doc) => {
            match doc.kind {
                TopLevel::App(app) => {
                    assert_eq!(app.name.value, "Project Management");
                    assert!(app.description.is_some());
                    assert_eq!(app.description.unwrap().value, "A project management application");
                }
                _ => panic!("Expected app block"),
            }
        }
        Err(e) => panic!("Parse failed: {:?}", e),
    }
}

#[test]
fn test_parse_valid_intent() {
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
      status: "failed""#;

    let opts = ParseOptions::default();
    let result = parse(input, &opts);
    
    match result {
        Ok(doc) => {
            match doc.kind {
                TopLevel::Intent(intent) => {
                    assert_eq!(intent.name.value, "Send Email");
                    assert!(intent.description.is_some());
                    assert_eq!(intent.description.unwrap().value, "Send an email to a recipient");
                }
                _ => panic!("Expected intent block"),
            }
        }
        Err(e) => panic!("Parse failed: {:?}", e),
    }
}

#[test]
fn test_parse_invalid_syntax() {
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
  - zip: pattern("^[0-9]{5}(-[0-9]{4})?$")

# This should cause a parse error - invalid syntax
invalid_field: [unclosed bracket"#;

    let opts = ParseOptions::default();
    let result = parse(input, &opts);
    
    // This should fail to parse due to invalid syntax
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_document() {
    let input = "";
    let opts = ParseOptions::default();
    let result = parse(input, &opts);
    
    // Empty document should fail
    assert!(result.is_err());
}
