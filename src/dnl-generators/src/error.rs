use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to generate text")]
    GenerateTextError,

    #[error(transparent)]
    Store(#[from] dnl_store::Error),

    #[error(transparent)]
    Llm(#[from] dnl_types::errors::LLMError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeDynamo(#[from] serde_dynamo::Error),
}
