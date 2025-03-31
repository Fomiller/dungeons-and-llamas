use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Unable to parse tool output to struct: {0}")]
    ParseOutput(String),
    #[error("No tool output available")]
    NoOutput,
}
