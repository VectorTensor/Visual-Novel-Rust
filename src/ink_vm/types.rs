use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the entire Ink story JSON structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InkStory {
    #[serde(rename = "inkVersion")]
    pub ink_version: i32,
    pub root: serde_json::Value,
    #[serde(rename = "listDefs")]
    pub list_defs: HashMap<String, serde_json::Value>,
}

/// Represents a value in the Ink VM
#[derive(Debug, Clone)]
pub enum InkValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Null,
}

impl From<&serde_json::Value> for InkValue {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    InkValue::Int(i as i32)
                } else if let Some(f) = n.as_f64() {
                    InkValue::Float(f as f32)
                } else {
                    InkValue::Null
                }
            }
            serde_json::Value::String(s) => InkValue::String(s.clone()),
            serde_json::Value::Bool(b) => InkValue::Bool(*b),
            _ => InkValue::Null,
        }
    }
}

/// Choice presented to the player
#[derive(Debug, Clone)]
pub struct Choice {
    pub index: usize,
    pub text: String,
}

/// Output from the Ink VM
#[derive(Debug, Clone)]
pub enum InkOutput {
    Text(String),
    Tag(String),
    Choice(Choice),
    End,
}
