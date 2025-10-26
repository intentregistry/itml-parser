#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A span representing a location in the source code
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, offset: usize, length: usize) -> Self {
        Self {
            line,
            column,
            offset,
            length,
        }
    }
}

/// The root document node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Document {
    pub span: Span,
    pub kind: TopLevel,
}

/// Top-level block types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TopLevel {
    App(App),
    Intent(Intent),
    Layout(Layout),
    Component(Component),
    Schema(Schema),
    Policy(Policy),
}

/// App block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct App {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub imports: Vec<StringLiteral>,
    pub routes: Vec<Route>,
    pub components: Vec<Component>,
    pub theme: Option<Theme>,
}

/// Intent block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Intent {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
    pub workflow: Vec<WorkflowStep>,
    pub rules: Vec<IntentRule>,
    pub tests: Vec<Test>,
}

/// Schema block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Schema {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub fields: Vec<Field>,
    pub validation: Vec<ValidationRule>,
}

/// Component block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Component {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
}

/// Layout block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layout {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub structure: Vec<StringLiteral>,
}

/// Policy block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Policy {
    pub span: Span,
    pub name: StringLiteral,
    pub description: Option<StringLiteral>,
    pub rules: Vec<IntentRule>,
}

/// Route definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Route {
    pub span: Span,
    pub path: StringLiteral,
    pub component: StringLiteral,
}

/// Workflow step
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkflowStep {
    pub span: Span,
    pub step: StringLiteral,
    pub action: StringLiteral,
    pub depends: Option<Vec<StringLiteral>>,
}

/// Rule definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntentRule {
    pub span: Span,
    pub name: StringLiteral,
    pub condition: StringLiteral,
    pub action: StringLiteral,
}

/// Test case
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Test {
    pub span: Span,
    pub name: StringLiteral,
    pub inputs: HashMap<String, Value>,
    pub expected: HashMap<String, Value>,
}

/// Field definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Field {
    pub span: Span,
    pub name: Ident,
    pub ty: TypeRef,
    pub default_value: Option<Value>,
}

/// Parameter definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Param {
    pub span: Span,
    pub name: StringLiteral,
    pub ty: TypeRef,
    pub attrs: Vec<ParamAttr>,
}

/// Parameter attributes
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ParamAttr {
    Required(bool),
    Default(Value),
    Pattern(StringLiteral),
    In(StringLiteral),
}

/// Validation rule
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidationRule {
    Required,
    Pattern(StringLiteral),
}

/// Theme definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Theme {
    pub span: Span,
    pub properties: HashMap<String, StringLiteral>,
}

/// Type reference
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeRef {
    String,
    Number,
    Boolean,
    DateTime,
    Uuid,
    Bytes,
    Enum(Vec<StringLiteral>),
    List(Box<TypeRef>),
    Map(Box<TypeRef>),
    Named(Ident),
}

/// Value types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    String(StringLiteral),
    Number(Number),
    Boolean(bool),
    Regex(RegexLiteral),
}

/// String literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringLiteral {
    pub span: Span,
    pub value: String,
}

/// Number literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Number {
    pub span: Span,
    pub value: f64,
}

/// Regex literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegexLiteral {
    pub span: Span,
    pub pattern: String,
    pub flags: String,
}

/// Identifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ident {
    pub span: Span,
    pub name: String,
}
