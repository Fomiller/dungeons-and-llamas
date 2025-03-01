use serde::*;
use dnl_types::tools::battle::BattleToolOutput;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterQuery {
    #[serde(rename(deserialize = "State"))]
    pub state: EncounterState
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterState {
    pub text: String,
    pub data: BattleToolOutput
}
