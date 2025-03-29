use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Prompt {
    pub text: String,
    pub variables: HashMap<String, String>,
}

impl Prompt {
    pub fn format(&self) -> String {
        let mut result = self.text.to_string();

        for (key, value) in &self.variables {
            result = result.replace(key, value);
        }

        result
    }
}
