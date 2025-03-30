use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to update store {0}{1}")]
    GenericUpdate(String, String),
    #[error("Failed to query store")]
    GenericQuery(String, String),
    #[error("Failed to execute begins with query")]
    GenericBeginsWithQuery(String, String),
    #[error("Failed to exeute get store")]
    GenericGet(String, String),
    #[error("Failed to failed to put store")]
    GenericPut(String, String),
    #[error("Failed to batch write root sort keys")]
    GenericBatchWriteRootSortKeys(String, String),
    #[error("Failed to batch update")]
    GenericBatchUpdate(String, String),

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
    #[error("Failed to get settings")]
    GetSettings(String, String),
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
}
