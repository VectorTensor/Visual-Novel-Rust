use super::types::*;
use serde_json::Value;
use std::collections::HashMap;

/// The Ink VM runtime that executes Ink stories
pub struct InkVM {
    story: InkStory,
    variables: HashMap<String, InkValue>,
    call_stack: Vec<String>,
    evaluation_stack: Vec<InkValue>,
    current_choices: Vec<Choice>,
    output_buffer: Vec<InkOutput>,
}

impl InkVM {
    /// Create a new Ink VM from a JSON string
    pub fn new(json_content: &str) -> Result<Self, String> {
        let story: InkStory = serde_json::from_str(json_content)
            .map_err(|e| format!("Failed to parse Ink JSON: {}", e))?;
        
        Ok(InkVM {
            story,
            variables: HashMap::new(),
            call_stack: vec!["start".to_string()],
            evaluation_stack: Vec::new(),
            current_choices: Vec::new(),
            output_buffer: Vec::new(),
        })
    }

    /// Initialize the VM and run global declarations
    pub fn initialize(&mut self) -> Result<(), String> {
        // Execute global declarations
        if let Some(root_obj) = self.story.root.as_array() {
            if root_obj.len() >= 3 {
                if let Some(global_decl) = root_obj[2].as_object() {
                    if let Some(decl_content) = global_decl.get("global decl") {
                        if let Some(decl_array) = decl_content.as_array() {
                            let decl_array = decl_array.clone();
                            self.execute_content(&decl_array)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Continue the story and get the next output
    pub fn continue_story(&mut self) -> Result<Vec<InkOutput>, String> {
        self.output_buffer.clear();
        self.current_choices.clear();

        if self.call_stack.is_empty() {
            return Ok(vec![InkOutput::End]);
        }

        // Pop the current knot to execute
        let current_knot = self.call_stack.pop().unwrap();
        
        // Get the knot content from the story
        if let Some(root_obj) = self.story.root.as_array() {
            if root_obj.len() >= 3 {
                if let Some(knots) = root_obj[2].as_object() {
                    if let Some(knot_content) = knots.get(&current_knot) {
                        if let Some(content_array) = knot_content.as_array() {
                            let content_array = content_array.clone();
                            self.execute_content(&content_array)?;
                        }
                    }
                }
            }
        }

        if self.output_buffer.is_empty() && self.current_choices.is_empty() && self.call_stack.is_empty() {
            self.output_buffer.push(InkOutput::End);
        }

        Ok(self.output_buffer.clone())
    }

    /// Choose a choice by index
    pub fn choose(&mut self, choice_index: usize) -> Result<(), String> {
        if choice_index >= self.current_choices.len() {
            return Err(format!("Invalid choice index: {}", choice_index));
        }
        
        // In a full implementation, this would navigate to the choice's target
        self.call_stack.clear();
        Ok(())
    }

    /// Get current available choices
    pub fn get_choices(&self) -> &[Choice] {
        &self.current_choices
    }

    /// Get current call stack depth
    pub fn get_call_stack_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Execute a content array
    fn execute_content(&mut self, content: &[Value]) -> Result<(), String> {
        let mut i = 0;
        while i < content.len() {
            let item = &content[i];
            
            match item {
                // String literal - add to output
                Value::String(s) => {
                    if s.starts_with('^') {
                        let text = s[1..].to_string();
                        // Filter out empty lines or just "\n" cleaned text
                        if !text.trim().is_empty() {
                            self.output_buffer.push(InkOutput::Text(text));
                        }
                    } else if s == "ev" {
                        // Start evaluation mode
                        i += 1;
                        while i < content.len() {
                            if content[i].as_str() == Some("/ev") {
                                break;
                            }
                            self.evaluate(&content[i])?;
                            i += 1;
                        }
                    } else if s == "done" || s == "end" {
                        return Ok(());
                    } else if s == "#" {
                        // Tag start
                        i += 1;
                        if let Some(tag_text) = content[i].as_str() {
                            if tag_text.starts_with('^') {
                                self.output_buffer.push(InkOutput::Tag(tag_text[1..].to_string()));
                            }
                        }
                        i += 1;
                        // Skip tag end "/#"
                        if content.get(i).and_then(|v| v.as_str()) == Some("/#") {
                            i += 1;
                        }
                        continue;
                    }
                }
                
                // Nested array - execute it
                Value::Array(arr) => {
                    self.execute_content(arr)?;
                }

                // Object - could be a variable assignment, choice, or control flow
                Value::Object(obj) => {
                    if let Some(var_name) = obj.get("VAR=") {
                        // Variable assignment
                        if let Some(var_str) = var_name.as_str() {
                            if let Some(value) = self.evaluation_stack.pop() {
                                self.variables.insert(var_str.to_string(), value);
                            }
                        }
                    } else if let Some(target) = obj.get("->") {
                        // Divert
                        if let Some(target_str) = target.as_str() {
                            // Simplify path - take the last part if it doesn't start with .
                            let parts: Vec<&str> = target_str.split('.').collect();
                            let target_knot = parts.last().unwrap_or(&"");
                            
                            if !target_knot.is_empty() && !target_knot.starts_with('^') && !target_knot.chars().next().unwrap().is_numeric() && *target_knot != "b" {
                                self.call_stack.push(target_knot.to_string());
                                return Ok(());
                            }
                        }
                    } else if let Some(branch_content) = obj.get("b") {
                        // branch content
                        if let Some(arr) = branch_content.as_array() {
                            self.execute_content(arr)?;
                        }
                    } else if obj.contains_key("*") {
                        // This is a choice
                        if let Some(choice_text_val) = obj.get("s") {
                            // Found choice text in 's' array
                            if let Some(choice_arr) = choice_text_val.as_array() {
                                for choice_item in choice_arr {
                                    if let Some(s) = choice_item.as_str() {
                                        if s.starts_with('^') {
                                            let choice_idx = self.current_choices.len();
                                            self.current_choices.push(Choice {
                                                index: choice_idx,
                                                text: s[1..].to_string(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                _ => {}
            }
            
            i += 1;
        }
        
        Ok(())
    }

    /// Evaluate an expression and push result to evaluation stack
    fn evaluate(&mut self, item: &Value) -> Result<(), String> {
        match item {
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    self.evaluation_stack.push(InkValue::Int(i as i32));
                } else if let Some(f) = n.as_f64() {
                    self.evaluation_stack.push(InkValue::Float(f as f32));
                }
            }
            Value::Bool(b) => {
                self.evaluation_stack.push(InkValue::Bool(*b));
            }
            Value::String(s) => {
                if s == "+" {
                    // Addition
                    if let (Some(b), Some(a)) = (self.evaluation_stack.pop(), self.evaluation_stack.pop()) {
                        match (a, b) {
                            (InkValue::Int(x), InkValue::Int(y)) => {
                                self.evaluation_stack.push(InkValue::Int(x + y));
                            }
                            _ => {}
                        }
                    }
                } else if s == "==" {
                    // Equality
                    if let (Some(b), Some(a)) = (self.evaluation_stack.pop(), self.evaluation_stack.pop()) {
                        let result = match (a, b) {
                            (InkValue::Bool(x), InkValue::Bool(y)) => x == y,
                            (InkValue::Int(x), InkValue::Int(y)) => x == y,
                            _ => false,
                        };
                        self.evaluation_stack.push(InkValue::Bool(result));
                    }
                } else if s == ">=" {
                    // Greater than or equal
                    if let (Some(b), Some(a)) = (self.evaluation_stack.pop(), self.evaluation_stack.pop()) {
                        let result = match (a, b) {
                            (InkValue::Int(x), InkValue::Int(y)) => x >= y,
                            _ => false,
                        };
                        self.evaluation_stack.push(InkValue::Bool(result));
                    }
                }
            }
            Value::Object(obj) => {
                if let Some(var_name) = obj.get("VAR?") {
                    // Variable read
                    if let Some(var_str) = var_name.as_str() {
                        let value = self.variables.get(var_str)
                            .cloned()
                            .unwrap_or(InkValue::Null);
                        self.evaluation_stack.push(value);
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}
