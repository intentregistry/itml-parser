use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itml_parser::{parse, format, lint, ParseOptions, FormatOptions, LintOptions};

fn generate_large_itml(size: usize) -> String {
    let mut content = String::new();
    
    // Generate a large schema with many fields
    content.push_str("schema \"LargeSchema\"\n");
    content.push_str("description: \"A large schema for benchmarking\"\n\n");
    content.push_str("fields:\n");
    
    for i in 0..size {
        content.push_str(&format!("  field{}: string\n", i));
    }
    
    content.push_str("\nvalidation:\n");
    for i in 0..(size / 10) {
        content.push_str(&format!("  - field{}: required\n", i));
    }
    
    content
}

fn benchmark_parse_large_schema(c: &mut Criterion) {
    let large_itml = generate_large_itml(1000);
    let opts = ParseOptions::default();
    
    c.bench_function("parse_large_schema_1000_fields", |b| {
        b.iter(|| {
            parse(black_box(&large_itml), black_box(&opts))
        })
    });
}

fn benchmark_parse_medium_schema(c: &mut Criterion) {
    let medium_itml = generate_large_itml(100);
    let opts = ParseOptions::default();
    
    c.bench_function("parse_medium_schema_100_fields", |b| {
        b.iter(|| {
            parse(black_box(&medium_itml), black_box(&opts))
        })
    });
}

fn benchmark_format_large_schema(c: &mut Criterion) {
    let large_itml = generate_large_itml(1000);
    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions::default();
    
    let doc = parse(&large_itml, &parse_opts).expect("Failed to parse");
    
    c.bench_function("format_large_schema_1000_fields", |b| {
        b.iter(|| {
            format(black_box(&doc), black_box(&format_opts))
        })
    });
}

fn benchmark_lint_large_schema(c: &mut Criterion) {
    let large_itml = generate_large_itml(1000);
    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions::default();
    
    let doc = parse(&large_itml, &parse_opts).expect("Failed to parse");
    
    c.bench_function("lint_large_schema_1000_fields", |b| {
        b.iter(|| {
            lint(black_box(&doc), black_box(&lint_opts))
        })
    });
}

fn benchmark_parse_app_with_components(c: &mut Criterion) {
    let app_itml = r#"app "Large App"
description: "A large app for benchmarking"

imports:
  - "@intent/ui-components"
  - "./schemas/address"
  - "./schemas/user"
  - "./schemas/product"
  - "./schemas/order"

routes:
  - path: "/dashboard"
    component: "Dashboard"
  - path: "/users"
    component: "UserList"
  - path: "/users/:id"
    component: "UserDetail"
  - path: "/products"
    component: "ProductList"
  - path: "/products/:id"
    component: "ProductDetail"
  - path: "/orders"
    component: "OrderList"
  - path: "/orders/:id"
    component: "OrderDetail"
  - path: "/settings"
    component: "Settings"

components:
  - name: "Dashboard"
    inputs:
      - name: "user" (User)
      - name: "stats" (Stats)
    outputs:
      - name: "refresh" (boolean)
  
  - name: "UserList"
    inputs:
      - name: "users" (list(User))
      - name: "filters" (UserFilters)
    outputs:
      - name: "selected" (User)
      - name: "filtered" (list(User))
  
  - name: "UserDetail"
    inputs:
      - name: "user" (User)
      - name: "editable" (boolean)
    outputs:
      - name: "updated" (User)
      - name: "deleted" (boolean)
  
  - name: "ProductList"
    inputs:
      - name: "products" (list(Product))
      - name: "category" (string)
    outputs:
      - name: "selected" (Product)
      - name: "filtered" (list(Product))
  
  - name: "ProductDetail"
    inputs:
      - name: "product" (Product)
      - name: "editable" (boolean)
    outputs:
      - name: "updated" (Product)
      - name: "deleted" (boolean)
  
  - name: "OrderList"
    inputs:
      - name: "orders" (list(Order))
      - name: "status" (OrderStatus)
    outputs:
      - name: "selected" (Order)
      - name: "filtered" (list(Order))
  
  - name: "OrderDetail"
    inputs:
      - name: "order" (Order)
      - name: "editable" (boolean)
    outputs:
      - name: "updated" (Order)
      - name: "deleted" (boolean)
  
  - name: "Settings"
    inputs:
      - name: "config" (Config)
    outputs:
      - name: "saved" (Config)

theme:
  primary: "007bff"
  secondary: "6c757d"
  success: "28a745"
  danger: "dc3545"
  warning: "ffc107"
  info: "17a2b8"
  light: "f8f9fa"
  dark: "343a40""#;

    let opts = ParseOptions::default();
    
    c.bench_function("parse_large_app", |b| {
        b.iter(|| {
            parse(black_box(app_itml), black_box(&opts))
        })
    });
}

fn benchmark_parse_intent_with_workflow(c: &mut Criterion) {
    let intent_itml = r#"intent "Complex Workflow Intent"
description: "A complex intent with many workflow steps for benchmarking"

inputs:
  - name: "user_id" (string)
    required: true
    pattern: "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
  - name: "action" (enum("create", "update", "delete", "read"))
    required: true
  - name: "data" (map(string))
    required: false
  - name: "options" (map(string))
    required: false
  - name: "priority" (enum("low", "normal", "high", "urgent"))
    default: "normal"

outputs:
  - name: "result" (map(string))
  - name: "status" (enum("success", "error", "partial"))
  - name: "message" (string)
  - name: "timestamp" (datetime)

workflow:
  - step: "validate_input"
    action: "validate_user_input"
  - step: "check_permissions"
    action: "check_user_permissions"
    depends: ["validate_input"]
  - step: "process_data"
    action: "process_user_data"
    depends: ["check_permissions"]
  - step: "validate_data"
    action: "validate_processed_data"
    depends: ["process_data"]
  - step: "execute_action"
    action: "execute_user_action"
    depends: ["validate_data"]
  - step: "log_action"
    action: "log_user_action"
    depends: ["execute_action"]
  - step: "notify_user"
    action: "notify_user_completion"
    depends: ["log_action"]
  - step: "cleanup"
    action: "cleanup_resources"
    depends: ["notify_user"]

rules:
  - name: "rate_limit"
    condition: "count_last_hour < 100"
    action: "allow"
  - name: "spam_check"
    condition: "data contains spam_keywords"
    action: "reject"
  - name: "permission_check"
    condition: "user.has_permission(action)"
    action: "allow"
  - name: "data_validation"
    condition: "data.is_valid()"
    action: "allow"
  - name: "resource_limit"
    condition: "system.resources_available()"
    action: "allow"

tests:
  - name: "valid_create_action"
    inputs:
      user_id: "123e4567-e89b-12d3-a456-426614174000"
      action: "create"
      data: '{"name": "test", "value": "123"}'
    expected:
      status: "success"
      message: "Action completed successfully"
  
  - name: "invalid_user_id"
    inputs:
      user_id: "invalid-uuid"
      action: "create"
      data: '{"name": "test"}'
    expected:
      status: "error"
      message: "Invalid user ID format"
  
  - name: "rate_limit_exceeded"
    inputs:
      user_id: "123e4567-e89b-12d3-a456-426614174000"
      action: "create"
      data: '{"name": "test"}'
    expected:
      status: "error"
      message: "Rate limit exceeded""#;

    let opts = ParseOptions::default();
    
    c.bench_function("parse_complex_intent", |b| {
        b.iter(|| {
            parse(black_box(intent_itml), black_box(&opts))
        })
    });
}

criterion_group!(
    benches,
    benchmark_parse_large_schema,
    benchmark_parse_medium_schema,
    benchmark_format_large_schema,
    benchmark_lint_large_schema,
    benchmark_parse_app_with_components,
    benchmark_parse_intent_with_workflow
);

criterion_main!(benches);
