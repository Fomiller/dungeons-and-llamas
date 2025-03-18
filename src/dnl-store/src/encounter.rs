use dnl_types::tools::battle::BattleToolOutput;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterQuery {
    pub text: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterState {
    pub text: String,
    pub data: BattleToolOutput,
}
