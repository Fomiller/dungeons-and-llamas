use serde::*;
use dnl_types::tools::battle::BattleToolOutput;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterQuery {
    pub text: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterState {
    pub text: String,
    pub data: BattleToolOutput
}
