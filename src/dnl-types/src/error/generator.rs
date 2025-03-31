use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Failed to generate text")]
    GenerateTextError,

    #[error(transparent)]
    Store(#[from] super::store::StoreError),

    #[error(transparent)]
    Llm(#[from] super::llm::LLMError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeDynamo(#[from] serde_dynamo::Error),
}
