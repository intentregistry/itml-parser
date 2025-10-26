#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use crate::{parse, format, lint, ParseOptions, FormatOptions, LintOptions, LintRule, DiagnosticLevel};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_itml(input: &str) -> Result<JsValue, JsValue> {
    let opts = ParseOptions::default();
    match parse(input, &opts) {
        Ok(doc) => {
            // Convert Document to JsValue
            match serde_wasm_bindgen::to_value(&doc) {
                Ok(js_value) => Ok(js_value),
                Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e))),
            }
        }
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn format_itml(input: &str, indent: Option<usize>) -> Result<String, JsValue> {
    let parse_opts = ParseOptions::default();
    let format_opts = FormatOptions {
        indent: indent.unwrap_or(2),
        trailing_newline: true,
    };
    
    match parse(input, &parse_opts) {
        Ok(doc) => Ok(format(&doc, &format_opts)),
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn lint_itml(input: &str, rules: Option<Vec<String>>) -> Result<JsValue, JsValue> {
    let parse_opts = ParseOptions::default();
    let lint_opts = LintOptions {
        rules: rules.unwrap_or_default().into_iter()
            .filter_map(|r| match r.as_str() {
                "ITML001" => Some(LintRule::ITML001),
                "ITML002" => Some(LintRule::ITML002),
                "ITML003" => Some(LintRule::ITML003),
                "ITML004" => Some(LintRule::ITML004),
                "ITML005" => Some(LintRule::ITML005),
                "ITML006" => Some(LintRule::ITML006),
                _ => None,
            })
            .collect(),
        fix: false,
    };
    
    match parse(input, &parse_opts) {
        Ok(doc) => {
            let diagnostics = lint(&doc, &lint_opts);
            match serde_wasm_bindgen::to_value(&diagnostics) {
                Ok(js_value) => Ok(js_value),
                Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e))),
            }
        }
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
