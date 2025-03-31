use thiserror::Error;

use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::batch_write_item::BatchWriteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError;
use aws_sdk_dynamodb::operation::update_item::UpdateItemError;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Generic update error: {0}")]
    GenericUpdate(SdkError<UpdateItemError>),
    #[error("Generic query error: {0}")]
    GenericQuery(SdkError<QueryError>),
    #[error("Generic begins with query error: {0}")]
    GenericBeginsWithQuery(SdkError<QueryError>),
    #[error("Generic get error: {0}")]
    GenericGet(SdkError<GetItemError>),
    #[error("Generic put error: {0}")]
    GenericPut(SdkError<PutItemError>),
    #[error("Generic batch write root sort keys error: {0}")]
    GenericBatchWriteRootSortKeys(SdkError<BatchWriteItemError>),
    #[error("Generic batch update error: {0}")]
    GenericBatchUpdate(SdkError<TransactWriteItemsError>),

    #[error("Failed to create user")]
    CreateUser(String, String),
    #[error("Failed to create new game")]
    CreateNewGame(String, String),

    #[error("Failed to save message token")]
    SaveMessageToken(String, String),
    #[error("Failed to save game settings")]
    SaveGameSettings(String, String),
    #[error("Failed to save new game state")]
    SaveNewGameState(String, String),
    #[error("Failed to save active game id")]
    SaveActiveGameId(String, String),
    #[error("Failed to save encounter")]
    SaveEncounter(String, String),

    #[error("Failed to get encounters")]
    GetEncounters(String, String),
    #[error("Failed to get state")]
    GetState(String, String),
    #[error("Failed to get enemies")]
    GetEnemies(String, String),
    #[error("Failed to get llm model")]
    GetLLMModel(String, String),
    #[error("Failed to get active game id")]
    GetActiveGameId(String, String),

    #[error("Failed to update state")]
    UpdateState(String, String),

    #[error("Failed to find active game id for user ")]
    ActiveGameIdNotFound(String),
    #[error("Failed to find llm model for game settings ")]
    LLMModelNotFound(String),
    #[error("Failed to find encounters for user {0} game {1} ")]
    EncountersNotFound(String, String),
    #[error("Failed to find enemies for: {0}")]
    EnemiesNotFound(String),
    #[error("Failed to find settings")]
    SettingsNotFound,
    #[error("Failed to create game id: {0}")]
    CreateGameId(String),

    #[error("AWS Sdk error: {0}")]
    AWSSdk(String),
    #[error("Serde Dynamo error: {0}")]
    SerdeDynamo(String),
    #[error("An error occured: {0}")]
    Other(String),
}
