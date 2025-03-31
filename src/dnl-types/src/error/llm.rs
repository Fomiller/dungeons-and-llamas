use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("converse error")]
    Converse,
}
